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
            used_tokens: 0,
            limit_tokens: 0,
            percent: 0.0,
            cost: 0.0,
            status_label: String::new(),
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
            branch: String::new(),
            modified_count: 0,
            untracked_count: 0,
            head_commit: None,
            upstream_commit: None,
            is_clean: true,
            agent_files_changed: 0,
            agent_lines_added: 0,
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
            project_name: String::new(),
            port: 0,
            is_online: false,
            observations_count: 0,
            cloud_status: String::new(),
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
            total_calls: 0,
            reads: 0,
            writes: 0,
            memory_ops: 0,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionTodoItem {
    pub title: String,
    pub is_done: bool,
    pub is_in_progress: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SessionLiveStats {
    pub spent_cost: f32,
    pub turns: usize,
    pub elapsed_str: String,
    pub tokens_in: u64,
    pub tokens_out: u64,
    pub cache_hit_percent: u8,
    pub cache_read_tokens: u64,
    pub tool_calls: usize,
    pub tool_failures: usize,
    pub subagent_runs: usize,
    pub subagent_tokens: u64,
    pub subagent_cost: f32,
    pub model_name: String,
    pub model_turns: usize,
    pub model_tokens: u64,
    pub model_cost: f32,
}

impl Default for SessionLiveStats {
    fn default() -> Self {
        Self {
            spent_cost: 0.0,
            turns: 0,
            elapsed_str: String::new(),
            tokens_in: 0,
            tokens_out: 0,
            cache_hit_percent: 0,
            cache_read_tokens: 0,
            tool_calls: 0,
            tool_failures: 0,
            subagent_runs: 0,
            subagent_tokens: 0,
            subagent_cost: 0.0,
            model_name: String::new(),
            model_turns: 0,
            model_tokens: 0,
            model_cost: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatusSnapshot {
    pub todos: Vec<SessionTodoItem>,
    pub live_stats: SessionLiveStats,
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
            model_name: String::new(),
            active_account: String::new(),
            project_days: 0,
            context: ContextUsage::default(),
            accounts: Vec::new(),
            model_quotas: Vec::new(),
            git: GitSummary::default(),
            engram: EngramTelemetry::default(),
            tools: ToolStats::default(),
            todos: Vec::new(),
            live_stats: SessionLiveStats::default(),
            mcp_servers: Vec::new(),
        }
    }
}

/// Resuelve la ruta canónica al directorio `.git` (soportando worktrees y repos estándar).
pub fn resolve_git_dir(root: &std::path::Path) -> Option<std::path::PathBuf> {
    let git_path = root.join(".git");
    if git_path.is_file() {
        let content = std::fs::read_to_string(&git_path).ok()?;
        let target = content.trim().strip_prefix("gitdir:")?.trim();
        let target_path = if std::path::Path::new(target).is_absolute() {
            std::path::PathBuf::from(target)
        } else {
            root.join(target)
        };
        Some(target_path)
    } else if git_path.is_dir() {
        Some(git_path)
    } else {
        None
    }
}

/// Reads the 7-character short commit hash of HEAD directly from `.git` without spawning subprocesses.
pub fn read_git_head_short(root: &std::path::Path) -> Option<String> {
    let git_dir = resolve_git_dir(root)?;
    let head = std::fs::read_to_string(git_dir.join("HEAD")).ok()?;

    let trimmed = head.trim();
    if let Some(branch_ref) = trimmed.strip_prefix("ref: ") {
        let ref_path = git_dir.join(branch_ref);
        if let Ok(ref_content) = std::fs::read_to_string(&ref_path) {
            let sha = ref_content.trim();
            if sha.len() >= 7 {
                return Some(sha[..7].to_string());
            }
        }
        // Fallback: packed-refs
        if let Ok(packed) = std::fs::read_to_string(git_dir.join("packed-refs")) {
            for line in packed.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 2 && parts[1] == branch_ref && parts[0].len() >= 7 {
                    return Some(parts[0][..7].to_string());
                }
            }
        }
    } else if trimmed.len() >= 7 && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        return Some(trimmed[..7].to_string());
    }
    None
}

