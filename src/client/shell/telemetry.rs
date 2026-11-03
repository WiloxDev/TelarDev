/// Telemetría tipada desacoplada del HUD / Status del dock derecho
/// Modela los datos puros sin PTYs, sin async y sin mutaciones durante render.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountQuota {
    pub name: String,
    pub percent: u8,
    pub is_leader: bool,
    pub role_symbol: &'static str,
    pub quota_desc: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelQuota {
    pub model_name: String,
    pub window_type: String,
    pub indicator: String,
    pub reset_in: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContextUsage {
    pub used_tokens: u64,
    pub limit_tokens: u64,
    pub percent: f32,
    pub cost: f32,
    pub status_label: String,
}

impl Default for ContextUsage {
    fn default() -> Self {
        Self {
            used_tokens: 179_200,
            limit_tokens: 1_000_000,
            percent: 17.9,
            cost: 0.0,
            status_label: "Optimo 17.9%".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitSummary {
    pub branch: String,
    pub modified_count: usize,
    pub untracked_count: usize,
    pub head_commit: Option<String>,
    pub upstream_commit: Option<String>,
    pub is_clean: bool,
    pub agent_files_changed: usize,
    pub agent_lines_added: usize,
}

impl Default for GitSummary {
    fn default() -> Self {
        Self {
            branch: "main".to_string(),
            modified_count: 0,
            untracked_count: 2,
            head_commit: Some("4645f7a".to_string()),
            upstream_commit: Some("9f64461".to_string()),
            is_clean: true,
            agent_files_changed: 2,
            agent_lines_added: 6,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngramTelemetry {
    pub project_name: String,
    pub port: u16,
    pub is_online: bool,
    pub observations_count: usize,
    pub cloud_status: String,
}

impl Default for EngramTelemetry {
    fn default() -> Self {
        Self {
            project_name: "telar-dev".to_string(),
            port: 7437,
            is_online: true,
            observations_count: 27,
            cloud_status: "solo local o no configurado".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolStats {
    pub total_calls: usize,
    pub reads: usize,
    pub writes: usize,
    pub memory_ops: usize,
}

impl Default for ToolStats {
    fn default() -> Self {
        Self {
            total_calls: 324,
            reads: 27,
            writes: 2,
            memory_ops: 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McpServerTelemetry {
    pub name: String,
    pub scope: &'static str,
    pub status: &'static str,
    pub is_online: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatusSnapshot {
    pub model_name: String,
    pub active_account: String,
    pub project_days: usize,
    pub context: ContextUsage,
    pub accounts: Vec<AccountQuota>,
    pub model_quotas: Vec<ModelQuota>,
    pub git: GitSummary,
    pub engram: EngramTelemetry,
    pub tools: ToolStats,
    pub mcp_servers: Vec<McpServerTelemetry>,
}

impl Default for StatusSnapshot {
    fn default() -> Self {
        Self {
            model_name: "gemini-3.8-flash-h".to_string(),
            active_account: "agy2_1".to_string(),
            project_days: 25,
            context: ContextUsage::default(),
            accounts: vec![
                AccountQuota {
                    name: "agy".into(),
                    percent: 100,
                    is_leader: true,
                    role_symbol: "•",
                    quota_desc: "100% Wk: 9 Wk:en 6d 2h".into(),
                },
                AccountQuota {
                    name: "agy_1".into(),
                    percent: 100,
                    is_leader: false,
                    role_symbol: "•",
                    quota_desc: "100% Wk:   Wk:en 1d 10h".into(),
                },
                AccountQuota {
                    name: "agy_2".into(),
                    percent: 100,
                    is_leader: false,
                    role_symbol: "•",
                    quota_desc: "100% Wk: 9 Wk:en 6d 3h".into(),
                },
                AccountQuota {
                    name: "agy2".into(),
                    percent: 100,
                    is_leader: false,
                    role_symbol: "•",
                    quota_desc: "100% Wk:   Wk:en 5d 12h".into(),
                },
                AccountQuota {
                    name: "agy2_1".into(),
                    percent: 97,
                    is_leader: true,
                    role_symbol: ">",
                    quota_desc: " 97% Wk: 9 Wk:en 5d 4h".into(),
                },
                AccountQuota {
                    name: "agy2_2".into(),
                    percent: 99,
                    is_leader: false,
                    role_symbol: ">",
                    quota_desc: " 99% Wk:   Wk:en 17h 28m".into(),
                },
                AccountQuota {
                    name: "agy3".into(),
                    percent: 100,
                    is_leader: false,
                    role_symbol: "•",
                    quota_desc: "100% Wk: 1 Wk:en 5d 12h".into(),
                },
                AccountQuota {
                    name: "agy3_1".into(),
                    percent: 99,
                    is_leader: false,
                    role_symbol: ">",
                    quota_desc: " 99% Wk: 7 Wk:en 1d 6h".into(),
                },
                AccountQuota {
                    name: "agy3_2".into(),
                    percent: 100,
                    is_leader: false,
                    role_symbol: "•",
                    quota_desc: "100% Wk: 9 Wk:en 2d 9h".into(),
                },
            ],
            model_quotas: vec![
                ModelQuota {
                    model_name: "Gemini Wk".into(),
                    window_type: "Wk".into(),
                    indicator: "↑17%".into(),
                    reset_in: "en 5d 4h".into(),
                },
                ModelQuota {
                    model_name: "Gemini 5h".into(),
                    window_type: "5h".into(),
                    indicator: "~0%".into(),
                    reset_in: "en 4h 52m".into(),
                },
                ModelQuota {
                    model_name: "Claude Wk".into(),
                    window_type: "Wk".into(),
                    indicator: "1 ~0%".into(),
                    reset_in: "en 6d 23h".into(),
                },
                ModelQuota {
                    model_name: "Claude 5h".into(),
                    window_type: "5h".into(),
                    indicator: "1 ~0%".into(),
                    reset_in: "en 4h 59m".into(),
                },
            ],
            git: GitSummary::default(),
            engram: EngramTelemetry::default(),
            tools: ToolStats::default(),
            mcp_servers: vec![
                McpServerTelemetry {
                    name: "context7".into(),
                    scope: "[remote]",
                    status: "en línea",
                    is_online: true,
                },
                McpServerTelemetry {
                    name: "engram".into(),
                    scope: "[local]",
                    status: "en línea",
                    is_online: true,
                },
                McpServerTelemetry {
                    name: "open-design".into(),
                    scope: "[local]",
                    status: "en línea",
                    is_online: true,
                },
            ],
        }
    }
}
