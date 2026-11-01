use std::fs;
use std::io;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::ports::proxy::{
    ProxyConfig, ProxyConfigError, ProxyHandle, ProxyLifecyclePort, ProxyStatus,
};

pub struct CLIProxyAdapter {
    binary_path: PathBuf,
    work_dir: PathBuf,
    config_path: PathBuf,
    active_child: std::sync::Mutex<Option<Child>>,
    is_healthy: Arc<AtomicBool>,
}

impl CLIProxyAdapter {
    pub fn new(binary_path: PathBuf, work_dir: PathBuf, config_path: PathBuf) -> Self {
        Self {
            binary_path,
            work_dir,
            config_path,
            active_child: std::sync::Mutex::new(None),
            is_healthy: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn default_paths() -> io::Result<Self> {
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "HOME directory not found"))?;

        let work_dir = home.join(".local/share/telar/proxy");
        let binary_path = work_dir.join("cli-proxy-api");
        let config_path = home.join(".config/telar/proxy/config.yaml");

        Ok(Self::new(binary_path, work_dir, config_path))
    }

    pub fn generate_config_yaml(config: &ProxyConfig) -> Result<String, ProxyConfigError> {
        config.validate()?;

        let secret_key_line = match &config.secret_key {
            Some(key) => format!("  secret-key: \"{key}\"\n"),
            None => String::new(),
        };

        Ok(format!(
            "config-version: 8\n\
             \n\
             server:\n\
             \x20\x20host: \"{host}\"\n\
             \x20\x20port: {port}\n\
             \n\
             management:\n\
             \x20\x20allow-remote: false\n\
             {secret_key_line}\
             \x20\x20disable-control-panel: false\n\
             \n\
             access:\n\
             \x20\x20api-keys:\n\
             \x20\x20\x20\x20- \"telar-local-client-key\"\n",
            host = config.host,
            port = config.port,
        ))
    }

    pub fn write_config_file(&self, config: &ProxyConfig) -> io::Result<()> {
        let yaml = Self::generate_config_yaml(config)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;

        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.config_path, yaml)
    }

    fn check_socket_alive(addr: SocketAddr) -> bool {
        std::net::TcpStream::connect_timeout(&addr, Duration::from_millis(150)).is_ok()
    }
}

