// Proxy adapter manages the local CLIProxyAPI backend and CPAMC asset provisioning.
// Implements ProxyLifecyclePort under hexagonal boundary constraints.
#![allow(dead_code, unused_imports)]

pub mod adapter;
pub mod provision;

pub use adapter::CLIProxyAdapter;
pub use provision::{
    check_provisioning_status, ensure_all_provisioned, provision_cliproxyapi_binary,
    provision_cpamc_html, ProvisioningStatus, CLIPROXYAPI_VERSION, CPAMC_VERSION,
};

#[cfg(test)]
mod alpha_tests {
    use super::*;
    use crate::ports::proxy::{ProxyConfig, ProxyLifecyclePort, ProxyStatus};
    use crate::ports::window::WindowTarget;
    use portable_pty::CommandBuilder;
    use std::ffi::OsStr;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_alpha_e2e_lifecycle_and_env_injection() {
        let proof_bin = PathBuf::from("/var/tmp/telar-r3-proof/cli-proxy-api");
        let proof_html = PathBuf::from("/var/tmp/telar-r3-proof/management.html");

        if !proof_bin.is_file() || !proof_html.is_file() {
            // Skip live execution if binary was cleaned up; contracts remain tested by unit tests
            return;
        }

        let temp_dir = std::env::temp_dir().join(format!("telar-alpha-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        // Copy management.html so CLIProxyAPI serves it
        let html_dest = temp_dir.join("management.html");
        fs::copy(&proof_html, &html_dest).unwrap();

        let config_path = temp_dir.join("config.yaml");
        let adapter = CLIProxyAdapter::new(proof_bin, temp_dir.clone(), config_path);

        let port = 8339;
        let mut config = ProxyConfig::new("127.0.0.1", port);
        config.secret_key = Some("alpha-test-secret".to_string());

        // 1. Start proxy backend
        let handle = adapter.start(&config).expect("failed to start proxy in alpha test");
        assert_eq!(handle.endpoint.port(), port);
        assert_eq!(adapter.status(&handle), ProxyStatus::Healthy);

        // 2. Test PTY environment injection
        let mut cmd = CommandBuilder::new("shell");
        crate::integration::apply_proxy_env(&mut cmd, handle.endpoint);

        let expected_proxy = format!("http://127.0.0.1:{port}");
        assert_eq!(cmd.get_env("HTTP_PROXY"), Some(OsStr::new(&expected_proxy)));
        assert_eq!(cmd.get_env("HTTPS_PROXY"), Some(OsStr::new(&expected_proxy)));
        assert_eq!(
            cmd.get_env("OPENAI_BASE_URL"),
            Some(OsStr::new(&format!("{expected_proxy}/v1")))
        );

        // 3. Test CPAMC window target URL construction
        let oauth_url = WindowTarget::OAuth.build_url(&expected_proxy);
        assert_eq!(
            oauth_url,
            format!("{expected_proxy}/management.html#/oauth")
        );

        let auth_files_url = WindowTarget::AuthFiles.build_url(&expected_proxy);
        assert_eq!(
            auth_files_url,
            format!("{expected_proxy}/management.html#/auth-files")
        );

        // 4. Test HTTP connection to management SPA
        let resp = std::net::TcpStream::connect(handle.endpoint);
        assert!(resp.is_ok(), "could not connect to proxy loopback port");

        // 5. Stop proxy cleanly
        assert!(adapter.stop(&handle).is_ok());
        assert_eq!(adapter.status(&handle), ProxyStatus::Stopped);

        // Clean test dir
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