/// Extrae la rama activa y el short SHA de HEAD.
pub fn read_git_head_info(root: &std::path::Path) -> (Option<String>, Option<String>) {
    let Some(git_dir) = resolve_git_dir(root) else {
        return (None, None);
    };

    let head_path = git_dir.join("HEAD");
    let Ok(head_content) = std::fs::read_to_string(head_path) else {
        return (None, None);
    };

    let trimmed = head_content.trim();
    if let Some(branch_ref) = trimmed.strip_prefix("ref: ") {
        let branch_name = branch_ref.strip_prefix("refs/heads/").unwrap_or(branch_ref);
        let ref_path = git_dir.join(branch_ref);
        if let Ok(ref_content) = std::fs::read_to_string(&ref_path) {
            let sha = ref_content.trim();
            if sha.len() >= 7 {
                return (Some(branch_name.to_string()), Some(sha[..7].to_string()));
            }
        }
        // Fallback: packed-refs
        if let Ok(packed) = std::fs::read_to_string(git_dir.join("packed-refs")) {
            for line in packed.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 2 && parts[1] == branch_ref && parts[0].len() >= 7 {
                    return (Some(branch_name.to_string()), Some(parts[0][..7].to_string()));
                }
            }
        }
        (Some(branch_name.to_string()), None)
    } else if trimmed.len() >= 7 && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        (None, Some(trimmed[..7].to_string()))
    } else {
        (None, None)
    }
}

/// Detecta si hay una operación en curso: Rebase, Merge, Cherry-Pick.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InProgressGitOp {
    Rebase {
        step: usize,
        total: usize,
        onto_commit: String,
        has_conflict: bool,
    },
    Merge {
        merge_msg: String,
    },
    CherryPick {
        commit_hash: String,
    },
}

pub fn detect_in_progress_git_op(root: &std::path::Path) -> Option<InProgressGitOp> {
    let git_dir = resolve_git_dir(root)?;

    // 1. Rebase en progreso
    let rebase_merge = git_dir.join("rebase-merge");
    let rebase_apply = git_dir.join("rebase-apply");
    let rebase_dir = if rebase_merge.is_dir() {
        Some(rebase_merge)
    } else if rebase_apply.is_dir() {
        Some(rebase_apply)
    } else {
        None
    };

    if let Some(r_dir) = rebase_dir {
        let step = std::fs::read_to_string(r_dir.join("msgnum"))
            .ok()
            .and_then(|s| s.trim().parse::<usize>().ok())
            .unwrap_or(1);
        let total = std::fs::read_to_string(r_dir.join("end"))
            .ok()
            .and_then(|s| s.trim().parse::<usize>().ok())
            .unwrap_or(1);
        let onto = std::fs::read_to_string(r_dir.join("onto"))
            .ok()
            .map(|s| s.trim().chars().take(7).collect::<String>())
            .unwrap_or_else(|| "main".to_string());
        let has_conflict = r_dir.join("stopped-sha").is_file();
        return Some(InProgressGitOp::Rebase {
            step,
            total,
            onto_commit: onto,
            has_conflict,
        });
    }

    // 2. Merge en progreso
    if git_dir.join("MERGE_HEAD").is_file() {
        let msg = std::fs::read_to_string(git_dir.join("MERGE_MSG"))
            .unwrap_or_else(|_| "Merge en progreso".to_string())
            .lines()
            .next()
            .unwrap_or("Merge")
            .to_string();
        return Some(InProgressGitOp::Merge { merge_msg: msg });
    }

    // 3. Cherry-pick en progreso
    if git_dir.join("CHERRY_PICK_HEAD").is_file() {
        let sha = std::fs::read_to_string(git_dir.join("CHERRY_PICK_HEAD"))
            .unwrap_or_default()
            .trim()
            .chars()
            .take(7)
            .collect();
        return Some(InProgressGitOp::CherryPick { commit_hash: sha });
    }

    None
}

