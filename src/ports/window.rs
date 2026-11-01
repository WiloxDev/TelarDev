use std::fmt;
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowTarget {
    OAuth,
    AuthFiles,
    Overview,
    Custom(String),
}

impl WindowTarget {
    pub fn route_fragment(&self) -> &str {
        match self {
            Self::OAuth => "#/oauth",
            Self::AuthFiles => "#/auth-files",
            Self::Overview => "#/overview",
            Self::Custom(route) => route.as_str(),
        }
    }

    pub fn build_url(&self, base_url: &str) -> String {
        let trimmed_base = base_url.trim_end_matches('/');
        let fragment = self.route_fragment();
        if trimmed_base.ends_with("management.html") {
            format!("{trimmed_base}{fragment}")
        } else {
            format!("{trimmed_base}/management.html{fragment}")
        }
    }
}

impl fmt::Display for WindowTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.route_fragment())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowStatus {
    Closed,
    Opening,
    Open { window_id: u64 },
    Failed { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowHandle {
    pub window_id: u64,
    pub process_id: Option<u32>,
}

pub trait ManagementWindowPort: Send + Sync {
    fn open(&self, target: &WindowTarget, base_url: &str) -> io::Result<WindowHandle>;
    fn close(&self, handle: &WindowHandle) -> io::Result<()>;
    fn status(&self, handle: &WindowHandle) -> WindowStatus;
    fn is_open(&self, handle: &WindowHandle) -> bool {
        matches!(self.status(handle), WindowStatus::Open { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn test_window_target_urls() {
        let base = "http://127.0.0.1:8317";
        assert_eq!(
            WindowTarget::OAuth.build_url(base),
            "http://127.0.0.1:8317/management.html#/oauth"
        );
        assert_eq!(
            WindowTarget::AuthFiles.build_url(base),
            "http://127.0.0.1:8317/management.html#/auth-files"
        );
        assert_eq!(
            WindowTarget::Overview.build_url(base),
            "http://127.0.0.1:8317/management.html#/overview"
        );

        let explicit_html_base = "http://127.0.0.1:8317/management.html";
        assert_eq!(
            WindowTarget::OAuth.build_url(explicit_html_base),
            "http://127.0.0.1:8317/management.html#/oauth"
        );
    }

    struct MockWindowPort {
        open: AtomicBool,
    }

    impl ManagementWindowPort for MockWindowPort {
        fn open(&self, _target: &WindowTarget, _base_url: &str) -> io::Result<WindowHandle> {
            self.open.store(true, Ordering::SeqCst);
            Ok(WindowHandle {
                window_id: 42,
                process_id: Some(999),
            })
        }

        fn close(&self, _handle: &WindowHandle) -> io::Result<()> {
            self.open.store(false, Ordering::SeqCst);
            Ok(())
        }

        fn status(&self, handle: &WindowHandle) -> WindowStatus {
            if self.open.load(Ordering::SeqCst) {
                WindowStatus::Open {
                    window_id: handle.window_id,
                }
            } else {
                WindowStatus::Closed
            }
        }
    }

    #[test]
    fn test_mock_window_lifecycle() {
        let port = MockWindowPort {
            open: AtomicBool::new(false),
        };
        let handle = port.open(&WindowTarget::OAuth, "http://127.0.0.1:8317").unwrap();
        assert_eq!(handle.window_id, 42);
        assert!(port.is_open(&handle));
        assert_eq!(
            port.status(&handle),
            WindowStatus::Open { window_id: 42 }
        );

        assert!(port.close(&handle).is_ok());
        assert!(!port.is_open(&handle));
        assert_eq!(port.status(&handle), WindowStatus::Closed);
    }
}
