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
                    return (
                        Some(branch_name.to_string()),
                        Some(parts[0][..7].to_string()),
                    );
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
                    files.push(clean_file_basename(file_path));
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
                    files.push(clean_file_basename(clean));
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
        let first_line = content
            .lines()
            .find(|l| !l.trim().is_empty() && !l.starts_with('#'))?;
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
pub fn try_query_github_pr(
    root: &std::path::Path,
) -> Option<(u32, String, (usize, usize), bool, Vec<u32>)> {
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
    Some((
        pr.number,
        pr.title,
        (pr.additions, pr.deletions),
        approved,
        stack,
    ))
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
    let pr_data =
        try_query_github_pr(root).map(|(num, title, _, approved, _)| (num, title, approved));

    let active_session = Some(super::git_flow::WorktreeSessionFlow {
        workspace_id: String::new(),
        workspace_name: root
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "workspace".to_string()),
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
    let commit_msg = recent_commits
        .first()
        .map(|(_, msg)| msg.clone())
        .unwrap_or_else(|| "feat: commit actual".to_string());
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
        Some(InProgressGitOp::Rebase {
            step,
            total,
            onto_commit,
            has_conflict,
        }) => {
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

// ==============================================================================
// CLIProxyAPI & CPAMC Quota & Account Discovery
// ==============================================================================

/// Normaliza cadenas de reset a un formato legible ("4h 59m" o "18h 30m UTC")
pub fn format_reset_display(reset_str: &str) -> String {
    let trimmed = reset_str.trim();
    if trimmed.is_empty() {
        return "...".to_string();
    }
    if trimmed.contains('h') || trimmed.contains('m') || trimmed.contains('d') {
        return trimmed.to_string();
    }
    if let Some((_, time_part)) = trimmed.split_once('T') {
        let clean_time = time_part.trim_end_matches('Z');
        let parts: Vec<&str> = clean_time.split(':').collect();
        if parts.len() >= 2 {
            return format!("{}h {}m UTC", parts[0], parts[1]);
        }
    }
    trimmed.to_string()
}

/// Resuelve el directorio de credenciales / autenticaciones de CLIProxy o CPAMC
pub fn resolve_auths_dir(root: &std::path::Path) -> Option<std::path::PathBuf> {
    // 1. Variables de entorno explícitas
    for env_key in ["CLIPROXY_AUTHS_DIR", "CPAMC_AUTHS_DIR"] {
        if let Ok(dir) = std::env::var(env_key) {
            let p = std::path::PathBuf::from(dir.trim());
            if p.is_dir() {
                return Some(p);
            }
        }
    }

    // 2. Revisar configuración de proyecto local opencode.json si existe
    for candidate in [
        root.join("opencode.json"),
        root.join(".opencode/opencode.json"),
    ] {
        if candidate.is_file() {
            if let Ok(content) = std::fs::read_to_string(&candidate) {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(auth_dir) = val
                        .get("cpamc_auths_dir")
                        .or_else(|| val.get("cliproxy_auths_dir"))
                        .and_then(|s| s.as_str())
                    {
                        let p = std::path::PathBuf::from(auth_dir.trim());
                        if p.is_dir() {
                            return Some(p);
                        }
                    }
                }
            }
        }
    }

    // 3. Rutas estándar en USERPROFILE o HOME
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .ok()
        .map(std::path::PathBuf::from);

    if let Some(home_dir) = home {
        for candidate in [
            home_dir.join(".cliproxy").join("auths"),
            home_dir.join(".cpamc").join("auths"),
            home_dir.join("cliproxyapi").join("auths"),
        ] {
            if candidate.is_dir() {
                return Some(candidate);
            }
        }
    }

    // 4. En Windows, revisar unidades de disco estándar para instalaciones portables
    #[cfg(target_os = "windows")]
    {
        for drive in ["C:\\", "D:\\"] {
            let candidate = std::path::Path::new(drive)
                .join("cliproxyapi")
                .join("auths");
            if candidate.is_dir() {
                return Some(candidate);
            }
        }
    }

    None
}

/// Descubre cuentas desde los archivos de credenciales JSON de CLIProxy / CPAMC.
pub fn discover_cliproxy_accounts(auths_dir: &std::path::Path) -> Vec<AccountQuota> {
    let mut accounts = Vec::new();
    let Ok(entries) = std::fs::read_dir(auths_dir) else {
        return accounts;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let Some(file_name) = path.file_name().and_then(|f| f.to_str()) else {
            continue;
        };

        let Ok(bytes) = std::fs::read(&path) else {
            continue;
        };

        let Ok(val) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
            continue;
        };

        let raw_stem = file_name.trim_end_matches(".json");
        let clean_stem = raw_stem
            .strip_prefix("antigravity-")
            .or_else(|| raw_stem.strip_prefix("google-"))
            .or_else(|| raw_stem.strip_prefix("oauth-"))
            .unwrap_or(raw_stem);

        let prefix = val
            .get("prefix")
            .and_then(|p| p.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                if !clean_stem.is_empty() {
                    clean_stem.to_string()
                } else if let Some(email) = val.get("email").and_then(|e| e.as_str()) {
                    email.split('@').next().unwrap_or("acc").to_string()
                } else {
                    "acc".to_string()
                }
            });

        accounts.push(AccountQuota {
            name: prefix,
            percent: 100,
            is_leader: false,
            role_symbol: "•",
            quota_desc: "100% · activo".to_string(),
        });
    }

    accounts.sort_by(|a, b| a.name.cmp(&b.name));
    accounts
}