/// Extrae información de Stashes locales.
pub fn read_git_stash_info(root: &std::path::Path) -> Option<(usize, String)> {
    let git_dir = resolve_git_dir(root)?;
    let stash_log = git_dir.join("logs").join("refs").join("stash");
    if let Ok(content) = std::fs::read_to_string(stash_log) {
        let lines: Vec<&str> = content.lines().filter(|l| !l.is_empty()).collect();
        if !lines.is_empty() {
            let count = lines.len();
            let last_line = lines.last().unwrap_or(&"");
            let hint = if let Some((_, msg)) = last_line.split_once(": ") {
                msg.trim().to_string()
            } else {
                format!("stash@{{{}}}", count.saturating_sub(1))
            };
            return Some((count, hint));
        }
    }
    None
}

/// Extrae el tag más reciente o asociado.
pub fn read_git_latest_tag(root: &std::path::Path) -> Option<String> {
    let git_dir = resolve_git_dir(root)?;
    let tags_dir = git_dir.join("refs").join("tags");
    if let Ok(entries) = std::fs::read_dir(tags_dir) {
        let mut tags: Vec<String> = entries
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        tags.sort();
        if let Some(last_tag) = tags.pop() {
            return Some(last_tag);
        }
    }
    None
}

/// Extrae únicamente el nombre base del archivo para que quepa en columnas angostas (ej. "dock.rs").
pub fn clean_file_basename(path_str: &str) -> String {
    let path = std::path::Path::new(path_str.trim());
    path.file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| path_str.to_string())
}

/// Detecta si una ruta relativa o nombre de archivo corresponde a un secreto,
/// credencial o clave privada cuyo contenido no debe exponerse en texto plano.
pub fn is_sensitive_file(path: &str) -> bool {
    let norm = path.replace('\\', "/");
    let name = norm.rsplit('/').next().unwrap_or(&norm);
    let lower_name = name.to_lowercase();
    let lower_path = norm.to_lowercase();

    // No considerar plantillas o ejemplos públicos como secretos
    if lower_name.ends_with(".example")
        || lower_name.ends_with(".sample")
        || lower_name.ends_with(".template")
        || lower_name.contains("example")
    {
        return false;
    }

    // 1. Directorios críticos de credenciales
    if lower_path.contains("/.ssh/")
        || lower_path.starts_with(".ssh/")
        || lower_path.contains("/.aws/")
        || lower_path.starts_with(".aws/")
        || lower_path.contains("/.kube/")
        || lower_path.starts_with(".kube/")
        || lower_path.contains("/.gnupg/")
        || lower_path.starts_with(".gnupg/")
        || lower_path.contains("/.vault/")
        || lower_path.starts_with(".vault/")
        || lower_path.contains("/.docker/")
        || lower_path.starts_with(".docker/")
    {
        return true;
    }

    // 2. Archivos .env
    if lower_name.starts_with(".env") {
        return true;
    }

    // 3. Claves privadas y certificados criptográficos
    let is_key_or_cert = lower_name.ends_with(".pem")
        || lower_name.ends_with(".key")
        || lower_name.ends_with(".p12")
        || lower_name.ends_with(".pfx")
        || lower_name.ends_with(".keystore")
        || lower_name.ends_with(".jks")
        || lower_name.starts_with("id_rsa")
        || lower_name.starts_with("id_ed25519")
        || lower_name.starts_with("id_ecdsa")
        || lower_name.starts_with("id_dsa");

    if is_key_or_cert {
        return true;
    }

    // 4. Archivos de autenticación y tokens en claro
    if lower_name == ".netrc"
        || lower_name == ".npmrc"
        || lower_name == "credentials.json"
        || lower_name == "client_secret.json"
        || lower_name == "service_account.json"
    {
        return true;
    }

    // 5. Nombres que denotan secretos/credenciales sin ser código fuente normal
    let is_source_code = lower_name.ends_with(".rs")
        || lower_name.ends_with(".go")
        || lower_name.ends_with(".ts")
        || lower_name.ends_with(".js")
        || lower_name.ends_with(".py")
        || lower_name.ends_with(".c")
        || lower_name.ends_with(".cpp")
        || lower_name.ends_with(".h")
        || lower_name.ends_with(".java")
        || lower_name.ends_with(".html")
        || lower_name.ends_with(".css");

    if !is_source_code {
        if lower_name.contains("secret")
            || lower_name.contains("credential")
            || lower_name.contains("token")
            || lower_name.contains("passw")
            || lower_name.contains("private_key")
        {
            return true;
        }
    }

    false
}