impl ProxyLifecyclePort for CLIProxyAdapter {
    fn start(&self, config: &ProxyConfig) -> io::Result<ProxyHandle> {
        config
            .validate()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;

        if !self.binary_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("cli-proxy-api binary not found at {:?}", self.binary_path),
            ));
        }

        self.write_config_file(config)?;

        fs::create_dir_all(&self.work_dir)?;

        let mut cmd = Command::new(&self.binary_path);
        cmd.arg("-config")
            .arg(&self.config_path)
            .current_dir(&self.work_dir)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        #[cfg(target_os = "linux")]
        {
            use std::os::unix::process::CommandExt;
            unsafe {
                cmd.pre_exec(|| {
                    libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGTERM);
                    Ok(())
                });
            }
        }

        let child = cmd.spawn()?;
        let pid = child.id();

        let host_ip: IpAddr = if config.host == "localhost" {
            "127.0.0.1".parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?
        } else {
            config.host.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?
        };
        let endpoint = SocketAddr::new(host_ip, config.port);

        // Store active child handle
        {
            let mut guard = self.active_child.lock().map_err(|_| {
                io::Error::new(io::ErrorKind::Other, "active_child mutex poisoned")
            })?;
            *guard = Some(child);
        }

        // Wait up to 6 seconds for health
        let start_time = Instant::now();
        let timeout = Duration::from_secs(6);
        let mut ready = false;

        while start_time.elapsed() < timeout {
            {
                let mut guard = self.active_child.lock().map_err(|_| {
                    io::Error::new(io::ErrorKind::Other, "active_child mutex poisoned")
                })?;
                if let Some(child) = guard.as_mut() {
                    if let Ok(Some(exit_status)) = child.try_wait() {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!("proxy child process exited early with status: {exit_status}"),
                        ));
                    }
                }
            }

            if Self::check_socket_alive(endpoint) {
                ready = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(50));
        }

        if !ready {
            let _ = self.stop(&ProxyHandle { pid, endpoint });
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                format!("proxy failed to open loopback socket at {endpoint} within 6s"),
            ));
        }

        self.is_healthy.store(true, Ordering::SeqCst);
        Ok(ProxyHandle { pid, endpoint })
    }

    fn stop(&self, _handle: &ProxyHandle) -> io::Result<()> {
        let mut guard = self.active_child.lock().map_err(|_| {
            io::Error::new(io::ErrorKind::Other, "active_child mutex poisoned")
        })?;

        if let Some(mut child) = guard.take() {
            #[cfg(unix)]
            {
                use libc::{kill, pid_t, SIGKILL, SIGTERM};
                let pid = child.id() as pid_t;
                unsafe {
                    kill(pid, SIGTERM);
                }

                let start = Instant::now();
                let limit = Duration::from_millis(1500);
                let mut exited = false;

                while start.elapsed() < limit {
                    if let Ok(Some(_)) = child.try_wait() {
                        exited = true;
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(25));
                }

                if !exited {
                    unsafe {
                        kill(pid, SIGKILL);
                    }
                    let _ = child.wait();
                }
            }

            #[cfg(windows)]
            {
                let _ = child.kill();
                let _ = child.wait();
            }
        }

        self.is_healthy.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn status(&self, _handle: &ProxyHandle) -> ProxyStatus {
        let mut guard = match self.active_child.lock() {
            Ok(g) => g,
            Err(_) => {
                return ProxyStatus::Degraded {
                    reason: "mutex poisoned".to_string(),
                }
            }
        };

        if let Some(child) = guard.as_mut() {
            match child.try_wait() {
                Ok(Some(status)) => {
                    self.is_healthy.store(false, Ordering::SeqCst);
                    ProxyStatus::Degraded {
                        reason: format!("child exited with: {status}"),
                    }
                }
                Ok(None) => {
                    if self.is_healthy.load(Ordering::SeqCst) {
                        ProxyStatus::Healthy
                    } else {
                        ProxyStatus::Starting
                    }
                }
                Err(e) => ProxyStatus::Degraded {
                    reason: e.to_string(),
                },
            }
        } else {
            ProxyStatus::Stopped
        }
    }

    fn endpoint(&self, handle: &ProxyHandle) -> SocketAddr {
        handle.endpoint
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_config_yaml_strict_loopback() {
        let config = ProxyConfig::new("127.0.0.1", 8319);
        let yaml = CLIProxyAdapter::generate_config_yaml(&config).unwrap();
        assert!(yaml.contains("host: \"127.0.0.1\""));
        assert!(yaml.contains("port: 8319"));
        assert!(yaml.contains("allow-remote: false"));
        assert!(!yaml.contains("secret-key"));
    }

    #[test]
    fn test_generate_config_yaml_with_secret_key() {
        let mut config = ProxyConfig::new("127.0.0.1", 8319);
        config.secret_key = Some("test-secret-12345".to_string());
        let yaml = CLIProxyAdapter::generate_config_yaml(&config).unwrap();
        assert!(yaml.contains("secret-key: \"test-secret-12345\""));
    }

    #[test]
    fn test_generate_config_yaml_rejects_wildcard() {
        let config = ProxyConfig::new("0.0.0.0", 8319);
        assert!(CLIProxyAdapter::generate_config_yaml(&config).is_err());
    }

    #[test]
    fn test_adapter_default_paths() {
        let adapter = CLIProxyAdapter::default_paths();
        assert!(adapter.is_ok());
        let a = adapter.unwrap();
        assert!(a.config_path.ends_with(".config/telar/proxy/config.yaml"));
        assert!(a.work_dir.ends_with(".local/share/telar/proxy"));
    }
}