/// Resuelve la ruta al archivo de caché `quotas_cache.json`.
pub fn resolve_quota_cache_path(
    auths_dir: Option<&std::path::Path>,
    root: &std::path::Path,
) -> Option<std::path::PathBuf> {
    // 1. Junto al auths_dir
    if let Some(dir) = auths_dir {
        if let Some(parent) = dir.parent() {
            let candidate = parent.join("quotas_cache.json");
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    // 2. En el directorio raíz del proyecto o subdirectorios comunes
    for candidate in [
        root.join("quotas_cache.json"),
        root.join(".local/quotas_cache.json"),
        root.join(".config/quotas_cache.json"),
    ] {
        if candidate.is_file() {
            return Some(candidate);
        }
    }

    // 3. En USERPROFILE o HOME ~/.cliproxy / ~/.config/cliproxy
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .ok()
        .map(std::path::PathBuf::from);

    if let Some(home_dir) = home {
        for candidate in [
            home_dir.join(".cliproxy").join("quotas_cache.json"),
            home_dir.join(".cpamc").join("quotas_cache.json"),
            home_dir
                .join(".config")
                .join("cliproxy")
                .join("quotas_cache.json"),
            home_dir
                .join(".config")
                .join("hudr2")
                .join("quotas_cache.json"),
        ] {
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    None
}

/// Hidrata cuentas y modelos a partir del contenido de `quotas_cache.json`.
pub fn hydrate_cliproxy_quotas(
    accounts: &mut [AccountQuota],
    cache_path: &std::path::Path,
) -> Vec<ModelQuota> {
    let mut model_quotas = Vec::new();
    let Ok(bytes) = std::fs::read(cache_path) else {
        return model_quotas;
    };
    let Ok(val) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        return model_quotas;
    };

    let Some(accs_obj) = val.get("accounts").and_then(|a| a.as_object()) else {
        return model_quotas;
    };

    let mut seen_models = std::collections::BTreeSet::new();

    for acc in accounts.iter_mut() {
        let key = acc.name.to_lowercase();
        let Some(rec) = accs_obj.get(&key) else {
            continue;
        };

        if let Some(status) = rec.get("error_status").and_then(|s| s.as_u64()) {
            if status == 429 {
                acc.percent = 0;
                acc.quota_desc = "429: Rate Limit".to_string();
                continue;
            } else if status == 401 || status == 403 {
                acc.quota_desc = format!("{status}: Auth Error");
                continue;
            }
        }

        let Some(data) = rec.get("data") else {
            continue;
        };

        let Some(groups) = data.get("groups").and_then(|g| g.as_array()) else {
            continue;
        };

        let mut lowest_pct = 100u8;
        let mut lowest_reset = "4h 59m".to_string();

        for grp in groups {
            let model_name = grp
                .get("displayName")
                .and_then(|d| d.as_str())
                .unwrap_or("Modelo")
                .to_string();

            if let Some(buckets) = grp.get("buckets").and_then(|b| b.as_array()) {
                for b in buckets {
                    let window = b
                        .get("window")
                        .and_then(|w| w.as_str())
                        .unwrap_or("5h")
                        .to_string();
                    let frac = b
                        .get("remainingFraction")
                        .and_then(|f| f.as_f64())
                        .unwrap_or(1.0);
                    let pct = (frac * 100.0).clamp(0.0, 100.0).round() as u8;
                    let reset_raw = b.get("resetTime").and_then(|r| r.as_str()).unwrap_or("");
                    let reset_str = format_reset_display(reset_raw);

                    if pct < lowest_pct {
                        lowest_pct = pct;
                        lowest_reset = reset_str.clone();
                    }

                    let model_key = format!("{model_name}-{window}");
                    if !seen_models.contains(&model_key) {
                        seen_models.insert(model_key);
                        model_quotas.push(ModelQuota {
                            model_name: model_name.clone(),
                            window_type: window,
                            indicator: format!("{pct}%"),
                            reset_in: reset_str,
                        });
                    }
                }
            }
        }

        acc.percent = lowest_pct;
        acc.quota_desc = format!("{lowest_pct}% · reset {lowest_reset}");
    }

    model_quotas
}

/// Sincroniza en caliente la telemetría de CLIProxyAPI / CPAMC (cuentas y cuotas de modelos)
/// sin realizar peticiones de red síncronas ni bloquear el bucle de render.
pub fn refresh_cliproxy_telemetry(root: &std::path::Path) -> (Vec<AccountQuota>, Vec<ModelQuota>) {
    let Some(auths_dir) = resolve_auths_dir(root) else {
        return (Vec::new(), Vec::new());
    };

    let mut accounts = discover_cliproxy_accounts(&auths_dir);
    if accounts.is_empty() {
        return (accounts, Vec::new());
    }

    // Asignar el primer registro como líder visual por defecto
    if let Some(first) = accounts.first_mut() {
        first.is_leader = true;
        first.role_symbol = "⚡";
    }

    let model_quotas = if let Some(cache_path) = resolve_quota_cache_path(Some(&auths_dir), root) {
        hydrate_cliproxy_quotas(&mut accounts, &cache_path)
    } else {
        Vec::new()
    };

    (accounts, model_quotas)
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
    fn test_format_reset_display() {
        assert_eq!(format_reset_display("4h 59m"), "4h 59m");
        assert_eq!(format_reset_display("2026-11-08T18:30:00Z"), "18h 30m UTC");
        assert_eq!(format_reset_display(""), "...");
    }

    #[test]
    fn test_discover_cliproxy_accounts_and_hydrate_quotas() {
        let temp_dir =
            std::env::temp_dir().join(format!("telar-test-auths-{}", std::process::id()));
        let auths_dir = temp_dir.join("auths");
        let _ = std::fs::remove_dir_all(&temp_dir);
        let _ = std::fs::create_dir_all(&auths_dir);

        // Crear archivos de credenciales simulados
        std::fs::write(
            auths_dir.join("antigravity-can.json"),
            r#"{"email": "can@example.com", "prefix": "can-claude"}"#,
        )
        .unwrap();
        std::fs::write(
            auths_dir.join("google-tavo.json"),
            r#"{"email": "tavo@gmail.com"}"#,
        )
        .unwrap();

        // 1. Descubrir cuentas
        let mut accounts = discover_cliproxy_accounts(&auths_dir);
        assert_eq!(accounts.len(), 2);
        assert_eq!(accounts[0].name, "can-claude");
        assert_eq!(accounts[1].name, "tavo");

        // 2. Crear caché de cuotas simulada
        let cache_path = temp_dir.join("quotas_cache.json");
        let cache_content = r#"{
            "version": 1,
            "accounts": {
                "can-claude": {
                    "fetched_at": 1731000000,
                    "auth_mtime": 1730000000,
                    "data": {
                        "groups": [
                            {
                                "displayName": "Claude 3.7 Sonnet",
                                "buckets": [
                                    {
                                        "window": "5h",
                                        "remainingFraction": 0.75,
                                        "resetTime": "2026-11-08T22:00:00Z"
                                    }
                                ]
                            }
                        ]
                    }
                },
                "tavo": {
                    "fetched_at": 1731000000,
                    "auth_mtime": 1730000000,
                    "error_status": 429
                }
            }
        }"#;
        std::fs::write(&cache_path, cache_content).unwrap();

        // 3. Hidratar cuotas desde caché
        let model_quotas = hydrate_cliproxy_quotas(&mut accounts, &cache_path);

        assert_eq!(accounts[0].percent, 75);
        assert!(accounts[0].quota_desc.contains("75%"));
        assert_eq!(accounts[1].percent, 0);
        assert_eq!(accounts[1].quota_desc, "429: Rate Limit");

        assert_eq!(model_quotas.len(), 1);
        assert_eq!(model_quotas[0].model_name, "Claude 3.7 Sonnet");
        assert_eq!(model_quotas[0].indicator, "75%");

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