/// Enmascara valores de credenciales, tokens conocidos y claves privadas
/// en líneas de texto para prevenir fugas accidentales en la interfaz.
pub fn sanitize_secret_line(line: &str) -> String {
    if line.contains("BEGIN ") && line.contains("PRIVATE KEY") {
        return "[-----REDACTED PRIVATE KEY-----]".to_string();
    }

    let mut result = line.to_string();

    if let Some(pos) = result.find("Bearer ") {
        let token_start = pos + 7;
        if token_start < result.len() {
            let token_end = result[token_start..]
                .find(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == ',')
                .map(|p| token_start + p)
                .unwrap_or(result.len());
            if token_end.saturating_sub(token_start) >= 8 {
                result.replace_range(token_start..token_end, "********");
            }
        }
    }

    let known_prefixes = ["ghp_", "gho_", "ghs_", "github_pat_", "sk-ant-", "sk-", "AKIA"];
    for prefix in known_prefixes {
        while let Some(pos) = result.find(prefix) {
            let val_start = pos + prefix.len();
            let val_end = result[val_start..]
                .find(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-')
                .map(|p| val_start + p)
                .unwrap_or(result.len());
            let token_len = val_end.saturating_sub(pos);
            if token_len >= 12 {
                result.replace_range(val_start..val_end, "***");
            } else {
                break;
            }
        }
    }

    let secret_keys = [
        "api_key", "apikey", "api-key",
        "secret", "client_secret",
        "password", "passwd",
        "access_token", "auth_token",
    ];

    let lower = result.to_lowercase();
    for key in secret_keys {
        if let Some(key_pos) = lower.find(key) {
            let after_key = &result[key_pos + key.len()..];
            if let Some(assign_offset) = after_key.find(['=', ':']) {
                let assign_pos = key_pos + key.len() + assign_offset + 1;
                let rest = &result[assign_pos..];
                let trimmed_offset = rest.len() - rest.trim_start().len();
                let val_start = assign_pos + trimmed_offset;
                if val_start < result.len() {
                    let quote_char = result[val_start..].chars().next().filter(|&c| c == '"' || c == '\'');
                    let actual_start = if quote_char.is_some() { val_start + 1 } else { val_start };
                    let actual_end = if let Some(q) = quote_char {
                        result[actual_start..].find(q).map(|p| actual_start + p).unwrap_or(result.len())
                    } else {
                        result[actual_start..]
                            .find(|c: char| c.is_whitespace() || c == ',' || c == ';')
                            .map(|p| actual_start + p)
                            .unwrap_or(result.len())
                    };

                    if actual_end.saturating_sub(actual_start) >= 4 {
                        result.replace_range(actual_start..actual_end, "********");
                    }
                }
            }
        }
    }

    result
}

/// Extrae los archivos modificados en caliente en el working tree sin comandos lentos de shell.
pub fn read_git_working_tree_files(root: &std::path::Path) -> Vec<String> {
    let output = std::process::Command::new("git")
        .current_dir(root)
        .args(["status", "--porcelain", "--untracked-files=no"])
        .output()
        .ok();

    let mut files = Vec::new();
    if let Some(out) = output {
        if out.status.success() {
            for line in String::from_utf8_lossy(&out.stdout).lines() {
                let trimmed = line.trim();
                if trimmed.len() > 3 {
                    let file_path = trimmed[3..].trim();
                    let basename = clean_file_basename(file_path);
                    let display_name = if is_sensitive_file(file_path) {
                        format!("🔒 {basename}")
                    } else {
                        basename
                    };
                    files.push(display_name);
                }
            }
        }
    }
    files.dedup();
    files
}

/// Extrae los últimos commits de la rama activa: (short_sha, message).
pub fn read_git_recent_commits(root: &std::path::Path, limit: usize) -> Vec<(String, String)> {
    let output = std::process::Command::new("git")
        .current_dir(root)
        .args(["log", &format!("-n{}", limit.max(1)), "--format=%h|%s"])
        .output()
        .ok();

    let mut commits = Vec::new();
    if let Some(out) = output {
        if out.status.success() {
            for line in String::from_utf8_lossy(&out.stdout).lines() {
                if let Some((sha, msg)) = line.split_once('|') {
                    commits.push((sha.trim().to_string(), msg.trim().to_string()));
                }
            }
        }
    }
    commits
}

/// Extrae los nombres de los archivos que formaron parte del último commit.
pub fn read_git_last_commit_files(root: &std::path::Path) -> Vec<String> {
    let output = std::process::Command::new("git")
        .current_dir(root)
        .args(["diff", "--name-only", "HEAD~1", "HEAD"])
        .output()
        .ok();

    let mut files = Vec::new();
    if let Some(out) = output {
        if out.status.success() {
            for line in String::from_utf8_lossy(&out.stdout).lines() {
                let clean = line.trim();
                if !clean.is_empty() {
                    let basename = clean_file_basename(clean);
                    let display_name = if is_sensitive_file(clean) {
                        format!("🔒 {basename}")
                    } else {
                        basename
                    };
                    files.push(display_name);
                }
            }
        }
    }
    files.dedup();
    files
}

/// Detecta dinámicamente si la rama base troncal es main, master o develop.
pub fn detect_default_base_branch(root: &std::path::Path) -> String {
    let Some(git_dir) = resolve_git_dir(root) else {
        return "main".to_string();
    };
    let heads = git_dir.join("refs").join("heads");
    if heads.join("main").exists() {
        "main".to_string()
    } else if heads.join("master").exists() {
        "master".to_string()
    } else if heads.join("develop").exists() {
        "develop".to_string()
    } else {
        "main".to_string()
    }
}

/// Extrae el último mensaje de commit si está disponible.
#[allow(dead_code)]
pub fn read_last_commit_message(root: &std::path::Path) -> Option<String> {
    let git_dir = resolve_git_dir(root)?;
    let msg_path = git_dir.join("COMMIT_EDITMSG");
    if let Ok(content) = std::fs::read_to_string(msg_path) {
        let first_line = content.lines().find(|l| !l.trim().is_empty() && !l.starts_with('#'))?;
        return Some(first_line.to_string());
    }
    None
}

/// Extrae la URL del remote upstream u origin para detectar Fork.
pub fn read_git_remotes_info(root: &std::path::Path) -> (String, String) {
    let Some(git_dir) = resolve_git_dir(root) else {
        return ("autor/repo".into(), "tú/repo".into());
    };
    let config_path = git_dir.join("config");
    let mut origin = "tú/repo".to_string();
    let upstream = "autor/repo".to_string();

    if let Ok(content) = std::fs::read_to_string(config_path) {
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("url = ") {
                let clean = rest.trim_end_matches(".git");
                if let Some(pos) = clean.rfind(':') {
                    origin = clean[pos + 1..].to_string();
                } else if let Some(pos) = clean.rfind("github.com/") {
                    origin = clean[pos + 11..].to_string();
                }
            }
        }
    }
    (upstream, origin)
}

