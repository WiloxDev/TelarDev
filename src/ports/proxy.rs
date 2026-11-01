use std::fmt;
use std::io;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProxyStatus {
    Stopped,
    Starting,
    Healthy,
    Degraded { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyHandle {
    pub pid: u32,
    pub endpoint: SocketAddr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProxyConfigError {
    InvalidHost(String),
    NonLoopbackHost(String),
    InvalidPort(u16),
}

impl fmt::Display for ProxyConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHost(host) => write!(f, "invalid proxy host: {host}"),
            Self::NonLoopbackHost(host) => {
                write!(f, "security violation: proxy host must be loopback, got: {host}")
            }
            Self::InvalidPort(port) => write!(f, "invalid proxy port: {port}"),
        }
    }
}

impl std::error::Error for ProxyConfigError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyConfig {
    pub host: String,
    pub port: u16,
    pub secret_key: Option<String>,
    pub auth_dir: Option<PathBuf>,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8317,
            secret_key: None,
            auth_dir: None,
        }
    }
}

impl ProxyConfig {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
            secret_key: None,
            auth_dir: None,
        }
    }

    pub fn validate(&self) -> Result<(), ProxyConfigError> {
        let trimmed = self.host.trim();
        if trimmed.is_empty() {
            return Err(ProxyConfigError::InvalidHost("empty host string".to_string()));
        }

        if trimmed == "localhost" {
            return Ok(());
        }

        match trimmed.parse::<IpAddr>() {
            Ok(ip) => {
                if ip.is_loopback() {
                    Ok(())
                } else {
                    Err(ProxyConfigError::NonLoopbackHost(trimmed.to_string()))
                }
            }
            Err(_) => Err(ProxyConfigError::InvalidHost(trimmed.to_string())),
        }
    }
}

pub trait ProxyLifecyclePort: Send + Sync {
    fn start(&self, config: &ProxyConfig) -> io::Result<ProxyHandle>;
    fn stop(&self, handle: &ProxyHandle) -> io::Result<()>;
    fn status(&self, handle: &ProxyHandle) -> ProxyStatus;
    fn endpoint(&self, handle: &ProxyHandle) -> SocketAddr;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn test_default_config_is_loopback() {
        let config = ProxyConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8317);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_localhost_string_is_accepted() {
        let config = ProxyConfig::new("localhost", 8319);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_ipv6_loopback_is_accepted() {
        let config = ProxyConfig::new("::1", 8319);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_wildcard_host_is_strictly_rejected() {
        for host in ["0.0.0.0", "::", ""] {
            let config = ProxyConfig::new(host, 8319);
            assert!(config.validate().is_err());
        }
    }

    #[test]
    fn test_public_or_lan_ip_is_strictly_rejected() {
        for host in ["192.168.1.1", "10.0.0.1", "8.8.8.8"] {
            let config = ProxyConfig::new(host, 8319);
            match config.validate() {
                Err(ProxyConfigError::NonLoopbackHost(h)) => assert_eq!(h, host),
                other => panic!("expected NonLoopbackHost for {host}, got {other:?}"),
            }
        }
    }

    struct MockProxyPort {
        is_running: AtomicBool,
    }

    impl ProxyLifecyclePort for MockProxyPort {
        fn start(&self, config: &ProxyConfig) -> io::Result<ProxyHandle> {
            config
                .validate()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;
            self.is_running.store(true, Ordering::SeqCst);
            let ip = if config.host == "localhost" {
                "127.0.0.1".parse().unwrap()
            } else {
                config.host.parse().unwrap()
            };
            Ok(ProxyHandle {
                pid: 1234,
                endpoint: SocketAddr::new(ip, config.port),
            })
        }

        fn stop(&self, _handle: &ProxyHandle) -> io::Result<()> {
            self.is_running.store(false, Ordering::SeqCst);
            Ok(())
        }

        fn status(&self, _handle: &ProxyHandle) -> ProxyStatus {
            if self.is_running.load(Ordering::SeqCst) {
                ProxyStatus::Healthy
            } else {
                ProxyStatus::Stopped
            }
        }

        fn endpoint(&self, handle: &ProxyHandle) -> SocketAddr {
            handle.endpoint
        }
    }

    #[test]
    fn test_mock_proxy_lifecycle() {
        let port = MockProxyPort {
            is_running: AtomicBool::new(false),
        };
        let config = ProxyConfig::new("127.0.0.1", 8319);
        let handle = port.start(&config).unwrap();
        assert_eq!(handle.pid, 1234);
        assert_eq!(port.status(&handle), ProxyStatus::Healthy);
        assert_eq!(port.endpoint(&handle), "127.0.0.1:8319".parse().unwrap());

        assert!(port.stop(&handle).is_ok());
        assert_eq!(port.status(&handle), ProxyStatus::Stopped);
    }
}
