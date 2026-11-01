// Ports define hexagonal architectural boundaries for Telar Terminal.
// Unused declarations will be consumed by adapters in R5 (CLIProxyAdapter and WebviewWindowAdapter).
#![allow(dead_code, unused_imports)]

pub mod proxy;
pub mod widget;
pub mod window;

pub use proxy::{ProxyConfig, ProxyConfigError, ProxyHandle, ProxyLifecyclePort, ProxyStatus};
pub use widget::{ProviderStatusToken, StatusColor, WidgetMetrics, WidgetProjectionPort};
pub use window::{ManagementWindowPort, WindowHandle, WindowStatus, WindowTarget};