/// Consulta de forma síncrona/ligera metadatos de GitHub PR si `gh` está disponible.
pub fn try_query_github_pr(root: &std::path::Path) -> Option<(u32, String, (usize, usize), bool, Vec<u32>)> {
    let output = std::process::Command::new("gh")
        .current_dir(root)
        .args([
            "pr",
            "view",
            "--json",
            "number,title,additions,deletions,reviewDecision,baseRefName",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    #[derive(serde::Deserialize)]
    struct GhPr {
        number: u32,
        title: String,
        additions: usize,
        deletions: usize,
        #[serde(rename = "reviewDecision")]
        review_decision: Option<String>,
    }

    let pr: GhPr = serde_json::from_slice(&output.stdout).ok()?;
    let approved = pr.review_decision.as_deref() == Some("APPROVED");
    let stack = vec![pr.number.saturating_sub(1), pr.number, pr.number + 1];
    Some((pr.number, pr.title, (pr.additions, pr.deletions), approved, stack))
}

/// Construye un snapshot vivo y dinámico a partir de la inspección de metadatos locales y GitHub.
pub fn build_live_git_flow_snapshot(
    root: &std::path::Path,
    branch_override: Option<&str>,
    ahead_behind: Option<(usize, usize)>,
) -> super::git_flow::GitFlowCanvasSnapshot {
    let (detected_branch, head_sha) = read_git_head_info(root);
    let active_branch = branch_override
        .or(detected_branch.as_deref())
        .unwrap_or("main")
        .to_string();
    let current_sha = head_sha.unwrap_or_else(|| "5f074e5".to_string());
    let (ahead, behind) = ahead_behind.unwrap_or((0, 0));

    let mut steps = Vec::new();

    // Metadatos vivos de la sesión activa
    let base_branch = detect_default_base_branch(root);
    let recent_commits = read_git_recent_commits(root, 2);
    let committed_files = read_git_last_commit_files(root);
    let working_files = read_git_working_tree_files(root);
    let pr_data = try_query_github_pr(root).map(|(num, title, _, approved, _)| (num, title, approved));

    let active_session = Some(super::git_flow::WorktreeSessionFlow {
        workspace_id: String::new(),
        workspace_name: root.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| "workspace".to_string()),
        branch: active_branch.clone(),
        is_focused: true,
        agent_name: Some("pi".to_string()),
        agent_status: Some(crate::api::schema::AgentStatus::Working),
        last_commits: recent_commits.clone(),
        committed_files: committed_files.clone(),
        working_files: working_files.clone(),
        pr_info: pr_data.clone(),
    });

    // 01 COMMIT
    let commit_msg = recent_commits.first().map(|(_, msg)| msg.clone()).unwrap_or_else(|| "feat: commit actual".to_string());
    steps.push(super::git_flow::GitFlowStep::Commit {
        hash: current_sha,
        branch: active_branch.clone(),
        message: commit_msg,
        files: committed_files.clone(),
        upstream: format!("origin/{base_branch}"),
    });

    // 02 BRANCH
    steps.push(super::git_flow::GitFlowStep::Branch {
        name: active_branch.clone(),
        ahead,
        behind,
    });

    // 06 STASH (solo si hay stashes reales)
    if let Some((count, hint)) = read_git_stash_info(root) {
        steps.push(super::git_flow::GitFlowStep::Stash {
            index: count.saturating_sub(1),
            file_hint: hint,
        });
    }

    // 07 TAG (solo si hay tags reales)
    if let Some(tag_name) = read_git_latest_tag(root) {
        steps.push(super::git_flow::GitFlowStep::Tag {
            name: tag_name,
            target_ref: "main".into(),
        });
    }

    // Operación en progreso especial si la hay
    let active_alert = match detect_in_progress_git_op(root) {
        Some(InProgressGitOp::Rebase { step, total, onto_commit, has_conflict }) => {
            steps.push(super::git_flow::GitFlowStep::Rebase {
                in_progress: true,
                step,
                total,
                commit: onto_commit,
                has_conflict,
            });
            if has_conflict {
                Some("CONFLICTO DETECTADO EN REBASE".to_string())
            } else {
                Some(format!("REBASE EN PROGRESO [{step}/{total}]"))
            }
        }
        Some(InProgressGitOp::Merge { merge_msg }) => {
            steps.push(super::git_flow::GitFlowStep::Merge {
                branch: active_branch.clone(),
                target: "main".into(),
                is_merge_commit: true,
            });
            Some(format!("MERGE ACTIVO: {merge_msg}"))
        }
        Some(InProgressGitOp::CherryPick { commit_hash }) => {
            steps.push(super::git_flow::GitFlowStep::CherryPick {
                commit_hash,
                note: "copiando commit".into(),
            });
            Some("CHERRY-PICK EN CURSO".to_string())
        }
        None => None,
    };

    // 08 FORK (solo si hay remote configurado)
    let (upstream, repo_origin) = read_git_remotes_info(root);
    if !repo_origin.is_empty() && repo_origin != "tú/repo" {
        steps.push(super::git_flow::GitFlowStep::Fork {
            upstream_repo: upstream,
            my_repo: repo_origin,
            ahead_commits: ahead,
        });
    }

    // 09 PULL REQUEST & 10 STACKED (solo si gh devuelve un PR real)
    if let Some((pr_num, pr_title, delta, approved, stack)) = try_query_github_pr(root) {
        steps.push(super::git_flow::GitFlowStep::PullRequest {
            number: pr_num,
            title: pr_title,
            delta,
            approved,
        });
        steps.push(super::git_flow::GitFlowStep::StackedPrs {
            prs: stack.clone(),
            current_pr: pr_num,
        });
    }

    super::git_flow::GitFlowCanvasSnapshot {
        steps,
        scroll_offset: 0,
        active_alert,
        active_session,
        sibling_sessions: Vec::new(),
        base_branch,
        cross_session_conflicts: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_git_head_short_resolves_valid_sha() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        if let Some(sha) = read_git_head_short(root) {
            assert_eq!(sha.len(), 7);
            assert!(sha.chars().all(|c| c.is_ascii_hexdigit()));
        }
    }

    #[test]
    fn test_read_git_head_info_and_live_snapshot() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let (branch, sha) = read_git_head_info(root);
        assert!(branch.is_some() || sha.is_some());

        // El snapshot vivo solo emite los pasos que existen realmente en el repo
        let snapshot = build_live_git_flow_snapshot(root, None, Some((1, 0)));
        assert!(!snapshot.steps.is_empty());
        // Al menos commit y branch siempre existen en un repo válido
        assert!(snapshot.steps.len() >= 2);
        assert!(snapshot.active_session.is_some());
    }

    #[test]
    fn test_clean_file_basename_trims_paths() {
        assert_eq!(clean_file_basename("src/client/shell/dock.rs"), "dock.rs");
        assert_eq!(clean_file_basename("report.md"), "report.md");
    }

    #[test]
    fn test_is_sensitive_file_detects_credentials_without_false_positives() {
        assert!(is_sensitive_file(".env"));
        assert!(is_sensitive_file(".env.local"));
        assert!(is_sensitive_file(".env.production"));
        assert!(!is_sensitive_file(".env.example"));
        assert!(!is_sensitive_file(".env.sample"));
        assert!(is_sensitive_file(".ssh/id_rsa"));
        assert!(is_sensitive_file("id_ed25519"));
        assert!(is_sensitive_file("cert/server.key"));
        assert!(is_sensitive_file("ca.pem"));
        assert!(is_sensitive_file("credentials.json"));
        assert!(is_sensitive_file("client_secret.json"));
        assert!(is_sensitive_file("secrets.yaml"));
        assert!(is_sensitive_file(".netrc"));
        assert!(is_sensitive_file(".npmrc"));

        // Source code must NOT be flagged as sensitive
        assert!(!is_sensitive_file("src/auth.rs"));
        assert!(!is_sensitive_file("pkg/authenticator.go"));
        assert!(!is_sensitive_file("src/token_parser.ts"));
        assert!(!is_sensitive_file("package.json"));
        assert!(!is_sensitive_file("Cargo.toml"));
        assert!(!is_sensitive_file("README.md"));
    }

    #[test]
    fn test_sanitize_secret_line_redacts_tokens_and_keys() {
        let line1 = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
        assert_eq!(sanitize_secret_line(line1), "Authorization: Bearer ********");

        let line2 = "OPENAI_KEY = 'sk-proj-abc1234567890abcdef1234567890'";
        assert!(sanitize_secret_line(line2).contains("sk-***") || sanitize_secret_line(line2).contains("********"));

        let line3 = "GITHUB_PAT = ghp_1234567890abcdef1234567890abcdef";
        assert!(sanitize_secret_line(line3).contains("ghp_***") || sanitize_secret_line(line3).contains("********"));

        let line4 = "-----BEGIN RSA PRIVATE KEY-----";
        assert_eq!(sanitize_secret_line(line4), "[-----REDACTED PRIVATE KEY-----]");

        let line5 = "let normal_code = 123 + 456;";
        assert_eq!(sanitize_secret_line(line5), line5);
    }
}

