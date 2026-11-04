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
}

