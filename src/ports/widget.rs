use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusColor {
    Green,
    Yellow,
    Red,
    Muted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderStatusToken {
    pub label: String,
    pub color_hint: StatusColor,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WidgetMetrics {
    pub proxy_active: bool,
    pub active_provider: Option<String>,
    pub active_model: Option<String>,
    pub tokens_used: Option<u64>,
    pub quota_remaining_percent: Option<f32>,
}

impl Default for WidgetMetrics {
    fn default() -> Self {
        Self {
            proxy_active: false,
            active_provider: None,
            active_model: None,
            tokens_used: None,
            quota_remaining_percent: None,
        }
    }
}

pub trait WidgetProjectionPort: Send + Sync {
    fn project_metrics(&self) -> WidgetMetrics;

    fn status_token(&self) -> ProviderStatusToken {
        let metrics = self.project_metrics();
        if !metrics.proxy_active {
            return ProviderStatusToken {
                label: "proxy:off".to_string(),
                color_hint: StatusColor::Muted,
            };
        }

        match (metrics.active_provider, metrics.quota_remaining_percent) {
            (Some(prov), Some(pct)) if pct < 10.0 => ProviderStatusToken {
                label: format!("{prov}:{pct:.0}%"),
                color_hint: StatusColor::Red,
            },
            (Some(prov), Some(pct)) if pct < 30.0 => ProviderStatusToken {
                label: format!("{prov}:{pct:.0}%"),
                color_hint: StatusColor::Yellow,
            },
            (Some(prov), _) => ProviderStatusToken {
                label: prov,
                color_hint: StatusColor::Green,
            },
            (None, _) => ProviderStatusToken {
                label: "proxy:ready".to_string(),
                color_hint: StatusColor::Green,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WidgetError {
    InvalidJson(String),
}

impl std::fmt::Display for WidgetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidJson(msg) => write!(f, "invalid widget payload: {msg}"),
        }
    }
}

impl std::error::Error for WidgetError {}

/// Example community widget demonstrating how third-party plugins
/// project provider quota and telemetry into the TUI status bar
/// via the hexagonal WidgetProjectionPort without owning process lifecycles.
pub struct CommunityProviderQuotaWidget {
    metrics: std::sync::RwLock<WidgetMetrics>,
}

impl Default for CommunityProviderQuotaWidget {
    fn default() -> Self {
        Self {
            metrics: std::sync::RwLock::new(WidgetMetrics::default()),
        }
    }
}

impl CommunityProviderQuotaWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_metrics(&self, metrics: WidgetMetrics) {
        if let Ok(mut lock) = self.metrics.write() {
            *lock = metrics;
        }
    }

    pub fn update_from_json(&self, raw_json: &str) -> Result<(), WidgetError> {
        let value: serde_json::Value = serde_json::from_str(raw_json)
            .map_err(|e| WidgetError::InvalidJson(e.to_string()))?;

        let obj = value.as_object().ok_or_else(|| {
            WidgetError::InvalidJson("payload root must be an object".to_string())
        })?;

        let proxy_active = obj
            .get("proxy_active")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let active_provider = obj
            .get("provider")
            .or_else(|| obj.get("active_provider"))
            .and_then(|v| v.as_str())
            .map(String::from);

        let active_model = obj
            .get("model")
            .or_else(|| obj.get("active_model"))
            .and_then(|v| v.as_str())
            .map(String::from);

        let tokens_used = obj.get("tokens_used").and_then(|v| v.as_u64());

        let quota_remaining_percent = obj
            .get("quota_remaining_percent")
            .and_then(|v| v.as_f64())
            .map(|f| f as f32);

        self.set_metrics(WidgetMetrics {
            proxy_active,
            active_provider,
            active_model,
            tokens_used,
            quota_remaining_percent,
        });

        Ok(())
    }
}

impl WidgetProjectionPort for CommunityProviderQuotaWidget {
    fn project_metrics(&self) -> WidgetMetrics {
        self.metrics.read().map(|g| g.clone()).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockWidgetSource {
        metrics: WidgetMetrics,
    }

    impl WidgetProjectionPort for MockWidgetSource {
        fn project_metrics(&self) -> WidgetMetrics {
            self.metrics.clone()
        }
    }

    #[test]
    fn test_widget_status_token_formatting() {
        let mut source = MockWidgetSource {
            metrics: WidgetMetrics::default(),
        };

        // When proxy is off:
        assert_eq!(
            source.status_token(),
            ProviderStatusToken {
                label: "proxy:off".to_string(),
                color_hint: StatusColor::Muted,
            }
        );

        // When proxy is on, no provider selected:
        source.metrics.proxy_active = true;
        assert_eq!(
            source.status_token(),
            ProviderStatusToken {
                label: "proxy:ready".to_string(),
                color_hint: StatusColor::Green,
            }
        );

        // When provider active with high quota:
        source.metrics.active_provider = Some("openai".to_string());
        source.metrics.quota_remaining_percent = Some(85.0);
        assert_eq!(
            source.status_token(),
            ProviderStatusToken {
                label: "openai".to_string(),
                color_hint: StatusColor::Green,
            }
        );

        // When provider quota warning (<30%):
        source.metrics.quota_remaining_percent = Some(25.0);
        assert_eq!(
            source.status_token(),
            ProviderStatusToken {
                label: "openai:25%".to_string(),
                color_hint: StatusColor::Yellow,
            }
        );

        // When provider quota critical (<10%):
        source.metrics.quota_remaining_percent = Some(5.0);
        assert_eq!(
            source.status_token(),
            ProviderStatusToken {
                label: "openai:5%".to_string(),
                color_hint: StatusColor::Red,
            }
        );
    }

    #[test]
    fn test_community_widget_json_update() {
        let widget = CommunityProviderQuotaWidget::new();

        // 1. Valid payload with provider and quota
        let json = r#"{"provider": "anthropic", "quota_remaining_percent": 18.0, "model": "claude-3-7-sonnet"}"#;
        assert!(widget.update_from_json(json).is_ok());

        let token = widget.status_token();
        assert_eq!(token.label, "anthropic:18%");
        assert_eq!(token.color_hint, StatusColor::Yellow);

        let metrics = widget.project_metrics();
        assert!(metrics.proxy_active);
        assert_eq!(metrics.active_provider, Some("anthropic".to_string()));
        assert_eq!(metrics.active_model, Some("claude-3-7-sonnet".to_string()));

        // 2. Reject non-object JSON safely
        let bad_json = r#""plain string""#;
        assert!(widget.update_from_json(bad_json).is_err());

        // 3. Reject malformed JSON safely
        let malformed = r#"{"provider": broken"#;
        assert!(widget.update_from_json(malformed).is_err());
    }

    #[test]
    fn test_community_widget_thread_safety() {
        use std::sync::Arc;
        let widget = Arc::new(CommunityProviderQuotaWidget::new());

        let w2 = Arc::clone(&widget);
        let handle = std::thread::spawn(move || {
            let json = r#"{"provider": "gemini", "quota_remaining_percent": 99.0}"#;
            w2.update_from_json(json).unwrap();
        });

        handle.join().unwrap();
        assert_eq!(
            widget.status_token(),
            ProviderStatusToken {
                label: "gemini".to_string(),
                color_hint: StatusColor::Green,
            }
        );
    }
}
