use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;

use super::dock_icons::resolve_file_icon;
use super::state::ClientShellConfig;

/// Identificador único de cada pestaña del inspector derecho
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DockTabId {
    #[default]
    Files,
    Status,
    GitFlow,
}

// Backward compatibility alias for deprecated Changes tab identifier.
#[allow(dead_code, non_upper_case_globals)]
pub const Changes: DockTabId = DockTabId::GitFlow;

#[allow(dead_code)]
impl DockTabId {
    pub fn title(self) -> &'static str {
        match self {
            Self::Status => "Status",
            Self::Files => "Files",
            Self::GitFlow => "Git Flow",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Status => "⚡",
            Self::Files => "📁",
            Self::GitFlow => "⎇",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Files => Self::Status,
            Self::Status => Self::GitFlow,
            Self::GitFlow => Self::Files,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Files => Self::GitFlow,
            Self::Status => Self::Files,
            Self::GitFlow => Self::Status,
        }
    }
}

/// Contexto de un workspace hermano que comparte el mismo repositorio base Git
pub struct SiblingWorkspaceContext<'a> {
    pub workspace_id: &'a str,
    pub workspace_name: &'a str,
    pub root_path: &'a Path,
    pub branch: Option<&'a str>,
    pub agent_status: Option<crate::api::schema::AgentStatus>,
    pub agent_name: Option<&'a str>,
}

/// Contexto del workspace activo provisto por referencia en cada frame (Punto 7)
pub struct WorkspaceContext<'a> {
    pub workspace_id: &'a str,
    pub workspace_name: &'a str,
    pub root_path: &'a Path,
    pub branch: Option<&'a str>,
    pub git_ahead_behind: Option<(usize, usize)>,
    pub agent_status: Option<crate::api::schema::AgentStatus>,
    pub agent_name: Option<&'a str>,
    // Retained for agent token breakdown integration in right dock
    #[allow(dead_code)]
    pub agent_tokens: Option<&'a [(String, String)]>,
    pub is_preview: bool,
    pub siblings: Vec<SiblingWorkspaceContext<'a>>,
}

/// Estado del modal flotante Quick Peek (Punto 6)
#[derive(Debug, Clone)]
pub struct QuickPeekState {
    pub title: String,
    pub lines: Vec<String>,
    pub scroll: usize,
}

impl QuickPeekState {
    pub fn new(title: impl Into<String>, lines: Vec<String>) -> Self {
        Self {
            title: title.into(),
            lines,
            scroll: 0,
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        if self.scroll + 1 < self.lines.len() {
            self.scroll += 1;
        }
    }
}

/// Contrato base de pestaña con ciclo de vida perezoso/reactivo (Punto 1)
#[allow(dead_code)]
pub trait DockTab: Send + Sync {
    fn id(&self) -> DockTabId;
    fn is_dirty(&self) -> bool;
    fn mark_dirty(&mut self);
    fn mark_clean(&mut self);
    fn hibernate(&mut self);
}

/// Pestaña 1: Status (Gentle HUD)
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct StatusTab {
    is_dirty: bool,
    cached_root: Option<PathBuf>,
    pub active_agent_status: Option<String>,
    pub running_jobs_count: usize,
    pub todos_summary: Option<String>,
    pub collapsed_sections: BTreeSet<u8>,
    pub telemetry: super::telemetry::StatusSnapshot,
}

impl Default for StatusTab {
    fn default() -> Self {
        Self::new()
    }
}

impl StatusTab {
    pub fn new() -> Self {
        let collapsed = BTreeSet::new();
        // In Gentle Shell flat rail layout, all sections (TODOS, AGENTS, CHANGES, STATS, INTEGRATIONS)
        // are open by default for immediate HUD visibility.
        Self {
            is_dirty: true,
            cached_root: None,
            active_agent_status: None,
            running_jobs_count: 0,
            todos_summary: None,
            collapsed_sections: collapsed,
            telemetry: super::telemetry::StatusSnapshot::default(),
        }
    }

    pub fn refresh_if_needed(&mut self, root: &Path, branch: Option<&str>) {
        if !self.is_dirty && self.cached_root.as_deref() == Some(root) {
            return;
        }

        self.cached_root = Some(root.to_path_buf());
        if let Some(b) = branch {
            self.telemetry.git.branch = b.to_string();
        }
        if let Some(head) = super::telemetry::read_git_head_short(root) {
            self.telemetry.git.head_commit = Some(head);
        }

        // Descubrimiento de cuotas e integraciones de CLIProxy / CPAMC
        let (accounts, model_quotas) = super::telemetry::refresh_cliproxy_telemetry(root);
        self.telemetry.accounts = accounts;
        self.telemetry.model_quotas = model_quotas;

        self.is_dirty = false;
    }

    pub fn sync_agent_telemetry(&mut self, tokens: Option<&[(String, String)]>) {
        let Some(tokens) = tokens else {
            // Si el pane activo no tiene tokens, resetear la telemetría viva a default
            self.telemetry.todos.clear();
            self.telemetry.live_stats = super::telemetry::SessionLiveStats::default();
            self.telemetry.context = super::telemetry::ContextUsage::default();
            self.telemetry.model_name.clear();
            self.telemetry.engram = super::telemetry::EngramTelemetry::default();
            return;
        };

        let mut todos = Vec::new();
        let mut has_stats_update = false;

        for (k, v) in tokens {
            if k == "profile" {
                self.telemetry.model_name = v.clone();
            } else if k == "ctx_pct" {
                if let Ok(pct) = v.parse::<f32>() {
                    self.telemetry.context.percent = pct;
                }
            } else if k == "ctx_tok" {
                if let Ok(tok) = v.parse::<u64>() {
                    self.telemetry.context.used_tokens = tok;
                }
            } else if k == "ctx_win" {
                if let Ok(win) = v.parse::<u64>() {
                    self.telemetry.context.limit_tokens = win;
                }
            } else if k == "engram_obs" {
                if let Ok(count) = v.parse::<usize>() {
                    self.telemetry.engram.observations_count = count;
                    self.telemetry.engram.is_online = true;
                }
            } else if k == "cost" {
                if let Ok(c) = v.parse::<f32>() {
                    self.telemetry.live_stats.spent_cost = c;
                    has_stats_update = true;
                }
            } else if k == "turns" {
                if let Ok(t) = v.parse::<usize>() {
                    self.telemetry.live_stats.turns = t;
                    has_stats_update = true;
                }
            } else if k == "tok_in" {
                if let Ok(tin) = v.parse::<u64>() {
                    self.telemetry.live_stats.tokens_in = tin;
                    has_stats_update = true;
                }
            } else if k == "tok_out" {
                if let Ok(tout) = v.parse::<u64>() {
                    self.telemetry.live_stats.tokens_out = tout;
                    has_stats_update = true;
                }
            } else if k == "elapsed" {
                self.telemetry.live_stats.elapsed_str = v.clone();
                has_stats_update = true;
            } else if k == "git_files" {
                if let Ok(files) = v.parse::<usize>() {
                    self.telemetry.git.agent_files_changed = files;
                }
            } else if k == "git_lines" {
                if let Ok(lines) = v.parse::<usize>() {
                    self.telemetry.git.agent_lines_added = lines;
                }
            } else if let Some(todo_idx_str) = k.strip_prefix("todo_") {
                // Formato: "todo_1" => "done:Titulo" o "prog:Titulo" o "open:Titulo"
                if let Some((status_prefix, title)) = v.split_once(':') {
                    let is_done = status_prefix == "done";
                    let is_in_progress = status_prefix == "prog";
                    todos.push((
                        todo_idx_str.parse::<usize>().unwrap_or(0),
                        super::telemetry::SessionTodoItem {
                            title: title.to_string(),
                            is_done,
                            is_in_progress,
                        },
                    ));
                }
            }
        }

        if !todos.is_empty() {
            todos.sort_by_key(|(idx, _)| *idx);
            self.telemetry.todos = todos.into_iter().map(|(_, item)| item).collect();
        }

        if has_stats_update {
            self.telemetry.live_stats.model_name = self.telemetry.model_name.clone();
        }
    }

    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }

    pub fn toggle_section(&mut self, section: u8) {
        if self.collapsed_sections.contains(&section) {
            self.collapsed_sections.remove(&section);
        } else {
            self.collapsed_sections.insert(section);
        }
        self.mark_dirty();
    }

    pub fn collapse_all(&mut self) {
        for s in 1..=9 {
            self.collapsed_sections.insert(s);
        }
        self.mark_dirty();
    }

    pub fn expand_all(&mut self) {
        self.collapsed_sections.clear();
        self.mark_dirty();
    }

    pub fn is_collapsed(&self, section: u8) -> bool {
        self.collapsed_sections.contains(&section)
    }
}

impl DockTab for StatusTab {
    fn id(&self) -> DockTabId {
        DockTabId::Status
    }
    fn is_dirty(&self) -> bool {
        self.is_dirty
    }
    fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }
    fn mark_clean(&mut self) {
        self.is_dirty = false;
    }
    fn hibernate(&mut self) {
        // En hibernación no consumimos ciclos de polling
    }
}

/// Entrada individual dentro del árbol jerárquico del explorador de archivos
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileTreeEntry {
    pub name: String,
    pub relative_path: PathBuf,
    pub is_dir: bool,
    pub depth: usize,
    pub is_expanded: bool,
}

const MAX_TREE_ENTRIES: usize = 1000;

fn scan_tree_dir(
    root: &Path,
    rel_dir: &Path,
    depth: usize,
    expanded: &BTreeSet<PathBuf>,
    show_hidden: bool,
    entries: &mut Vec<FileTreeEntry>,
) {
    if depth > 20 || entries.len() >= MAX_TREE_ENTRIES {
        return;
    }
    let target_dir = if rel_dir.as_os_str().is_empty() {
        root.to_path_buf()
    } else {
        root.join(rel_dir)
    };

    let Ok(dir_entries) = std::fs::read_dir(&target_dir) else {
        return;
    };

    let mut items = Vec::new();
    for entry in dir_entries.flatten() {
        if entries.len() + items.len() >= MAX_TREE_ENTRIES {
            break;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name == ".git" {
            continue;
        }
        if !show_hidden && name.starts_with('.') {
            continue;
        }
        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        items.push((name, is_dir));
    }

    items.sort_by(|(name_a, dir_a), (name_b, dir_b)| {
        dir_b.cmp(dir_a).then_with(|| name_a.cmp(name_b))
    });

    for (name, is_dir) in items {
        let rel_path = if rel_dir.as_os_str().is_empty() {
            PathBuf::from(&name)
        } else {
            rel_dir.join(&name)
        };
        let is_expanded = is_dir && expanded.contains(&rel_path);
        entries.push(FileTreeEntry {
            name,
            relative_path: rel_path.clone(),
            is_dir,
            depth,
            is_expanded,
        });

        if is_expanded {
            scan_tree_dir(root, &rel_path, depth + 1, expanded, show_hidden, entries);
        }
    }
}

/// Pestaña 2: Files (Explorador perezoso con caché bajo demanda y jerarquía)
#[derive(Debug)]
pub struct FilesTab {
    is_dirty: bool,
    cached_root: Option<PathBuf>,
    pub entries: Vec<FileTreeEntry>,
    pub expanded_dirs: BTreeSet<PathBuf>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub visible_rows: usize,
    pub show_hidden: bool,
    pub search_query: Option<String>,
    pub is_searching: bool,
}

impl Default for FilesTab {
    fn default() -> Self {
        Self {
            is_dirty: false,
            cached_root: None,
            entries: Vec::new(),
            expanded_dirs: BTreeSet::new(),
            selected_index: 0,
            scroll_offset: 0,
            visible_rows: 0,
            show_hidden: true,
            search_query: None,
            is_searching: false,
        }
    }
}

impl DockTab for FilesTab {
    fn id(&self) -> DockTabId {
        DockTabId::Files
    }
    fn is_dirty(&self) -> bool {
        self.is_dirty
    }
    fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }
    fn mark_clean(&mut self) {
        self.is_dirty = false;
    }
    fn hibernate(&mut self) {
        // Al hibernar, mantenemos la lista pero no hacemos escaneos
    }
}

impl FilesTab {
    pub fn rebuild_tree(&mut self, root: &Path) {
        self.cached_root = Some(root.to_path_buf());
        self.entries.clear();
        scan_tree_dir(
            root,
            Path::new(""),
            0,
            &self.expanded_dirs,
            self.show_hidden,
            &mut self.entries,
        );
        if self.selected_index >= self.entries.len() {
            self.selected_index = self.entries.len().saturating_sub(1);
        }
        self.ensure_selected_visible();
        self.is_dirty = false;
    }

    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        if let Some(root) = self.cached_root.clone() {
            self.rebuild_tree(&root);
        }
    }

    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }

    pub fn refresh_if_needed(&mut self, root: &Path) {
        if !self.is_dirty && self.cached_root.as_deref() == Some(root) {
            return;
        }

        if self.cached_root.as_deref() != Some(root) {
            self.expanded_dirs.clear();
            self.selected_index = 0;
            self.scroll_offset = 0;
        }
        self.rebuild_tree(root);
    }

    pub fn start_search(&mut self) {
        self.is_searching = true;
        if self.search_query.is_none() {
            self.search_query = Some(String::new());
        }
        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    pub fn cancel_search(&mut self) {
        self.is_searching = false;
        self.search_query = None;
        self.clamp_selection();
    }

    pub fn finish_search(&mut self) {
        self.is_searching = false;
        self.clamp_selection();
    }

    pub fn search_push(&mut self, c: char) {
        let query = self.search_query.get_or_insert_with(String::new);
        query.push(c);
        self.selected_index = 0;
        self.scroll_offset = 0;
        self.clamp_selection();
    }

    pub fn search_backspace(&mut self) {
        if let Some(query) = &mut self.search_query {
            query.pop();
            self.selected_index = 0;
            self.scroll_offset = 0;
            self.clamp_selection();
        }
    }

    pub fn filtered_indices(&self) -> Vec<usize> {
        if let Some(query) = &self.search_query {
            let q = query.to_lowercase();
            if q.is_empty() {
                (0..self.entries.len()).collect()
            } else {
                self.entries
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| {
                        e.name.to_lowercase().contains(&q)
                            || e.relative_path
                                .to_string_lossy()
                                .to_lowercase()
                                .contains(&q)
                    })
                    .map(|(i, _)| i)
                    .collect()
            }
        } else {
            (0..self.entries.len()).collect()
        }
    }

    pub fn clamp_selection(&mut self) {
        let count = self.filtered_indices().len();
        if self.selected_index >= count {
            self.selected_index = count.saturating_sub(1);
        }
        self.ensure_selected_visible();
    }

    pub fn selected_entry(&self) -> Option<&FileTreeEntry> {
        let indices = self.filtered_indices();
        let &real_idx = indices.get(self.selected_index)?;
        self.entries.get(real_idx)
    }

    /// Retorna la ruta completa y el nombre del directorio seleccionado (útil para abrir como nuevo Space)
    pub fn selected_dir_path(&self) -> Option<(PathBuf, String)> {
        let entry = self.selected_entry()?;
        if !entry.is_dir {
            return None;
        }
        let root = self.cached_root.as_ref()?;
        let full_path = root.join(&entry.relative_path);
        Some((full_path, entry.name.clone()))
    }

    pub fn set_selected_expanded(&mut self, expanded: bool) {
        if let Some(entry) = self.entries.get(self.selected_index) {
            if entry.is_dir {
                let path = entry.relative_path.clone();
                if expanded {
                    self.expanded_dirs.insert(path);
                } else {
                    self.expanded_dirs.remove(&path);
                }
                if let Some(root) = self.cached_root.clone() {
                    self.rebuild_tree(&root);
                }
            }
        }
    }

    pub fn toggle_selected_expanded(&mut self) {
        if let Some(entry) = self.entries.get(self.selected_index) {
            if entry.is_dir {
                let is_expanded = self.expanded_dirs.contains(&entry.relative_path);
                self.set_selected_expanded(!is_expanded);
            }
        }
    }

    pub fn toggle_expand(&mut self, index: usize) {
        if let Some(entry) = self.entries.get(index) {
            if entry.is_dir {
                let path = entry.relative_path.clone();
                if self.expanded_dirs.contains(&path) {
                    self.expanded_dirs.remove(&path);
                } else {
                    self.expanded_dirs.insert(path);
                }
                if let Some(root) = self.cached_root.clone() {
                    self.rebuild_tree(&root);
                }
            }
        }
    }

    pub fn select_next(&mut self) {
        let count = self.filtered_indices().len();
        if count > 0 && self.selected_index + 1 < count {
            self.selected_index += 1;
            self.ensure_selected_visible();
        }
    }

    pub fn select_prev(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
        self.ensure_selected_visible();
    }

    pub fn set_visible_rows(&mut self, rows: usize) {
        self.visible_rows = rows;
        self.ensure_selected_visible();
    }

    pub fn ensure_selected_visible(&mut self) {
        if self.visible_rows == 0 {
            return;
        }
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + self.visible_rows {
            self.scroll_offset = self
                .selected_index
                .saturating_sub(self.visible_rows)
                .saturating_add(1);
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        if self.scroll_offset + self.visible_rows < self.entries.len() {
            self.scroll_offset += 1;
        }
    }
}

/// Pestaña 3: Changes (Git summary reactivo)
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct ChangesTab {
    is_dirty: bool,
    pub last_branch: Option<String>,
    pub ahead_behind: Option<(usize, usize)>,
}

impl DockTab for ChangesTab {
    fn id(&self) -> DockTabId {
        DockTabId::GitFlow
    }
    fn is_dirty(&self) -> bool {
        self.is_dirty
    }
    fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }
    fn mark_clean(&mut self) {
        self.is_dirty = false;
    }
    fn hibernate(&mut self) {}
}

/// Pestaña 3: Git Flow (flujo visual continuo reactivo de Git y GitHub)
#[derive(Debug, Default)]
pub struct GitFlowTab {
    pub canvas: super::git_flow::GitFlowCanvasSnapshot,
    pub is_dirty: bool,
    pub last_branch: Option<String>,
    pub ahead_behind: Option<(usize, usize)>,
    pub cached_root: Option<PathBuf>,
}

impl GitFlowTab {
    pub fn refresh_if_needed(
        &mut self,
        root: &Path,
        branch: Option<&str>,
        ahead_behind: Option<(usize, usize)>,
        siblings: &[SiblingWorkspaceContext],
    ) {
        if !self.is_dirty
            && self.cached_root.as_deref() == Some(root)
            && self.last_branch.as_deref() == branch
            && self.canvas.sibling_sessions.len() == siblings.len()
        {
            return;
        }

        self.cached_root = Some(root.to_path_buf());
        self.last_branch = branch.map(|s| s.to_string());
        self.ahead_behind = ahead_behind;
        let mut snapshot = super::telemetry::build_live_git_flow_snapshot(root, branch, ahead_behind);

        // Poblar sesiones hermanas desde siblings
        let mut sibling_flows = Vec::new();
        for sib in siblings {
            let sib_branch = sib.branch.unwrap_or("main");
            let sib_working_files = super::telemetry::read_git_working_tree_files(sib.root_path);
            let sib_recent_commits = super::telemetry::read_git_recent_commits(sib.root_path, 1);
            let sib_committed_files = super::telemetry::read_git_last_commit_files(sib.root_path);

            sibling_flows.push(super::git_flow::WorktreeSessionFlow {
                workspace_id: sib.workspace_id.to_string(),
                workspace_name: sib.workspace_name.to_string(),
                branch: sib_branch.to_string(),
                is_focused: false,
                agent_name: sib.agent_name.map(|n| n.to_string()),
                agent_status: sib.agent_status,
                last_commits: sib_recent_commits,
                committed_files: sib_committed_files,
                working_files: sib_working_files,
                pr_info: None,
            });
        }

        // Radar de conflictos en caliente entre la sesión activa y las sesiones hermanas
        let mut all_sessions = Vec::new();
        if let Some(active) = &snapshot.active_session {
            all_sessions.push(active.clone());
        }
        all_sessions.extend(sibling_flows.clone());
        snapshot.cross_session_conflicts = super::git_flow::detect_cross_session_conflicts(&all_sessions);
        snapshot.sibling_sessions = sibling_flows;

        self.canvas = snapshot;
        self.is_dirty = false;
    }
}

impl DockTab for GitFlowTab {
    fn id(&self) -> DockTabId {
        DockTabId::GitFlow
    }
    fn is_dirty(&self) -> bool {
        self.is_dirty
    }
    fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }
    fn mark_clean(&mut self) {
        self.is_dirty = false;
    }
    fn hibernate(&mut self) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DockLayoutMode {
    #[default]
    SingleTab,
    SplitVertical,
}

impl DockLayoutMode {
    pub fn toggle(self) -> Self {
        match self {
            Self::SingleTab => Self::SplitVertical,
            Self::SplitVertical => Self::SingleTab,
        }
    }
}

/// Estado de pestañas aislado por cada workspace para conservar memoria de navegación
#[derive(Debug, Default)]
pub struct WorkspaceDockState {
    pub status_tab: StatusTab,
    pub files_tab: FilesTab,
    pub changes_tab: ChangesTab,
    pub git_flow_tab: GitFlowTab,
}

/// Estado global del Right Dock persistido en ClientShellState
#[derive(Debug, Default)]
pub struct RightDockState {
    pub active_tab: DockTabId,
    pub layout_mode: DockLayoutMode,
    pub status_tab: StatusTab,
    pub files_tab: FilesTab,
    pub changes_tab: ChangesTab,
    pub git_flow_tab: GitFlowTab,
    pub workspace_cache: std::collections::HashMap<String, WorkspaceDockState>,
    pub last_workspace_id: Option<String>,
    pub quick_peek: Option<QuickPeekState>,
}

impl RightDockState {
    pub fn select_tab(&mut self, tab: DockTabId) {
        if self.active_tab != tab {
            // Hibernar pestaña previa (Punto 1)
            match self.active_tab {
                DockTabId::Status => self.status_tab.hibernate(),
                DockTabId::Files => self.files_tab.hibernate(),
                DockTabId::GitFlow => {
                    self.git_flow_tab.hibernate();
                    self.changes_tab.hibernate();
                }
            }
            self.active_tab = tab;
            // Marcar como dirty la nueva para refrescar perezosamente
            match self.active_tab {
                DockTabId::Status => self.status_tab.mark_dirty(),
                DockTabId::Files => self.files_tab.mark_dirty(),
                DockTabId::GitFlow => {
                    self.git_flow_tab.mark_dirty();
                    self.changes_tab.mark_dirty();
                }
            }
        }
    }

    /// Sincronización reactiva ante cambio de workspace con preservación de estado
    pub fn sync_workspace(&mut self, current_workspace_id: &str) {
        if self.last_workspace_id.as_deref() != Some(current_workspace_id) {
            // 1. Si había un workspace anterior activo, guardar su estado en la caché
            if let Some(old_id) = self.last_workspace_id.take() {
                let old_state = WorkspaceDockState {
                    status_tab: std::mem::take(&mut self.status_tab),
                    files_tab: std::mem::take(&mut self.files_tab),
                    changes_tab: std::mem::take(&mut self.changes_tab),
                    git_flow_tab: std::mem::take(&mut self.git_flow_tab),
                };
                self.workspace_cache.insert(old_id, old_state);
            }

            // 2. Restaurar estado del nuevo workspace si existía en caché, o inicializar limpio
            if let Some(mut restored) = self.workspace_cache.remove(current_workspace_id) {
                // Marcar como dirty para re-validar frescura sin destruir carpetas expandidas ni scroll
                restored.status_tab.mark_dirty();
                restored.files_tab.mark_dirty();
                restored.changes_tab.mark_dirty();
                restored.git_flow_tab.mark_dirty();
                self.status_tab = restored.status_tab;
                self.files_tab = restored.files_tab;
                self.changes_tab = restored.changes_tab;
                self.git_flow_tab = restored.git_flow_tab;
            } else {
                self.status_tab = StatusTab::default();
                self.files_tab = FilesTab::default();
                self.changes_tab = ChangesTab::default();
                self.git_flow_tab = GitFlowTab::default();
                self.status_tab.mark_dirty();
                self.files_tab.mark_dirty();
                self.changes_tab.mark_dirty();
                self.git_flow_tab.mark_dirty();
            }

            self.last_workspace_id = Some(current_workspace_id.to_string());
            self.quick_peek = None; // Cierra cualquier preview si cambiaste de workspace
        }
    }

    /// Elimina de la caché los workspaces que ya no existen en el servidor
    pub fn prune_dead_workspaces(
        &mut self,
        active_workspace_ids: &std::collections::HashSet<&str>,
    ) {
        self.workspace_cache
            .retain(|id, _| active_workspace_ids.contains(id.as_str()));
    }

    pub fn activate_file_selection(&mut self) {
        if let Some(entry) = self.files_tab.selected_entry().cloned() {
            if entry.is_dir {
                self.files_tab.toggle_selected_expanded();
            } else if let Some(root) = &self.files_tab.cached_root {
                let file_path = root.join(&entry.relative_path);
                // Lectura acotada con tope de tamaño (máx 128 KB y 100 líneas) para evitar freeze
                let lines = match std::fs::File::open(&file_path) {
                    Ok(file) => {
                        use std::io::{BufRead, BufReader, Read};
                        let mut reader = BufReader::new(file.take(128 * 1024));
                        let mut collected = Vec::new();
                        let mut line_buf = String::new();
                        while collected.len() < 100 {
                            line_buf.clear();
                            match reader.read_line(&mut line_buf) {
                                Ok(0) => break, // EOF o reached 128KB
                                Ok(_) => {
                                    let trimmed = line_buf.trim_end_matches(['\r', '\n']);
                                    collected.push(trimmed.to_string());
                                }
                                Err(_) => {
                                    if collected.is_empty() {
                                        collected.push("[Error: Archivo binario o codificación no soportada]".into());
                                    }
                                    break;
                                }
                            }
                        }
                        collected
                    }
                    Err(_) => {
                        vec!["[Error: No se pudo leer el archivo o es inaccesible]".into()]
                    }
                };
                self.quick_peek = Some(QuickPeekState::new(entry.name, lines));
            }
        }
    }
}

pub(super) fn render_right_dock(
    buffer: &mut Buffer,
    area: Rect,
    config: &ClientShellConfig,
    dock_state: &mut RightDockState,
    context: Option<&WorkspaceContext>,
    hits: &mut super::state::ShellHitMap,
) {
    if area.is_empty() {
        return;
    }

    hits.right_dock_area = area;

    // Sincronización atómica de contexto si cambia el workspace (Punto 7)
    if let Some(ctx) = context {
        dock_state.sync_workspace(ctx.workspace_id);
    }

    // Fondo del dock sincronizado con el sidebar izquierdo (sidebar_bg)
    buffer.set_style(area, Style::default().bg(config.palette.sidebar_bg));

    // Separador vertical izquierdo
    let separator_x = area.x;
    for y in area.y..area.bottom() {
        if let Some(cell) = buffer.cell_mut((separator_x, y)) {
            cell.set_symbol("│");
            cell.set_style(Style::default().fg(config.palette.surface_dim));
        }
    }

    // Modo Compact Rail (3 columnas o menos)
    if area.width <= 3 {
        let icon_x = area.x + 1;
        if area.width >= 3 {
            let active = dock_state.active_tab;
            let status_color = if active == DockTabId::Status {
                config.palette.accent
            } else {
                config.palette.subtext0
            };
            let files_color = if active == DockTabId::Files {
                config.palette.accent
            } else {
                config.palette.subtext0
            };
            let git_flow_color = if active == DockTabId::GitFlow {
                config.palette.accent
            } else {
                config.palette.subtext0
            };

            if area.height >= 2 {
                buffer.set_string(icon_x, area.y + 1, "📁", Style::default().fg(files_color));
                hits.right_dock_tabs
                    .push((Rect::new(icon_x, area.y + 1, 2, 1), DockTabId::Files));
            }
            if area.height >= 4 {
                buffer.set_string(icon_x, area.y + 3, "⚡", Style::default().fg(status_color));
                hits.right_dock_tabs
                    .push((Rect::new(icon_x, area.y + 3, 2, 1), DockTabId::Status));
            }
            if area.height >= 6 {
                buffer.set_string(icon_x, area.y + 5, "⎇", Style::default().fg(git_flow_color));
                hits.right_dock_tabs
                    .push((Rect::new(icon_x, area.y + 5, 2, 1), DockTabId::GitFlow));
            }

            if area.height > 0 {
                buffer.set_string(icon_x, area.y, "«", Style::default().fg(config.palette.subtext0));
                hits.right_dock_header_toggle = Rect::new(icon_x, area.y, 1, 1);
            }

            if area.height > 1 {
                let toggle_y = area.bottom().saturating_sub(1);
                buffer.set_string(icon_x, toggle_y, "«", Style::default().fg(config.palette.subtext0));
                hits.right_dock_toggle = Rect::new(icon_x, toggle_y, 1, 1);
            }
        }
        return;
    }

    // Modo Expandido: Tab Header en la primera fila con chips diferenciados (Punto 8)
    let content_x = area.x + 1;
    let tab_y = area.y;

    let tab_text_color = match config.palette.panel_bg {
        ratatui::style::Color::Reset => config.palette.surface_dim,
        c => c,
    };
    let active_style = Style::default()
        .fg(tab_text_color)
        .bg(config.palette.accent)
        .add_modifier(ratatui::style::Modifier::BOLD);
    let inactive_style = Style::default()
        .fg(config.palette.subtext0)
        .bg(if config.palette.sidebar_bg == ratatui::style::Color::Reset {
            config.palette.surface0
        } else {
            config.palette.sidebar_bg
        });

    let status_style = if dock_state.active_tab == DockTabId::Status {
        active_style
    } else {
        inactive_style
    };
    let files_style = if dock_state.active_tab == DockTabId::Files {
        active_style
    } else {
        inactive_style
    };
    let git_flow_style = if dock_state.active_tab == DockTabId::GitFlow {
        active_style
    } else {
        inactive_style
    };

    let collapse_x = area.right().saturating_sub(4);
    let has_collapse_btn = collapse_x > content_x + 11 && area.width >= 16;

    // Indicador Split View a la izquierda del botón de colapso.
    // Usamos formato compacto [⊞] cuando el ancho es ajustado (36..47) para evitar solapar pestañas.
    let (has_split_btn, split_label, split_w) = if area.width >= 48 {
        let label = match dock_state.layout_mode {
            DockLayoutMode::SingleTab => "[⊞ Split]",
            DockLayoutMode::SplitVertical => "[⊟ Single]",
        };
        (true, label, 10u16)
    } else if area.width >= 36 {
        let label = match dock_state.layout_mode {
            DockLayoutMode::SingleTab => "[⊞]",
            DockLayoutMode::SplitVertical => "[⊟]",
        };
        (true, label, 3u16)
    } else {
        (false, "", 0u16)
    };

    let split_x = if has_split_btn {
        collapse_x.saturating_sub(split_w + 1)
    } else {
        collapse_x
    };

    let tab_right_limit = if has_split_btn {
        split_x
    } else if has_collapse_btn {
        collapse_x
    } else {
        area.right()
    };

    buffer.set_string(content_x, tab_y, " 📁 Files ", files_style);
    hits.right_dock_tabs
        .push((Rect::new(content_x, tab_y, 11, 1), DockTabId::Files));

    if content_x + 22 <= tab_right_limit {
        buffer.set_string(content_x + 11, tab_y, " ⚡ Status ", status_style);
        hits.right_dock_tabs
            .push((Rect::new(content_x + 11, tab_y, 11, 1), DockTabId::Status));
    }
    if content_x + 31 <= tab_right_limit {
        buffer.set_string(content_x + 22, tab_y, " ⎇ Flow ", git_flow_style);
        hits.right_dock_tabs
            .push((Rect::new(content_x + 22, tab_y, 9, 1), DockTabId::GitFlow));
    }

    if has_split_btn {
        let split_style = if dock_state.layout_mode == DockLayoutMode::SplitVertical {
            Style::default()
                .fg(config.palette.peach)
                .bg(config.palette.surface0)
        } else {
            Style::default().fg(config.palette.subtext0)
        };
        buffer.set_string(split_x, tab_y, split_label, split_style);
        hits.right_dock_split_toggle = Rect::new(split_x, tab_y, split_w, 1);
    }

    // Botón de colapso [»] en el extremo derecho del header (renderizado después para garantizar visibilidad)
    if has_collapse_btn {
        buffer.set_string(
            collapse_x,
            tab_y,
            "[»]",
            Style::default().fg(config.palette.subtext0),
        );
        hits.right_dock_header_toggle = Rect::new(collapse_x, tab_y, 3, 1);
    }

    // Línea separadora horizontal del header
    let header_divider_y = tab_y + 1;
    if header_divider_y < area.bottom() {
        for x in content_x..area.right() {
            if let Some(cell) = buffer.cell_mut((x, header_divider_y)) {
                cell.set_symbol("─");
                cell.set_style(Style::default().fg(config.palette.overlay0));
            }
        }
    }

    // Renderizado según layout_mode (Punto 5: Split View opcional)
    let body_y = header_divider_y + 1;
    let available_rows = area.bottom().saturating_sub(body_y).saturating_sub(1);

    if dock_state.layout_mode == DockLayoutMode::SplitVertical && available_rows >= 14 {
        let half_rows = available_rows / 2;
        if let Some(ctx) = context {
            dock_state.status_tab.refresh_if_needed(ctx.root_path, ctx.branch);
            dock_state.status_tab.sync_agent_telemetry(ctx.agent_tokens);
        }
        render_status_content(
            buffer,
            content_x,
            body_y,
            half_rows,
            area.width,
            config,
            context,
            &mut dock_state.status_tab,
            Some(hits),
        );

        let split_divider_y = body_y + half_rows;
        for x in content_x..area.right() {
            if let Some(cell) = buffer.cell_mut((x, split_divider_y)) {
                cell.set_symbol("─");
                cell.set_style(Style::default().fg(config.palette.overlay0));
            }
        }
        let lower_body_y = split_divider_y + 1;
        let lower_rows = area.bottom().saturating_sub(lower_body_y).saturating_sub(1);
        if let Some(ctx) = context {
            dock_state.files_tab.refresh_if_needed(ctx.root_path);
        }
        render_files_content(
            buffer,
            content_x,
            lower_body_y,
            lower_rows,
            area.width,
            config,
            &mut dock_state.files_tab,
            hits,
        );
    } else {
        match dock_state.active_tab {
            DockTabId::Status => {
                if let Some(ctx) = context {
                    dock_state.status_tab.refresh_if_needed(ctx.root_path, ctx.branch);
                    dock_state.status_tab.sync_agent_telemetry(ctx.agent_tokens);
                }
                render_status_content(
                    buffer,
                    content_x,
                    body_y,
                    available_rows,
                    area.width,
                    config,
                    context,
                    &mut dock_state.status_tab,
                    Some(hits),
                );
            }
            DockTabId::Files => {
                if let Some(ctx) = context {
                    dock_state.files_tab.refresh_if_needed(ctx.root_path);
                }
                render_files_content(
                    buffer,
                    content_x,
                    body_y,
                    available_rows,
                    area.width,
                    config,
                    &mut dock_state.files_tab,
                    hits,
                );
            }
            DockTabId::GitFlow => {
                if let Some(ctx) = context {
                    dock_state.git_flow_tab.refresh_if_needed(
                        ctx.root_path,
                        ctx.branch,
                        ctx.git_ahead_behind,
                        &ctx.siblings,
                    );
                }
                render_git_flow_content(
                    buffer,
                    content_x,
                    body_y,
                    available_rows,
                    area.width,
                    config,
                    context,
                    &mut dock_state.git_flow_tab,
                );
            }
        }
    }

    let toggle_x = area.right().saturating_sub(2);
    let toggle_y = area.bottom().saturating_sub(1);
    if area.width > 1 && area.height > 0 {
        buffer.set_string(toggle_x, toggle_y, "»", Style::default().fg(config.palette.subtext0));
        hits.right_dock_toggle = Rect::new(toggle_x, toggle_y, 1, 1);
    }

    // Si hay un Quick Peek activo, renderizar el modal flotante (Punto 6)
    if let Some(peek) = &dock_state.quick_peek {
        render_quick_peek_modal(buffer, area, config, peek);
    }
}

fn render_panel_header_rail(
    buffer: &mut Buffer,
    x: u16,
    y: u16,
    width: u16,
    title: &str,
    right_tag: Option<&str>,
    is_open: bool,
    header_style: Style,
    tag_style: Style,
    hits: Option<&mut super::state::ShellHitMap>,
    section_id: Option<u8>,
) {
    if width < 6 || y >= buffer.area.bottom() || x >= buffer.area.right() {
        return;
    }
    let glyph = if is_open { "▾" } else { "▸" };
    let header_text = format!("{glyph} {title}");
    buffer.set_string(x, y, &header_text, header_style);

    let tag_len = right_tag.map(|t| t.chars().count() as u16).unwrap_or(0);
    if let Some(tag) = right_tag {
        let tag_x = x + width.saturating_sub(tag_len);
        if tag_x > x + (header_text.chars().count() as u16) {
            buffer.set_stringn(tag_x, y, tag, tag_len as usize, tag_style);
        }
    }

    if let (Some(h), Some(sec)) = (hits, section_id) {
        h.right_dock_section_toggles
            .push((Rect::new(x, y, width, 1), sec));
    }
}

fn render_panel_kv_rail(
    buffer: &mut Buffer,
    x: u16,
    y: u16,
    width: u16,
    key: &str,
    val_left: &str,
    val_right: Option<&str>,
    key_style: Style,
    val_left_style: Style,
    val_right_style: Style,
) {
    if width < 8 || y >= buffer.area.bottom() || x >= buffer.area.right() {
        return;
    }
    let key_w = key.chars().count() as u16;
    buffer.set_stringn(x, y, key, key_w as usize, key_style);

    let left_x = x + key_w;
    let right_len = val_right.map(|r| r.chars().count() as u16).unwrap_or(0);
    let right_x = x + width.saturating_sub(right_len);

    let max_left = if right_len > 0 && right_x > left_x {
        (right_x.saturating_sub(left_x + 1)) as usize
    } else {
        (width.saturating_sub(key_w)) as usize
    };

    buffer.set_stringn(left_x, y, val_left, max_left, val_left_style);

    if let Some(right) = val_right {
        if right_x > left_x + (val_left.chars().count() as u16) {
            buffer.set_stringn(right_x, y, right, right_len as usize, val_right_style);
        }
    }
}

fn render_status_content(
    buffer: &mut Buffer,
    x: u16,
    y: u16,
    rows: u16,
    width: u16,
    config: &ClientShellConfig,
    context: Option<&WorkspaceContext>,
    status_tab: &mut StatusTab,
    mut hits: Option<&mut super::state::ShellHitMap>,
) {
    if rows < 4 || width < 12 {
        return;
    }

    let ws_name = context.map(|c| c.workspace_name).unwrap_or("unavailable");
    let is_preview = context.map(|c| c.is_preview).unwrap_or(false);
    let branch_name = context.and_then(|c| c.branch).unwrap_or("unavailable");

    let header_style = Style::default()
        .fg(config.palette.text)
        .add_modifier(ratatui::style::Modifier::BOLD);
    let tag_style = Style::default().fg(config.palette.overlay0);
    let key_style = Style::default().fg(config.palette.overlay0);
    let val_accent = Style::default().fg(config.palette.accent);
    let val_text = Style::default().fg(config.palette.text);
    let val_dim = Style::default().fg(config.palette.overlay0);

    let rail_w = width.saturating_sub(1);
    let mut cur_y = y;
    let bottom_limit = y + rows;

    // 0. Workspace info & Modo preview
    if cur_y < bottom_limit {
        let ws_style = Style::default().fg(config.palette.accent);
        let preview_tag = if is_preview { " [Preview]" } else { "" };
        let ws_disp = format!("● {ws_name}{preview_tag}");
        buffer.set_stringn(
            x + 1,
            cur_y,
            ws_disp,
            rail_w as usize,
            ws_style,
        );
        cur_y += 1;
    }

    // 1. SECCIÓN: TODOS (Sección 1)
    if cur_y < bottom_limit {
        let is_col = status_tab.is_collapsed(1);
        let done_count = status_tab.telemetry.todos.iter().filter(|t| t.is_done).count();
        let total_count = status_tab.telemetry.todos.len();
        let count_tag = if total_count == 0 {
            "no todos".to_string()
        } else {
            format!("{done_count}/{total_count}")
        };

        render_panel_header_rail(
            buffer,
            x + 1,
            cur_y,
            rail_w,
            "TODOS",
            Some(&count_tag),
            !is_col,
            header_style,
            tag_style,
            hits.as_deref_mut(),
            Some(1),
        );
        cur_y += 1;

        if !is_col {
            if status_tab.telemetry.todos.is_empty() {
                if cur_y < bottom_limit {
                    render_panel_kv_rail(
                        buffer,
                        x + 2,
                        cur_y,
                        rail_w.saturating_sub(2),
                        "",
                        "no active todos",
                        Some("/gentle:todo"),
                        key_style,
                        val_dim,
                        Style::default().fg(config.palette.overlay0),
                    );
                    cur_y += 1;
                }
            } else {
                for todo in &status_tab.telemetry.todos {
                    if cur_y >= bottom_limit.saturating_sub(1) {
                        break;
                    }
                    let bullet = if todo.is_in_progress {
                        "• "
                    } else if todo.is_done {
                        "✓ "
                    } else {
                        "○ "
                    };
                    let bullet_style = if todo.is_in_progress {
                        Style::default().fg(config.palette.mauve)
                    } else if todo.is_done {
                        Style::default().fg(config.palette.green)
                    } else {
                        Style::default().fg(config.palette.overlay0)
                    };
                    let text_style = if todo.is_in_progress {
                        Style::default().fg(config.palette.mauve)
                    } else if todo.is_done {
                        Style::default().fg(config.palette.subtext0)
                    } else {
                        val_text
                    };

                    buffer.set_string(x + 2, cur_y, bullet, bullet_style);
                    let title_x = x + 4;
                    let max_w = (rail_w.saturating_sub(4)) as usize;
                    buffer.set_stringn(title_x, cur_y, &todo.title, max_w, text_style);
                    cur_y += 1;
                }
            }
            cur_y += 1; // Espacio vertical entre bloques
        }
    }

    // 2. SECCIÓN: AGENTS (Sección 2)
    if cur_y < bottom_limit {
        let is_col = status_tab.is_collapsed(2);
        let agent_st = context
            .and_then(|c| c.agent_status)
            .filter(|s| *s != crate::api::schema::AgentStatus::Unknown)
            .map(super::status_text)
            .unwrap_or("idle");

        let agents_tag = agent_st.to_string();

        render_panel_header_rail(
            buffer,
            x + 1,
            cur_y,
            rail_w,
            "AGENTS",
            Some(&agents_tag),
            !is_col,
            header_style,
            tag_style,
            hits.as_deref_mut(),
            Some(2),
        );
        cur_y += 1;

        if !is_col {
            let agent_disp = context
                .and_then(|c| c.agent_name)
                .unwrap_or("pi");

            if agent_st == "working" {
                if cur_y < bottom_limit {
                    render_panel_kv_rail(
                        buffer,
                        x + 2,
                        cur_y,
                        rail_w.saturating_sub(2),
                        "• ",
                        agent_disp,
                        Some("working"),
                        Style::default().fg(config.palette.mauve),
                        Style::default().fg(config.palette.mauve),
                        val_dim,
                    );
                    cur_y += 1;
                }
            } else if agent_st == "blocked" || agent_st == "waiting" {
                if cur_y < bottom_limit {
                    render_panel_kv_rail(
                        buffer,
                        x + 2,
                        cur_y,
                        rail_w.saturating_sub(2),
                        "▲ ",
                        agent_disp,
                        Some(agent_st),
                        Style::default().fg(config.palette.peach),
                        Style::default().fg(config.palette.peach),
                        val_dim,
                    );
                    cur_y += 1;
                }
            } else {
                if cur_y < bottom_limit {
                    render_panel_kv_rail(
                        buffer,
                        x + 2,
                        cur_y,
                        rail_w.saturating_sub(2),
                        "",
                        "none running",
                        Some("/gentle:agents"),
                        key_style,
                        val_dim,
                        Style::default().fg(config.palette.overlay0),
                    );
                    cur_y += 1;
                }
            }
            cur_y += 1;
        }
    }

    // 3. SECCIÓN: CHANGES (Sección 3)
    if config.right_dock_show_git && cur_y < bottom_limit {
        let is_col = status_tab.is_collapsed(3);
        let files_changed = status_tab.telemetry.git.agent_files_changed;
        let ahead_behind = context.and_then(|c| c.git_ahead_behind);
        
        let changes_tag = if branch_name != "unavailable" {
            let ab_str = match ahead_behind {
                Some((ahead, behind)) if ahead > 0 && behind > 0 => format!(" · ↑{ahead} ↓{behind}"),
                Some((ahead, 0)) if ahead > 0 => format!(" · ↑{ahead}"),
                Some((0, behind)) if behind > 0 => format!(" · ↓{behind}"),
                _ => String::new(),
            };
            if files_changed == 0 {
                format!("{branch_name}{ab_str} · clean")
            } else {
                format!("{branch_name}{ab_str} · {files_changed} files")
            }
        } else if files_changed == 0 {
            "clean".to_string()
        } else {
            format!("{files_changed} files")
        };

        render_panel_header_rail(
            buffer,
            x + 1,
            cur_y,
            rail_w,
            "CHANGES",
            Some(&changes_tag),
            !is_col,
            header_style,
            tag_style,
            hits.as_deref_mut(),
            Some(3),
        );
        cur_y += 1;

        if !is_col {
            if cur_y < bottom_limit {
                if files_changed == 0 {
                    let clean_label = match ahead_behind {
                        Some((ahead, behind)) if ahead > 0 || behind > 0 => {
                            let mut parts = Vec::new();
                            if ahead > 0 {
                                parts.push(format!("{ahead} ahead"));
                            }
                            if behind > 0 {
                                parts.push(format!("{behind} behind"));
                            }
                            format!("clean · {}", parts.join(", "))
                        }
                        _ => "working tree clean".to_string(),
                    };
                    render_panel_kv_rail(
                        buffer,
                        x + 2,
                        cur_y,
                        rail_w.saturating_sub(2),
                        "",
                        &clean_label,
                        Some("/gentle:changes"),
                        key_style,
                        val_dim,
                        Style::default().fg(config.palette.overlay0),
                    );
                } else {
                    let diff_str = format!("+{} lines · {} files", status_tab.telemetry.git.agent_lines_added, files_changed);
                    render_panel_kv_rail(
                        buffer,
                        x + 2,
                        cur_y,
                        rail_w.saturating_sub(2),
                        "",
                        &diff_str,
                        Some("/gentle:changes"),
                        key_style,
                        Style::default().fg(config.palette.green),
                        Style::default().fg(config.palette.overlay0),
                    );
                }
                cur_y += 1;
            }
            cur_y += 1;
        }
    }

    // 4. SECCIÓN: STATS (Sección 4)
    if cur_y < bottom_limit {
        let is_col = status_tab.is_collapsed(4);
        let stats = &status_tab.telemetry.live_stats;
        let has_live_stats = stats.turns > 0 || stats.spent_cost > 0.0 || stats.tokens_in > 0;
        let cost_tag = if has_live_stats {
            format!("${:.3} sub", stats.spent_cost)
        } else {
            "no telemetry".to_string()
        };

        render_panel_header_rail(
            buffer,
            x + 1,
            cur_y,
            rail_w,
            "STATS",
            Some(&cost_tag),
            !is_col,
            header_style,
            tag_style,
            hits.as_deref_mut(),
            Some(4),
        );
        cur_y += 1;

        if !is_col {
            if !has_live_stats {
                if cur_y < bottom_limit {
                    render_panel_kv_rail(
                        buffer,
                        x + 2,
                        cur_y,
                        rail_w.saturating_sub(2),
                        "",
                        "not reported",
                        Some("/gentle:stats"),
                        key_style,
                        val_dim,
                        Style::default().fg(config.palette.overlay0),
                    );
                    cur_y += 1;
                }
            } else {
                // spent
                if cur_y < bottom_limit {
                    let spent_str = format!("${:.3} sub", stats.spent_cost);
                    let right_turns = if stats.elapsed_str.is_empty() {
                        format!("{} turns", stats.turns)
                    } else {
                        format!("{} turns · {}", stats.turns, stats.elapsed_str)
                    };
                    render_panel_kv_rail(
                        buffer,
                        x + 2,
                        cur_y,
                        rail_w.saturating_sub(2),
                        "spent   ",
                        &spent_str,
                        Some(&right_turns),
                        key_style,
                        val_text,
                        val_dim,
                    );
                    cur_y += 1;
                }

                // context (si fue reportado activamente por la sesión)
                if status_tab.telemetry.context.limit_tokens > 0 && cur_y < bottom_limit {
                    let ctx_pct = status_tab.telemetry.context.percent as u32;
                    let ctx_str = format!(
                        "{}% · {}k/{}k",
                        ctx_pct,
                        status_tab.telemetry.context.used_tokens / 1000,
                        status_tab.telemetry.context.limit_tokens / 1000
                    );
                    render_panel_kv_rail(
                        buffer,
                        x + 2,
                        cur_y,
                        rail_w.saturating_sub(2),
                        "context ",
                        &ctx_str,
                        None,
                        key_style,
                        val_text,
                        val_dim,
                    );
                    cur_y += 1;
                }

                // tokens
                if cur_y < bottom_limit {
                    let in_k = format!("in {}k", stats.tokens_in / 1000);
                    let out_k = format!("out {:.1}k", stats.tokens_out as f32 / 1000.0);
                    let tok_str = format!("{in_k} · {out_k}");
                    render_panel_kv_rail(
                        buffer,
                        x + 2,
                        cur_y,
                        rail_w.saturating_sub(2),
                        "tokens  ",
                        &tok_str,
                        None,
                        key_style,
                        val_text,
                        val_dim,
                    );
                    cur_y += 1;
                }

                // cache (sólo si fue reportado > 0)
                if stats.cache_read_tokens > 0 && cur_y < bottom_limit {
                    let cache_str = format!("{}% hit · {}k read", stats.cache_hit_percent, stats.cache_read_tokens / 1000);
                    render_panel_kv_rail(
                        buffer,
                        x + 2,
                        cur_y,
                        rail_w.saturating_sub(2),
                        "cache   ",
                        &cache_str,
                        None,
                        key_style,
                        val_text,
                        val_dim,
                    );
                    cur_y += 1;
                }

                // tools (sólo si hubo calls reportadas)
                if stats.tool_calls > 0 && cur_y < bottom_limit {
                    let tools_str = if stats.tool_failures > 0 {
                        format!("{} calls · {} failed", stats.tool_calls, stats.tool_failures)
                    } else {
                        format!("{} calls", stats.tool_calls)
                    };
                    let tool_val_style = if stats.tool_failures > 0 {
                        Style::default().fg(config.palette.yellow)
                    } else {
                        val_text
                    };
                    render_panel_kv_rail(
                        buffer,
                        x + 2,
                        cur_y,
                        rail_w.saturating_sub(2),
                        "tools   ",
                        &tools_str,
                        None,
                        key_style,
                        tool_val_style,
                        val_dim,
                    );
                    cur_y += 1;
                }

                // agents / subagents (sólo si hubo subagents ejecutados)
                if stats.subagent_runs > 0 && cur_y < bottom_limit {
                    let ag_str = format!("{} runs · {}k · ${:.3} sub", stats.subagent_runs, stats.subagent_tokens / 1000, stats.subagent_cost);
                    render_panel_kv_rail(
                        buffer,
                        x + 2,
                        cur_y,
                        rail_w.saturating_sub(2),
                        "agents  ",
                        &ag_str,
                        None,
                        key_style,
                        val_text,
                        val_dim,
                    );
                    cur_y += 1;
                }

                // model breakdown
                if cur_y < bottom_limit && !stats.model_name.is_empty() {
                    let mod_desc = format!("{} turns · {}k · ${:.3} sub", stats.model_turns, stats.model_tokens / 1000, stats.model_cost);
                    render_panel_kv_rail(
                        buffer,
                        x + 2,
                        cur_y,
                        rail_w.saturating_sub(2),
                        "",
                        &stats.model_name,
                        Some(&mod_desc),
                        key_style,
                        val_accent,
                        val_dim,
                    );
                    cur_y += 1;
                }
            }

            cur_y += 1;
        }
    }

    // 5. SECCIÓN: INTEGRATIONS (Sección 5)
    if cur_y < bottom_limit {
        let is_col = status_tab.is_collapsed(5);
        let mcp_count = status_tab.telemetry.mcp_servers.len();
        let accounts_count = status_tab.telemetry.accounts.len();
        let total_integrations = mcp_count + accounts_count;
        let int_count_str = if total_integrations == 0 {
            "0".to_string()
        } else {
            total_integrations.to_string()
        };

        render_panel_header_rail(
            buffer,
            x + 1,
            cur_y,
            rail_w,
            "INTEGRATIONS",
            Some(&int_count_str),
            !is_col,
            header_style,
            tag_style,
            hits.as_deref_mut(),
            Some(5),
        );
        cur_y += 1;

        if !is_col {
            let has_profile = !status_tab.telemetry.model_name.is_empty();
            let has_engram = status_tab.telemetry.engram.is_online || status_tab.telemetry.engram.observations_count > 0;
            let has_accounts = !status_tab.telemetry.accounts.is_empty();

            if has_profile && cur_y < bottom_limit {
                let profile_str = format!("Agent profile: {}", status_tab.telemetry.model_name);
                buffer.set_stringn(x + 2, cur_y, profile_str, rail_w.saturating_sub(2) as usize, val_dim);
                cur_y += 1;
            }

            if has_engram && cur_y < bottom_limit {
                let obs_id = status_tab.telemetry.engram.observations_count;
                let mem_str = format!("- 🧠 {ws_name} · ✓ saved #{obs_id}");
                buffer.set_stringn(
                    x + 2,
                    cur_y,
                    mem_str,
                    rail_w.saturating_sub(2) as usize,
                    Style::default().fg(config.palette.mauve),
                );
                cur_y += 1;
            }

            // CLIProxyAPI / CPAMC Accounts
            for acc in &status_tab.telemetry.accounts {
                if cur_y >= bottom_limit {
                    break;
                }
                let pct_style = if acc.percent < 15 {
                    Style::default().fg(config.palette.red)
                } else if acc.percent < 30 {
                    Style::default().fg(config.palette.yellow)
                } else {
                    Style::default().fg(config.palette.green)
                };
                render_panel_kv_rail(
                    buffer,
                    x + 2,
                    cur_y,
                    rail_w.saturating_sub(2),
                    acc.role_symbol,
                    &acc.name,
                    Some(&acc.quota_desc),
                    pct_style,
                    val_text,
                    val_dim,
                );
                cur_y += 1;
            }

            // Model Quotas
            for mq in &status_tab.telemetry.model_quotas {
                if cur_y >= bottom_limit {
                    break;
                }
                let desc = format!("{} · {}", mq.indicator, mq.reset_in);
                render_panel_kv_rail(
                    buffer,
                    x + 2,
                    cur_y,
                    rail_w.saturating_sub(2),
                    "◆ ",
                    &mq.model_name,
                    Some(&desc),
                    Style::default().fg(config.palette.blue),
                    val_text,
                    val_dim,
                );
                cur_y += 1;
            }

            if mcp_count == 0 && !has_profile && !has_engram && !has_accounts {
                if cur_y < bottom_limit {
                    render_panel_kv_rail(
                        buffer,
                        x + 2,
                        cur_y,
                        rail_w.saturating_sub(2),
                        "",
                        "none connected",
                        Some("/gentle:mcp"),
                        key_style,
                        val_dim,
                        Style::default().fg(config.palette.overlay0),
                    );
                    cur_y += 1;
                }
            } else {
                for mcp in &status_tab.telemetry.mcp_servers {
                    if cur_y >= bottom_limit {
                        break;
                    }
                    let mcp_style = if mcp.is_online {
                        Style::default().fg(config.palette.green)
                    } else {
                        val_dim
                    };
                    render_panel_kv_rail(
                        buffer,
                        x + 2,
                        cur_y,
                        rail_w.saturating_sub(2),
                        "• ",
                        &mcp.name,
                        Some(mcp.status),
                        mcp_style,
                        val_text,
                        val_dim,
                    );
                    cur_y += 1;
                }
            }

            cur_y += 1;
        }
    }

    // 6. FOOTER CANÓNICO GENTLE SHELL
    if bottom_limit.saturating_sub(cur_y) >= 1 {
        let footer_y = bottom_limit.saturating_sub(1);
        buffer.set_stringn(
            x + 1,
            footer_y,
            "ctrl+shift+t todos · alt+u usage · / commands",
            rail_w as usize,
            val_dim,
        );
    }
}

fn render_files_content(
    buffer: &mut Buffer,
    x: u16,
    y: u16,
    rows: u16,
    width: u16,
    config: &ClientShellConfig,
    files_tab: &mut FilesTab,
    hits: &mut super::state::ShellHitMap,
) {
    if rows < 2 || width < 12 {
        return;
    }

    let visible_rows = (rows.saturating_sub(2)) as usize;
    files_tab.set_visible_rows(visible_rows);

    let count_text = if files_tab.is_searching {
        let q = files_tab.search_query.as_deref().unwrap_or("");
        format!("🔍 /{q}_ [Esc:Cancelar]")
    } else if let Some(q) = &files_tab.search_query {
        format!("🔍 /{q} [Esc:Limpiar]")
    } else if !files_tab.entries.is_empty() {
        let hidden_tag = if files_tab.show_hidden {
            "[.:All]"
        } else {
            "[.:NoDot]"
        };
        format!(
            "• EXPLORER ({}/{}) [w:Space Space:Peek] {}",
            files_tab.selected_index + 1,
            files_tab.entries.len(),
            hidden_tag
        )
    } else if files_tab.cached_root.is_none() {
        "• EXPLORER [Cargando...]".to_string()
    } else {
        "• EXPLORER [Empty]".to_string()
    };
    let title_style = if files_tab.is_searching {
        Style::default().fg(config.palette.yellow)
    } else {
        Style::default().fg(config.palette.accent)
    };
    buffer.set_string(x + 1, y, count_text, title_style);

    let indices = files_tab.filtered_indices();
    if indices.is_empty() {
        if files_tab.cached_root.is_none() {
            buffer.set_string(
                x + 1,
                y + 2,
                "Cargando directorio...",
                Style::default().fg(config.palette.subtext0),
            );
        } else if files_tab.search_query.is_some() {
            buffer.set_string(
                x + 1,
                y + 2,
                "(no matches)",
                Style::default().fg(config.palette.subtext0),
            );
        } else {
            let msg = if let Some(root) = &files_tab.cached_root {
                if !root.exists() {
                    "(inaccessible or disconnected)"
                } else {
                    "(no files)"
                }
            } else {
                "(no files)"
            };
            buffer.set_string(
                x + 1,
                y + 2,
                msg,
                Style::default().fg(config.palette.subtext0),
            );
        }
        return;
    }

    let start = files_tab.scroll_offset.min(indices.len());
    for (display_idx, (real_pos, &real_idx)) in indices
        .iter()
        .enumerate()
        .skip(start)
        .take(visible_rows)
        .enumerate()
    {
        let entry = &files_tab.entries[real_idx];
        let row_y = y + 2 + (display_idx as u16);
        let row_rect = Rect::new(x, row_y, width, 1);
        hits.right_dock_file_rows.push((row_rect, real_idx));

        let is_selected = real_pos == files_tab.selected_index;
        if is_selected {
            buffer.set_style(row_rect, Style::default().bg(config.palette.surface0));
        }

        let max_right = x + width.saturating_sub(1);
        let mut curr_x = x + 1;

        // 1. Indentación limpia (2 espacios por nivel)
        let indent_cols = (entry.depth as u16) * 2;
        curr_x = curr_x.saturating_add(indent_cols);

        if curr_x >= max_right {
            continue;
        }

        // 2. Prefijo: Chevron minimalista para carpetas, icono Seti para archivos
        let (text_start_x, name_style) = if entry.is_dir {
            let chevron = if entry.is_expanded { "∨" } else { "›" };
            let chevron_style = if is_selected {
                Style::default().fg(config.palette.accent)
            } else {
                Style::default().fg(config.palette.subtext0)
            };
            buffer.set_string(curr_x, row_y, chevron, chevron_style);

            let name_style = if is_selected {
                Style::default().fg(config.palette.accent)
            } else if entry.name.starts_with('.') {
                Style::default().fg(config.palette.subtext0)
            } else {
                Style::default().fg(config.palette.text)
            };
            (curr_x + 2, name_style)
        } else {
            let icon = resolve_file_icon(&entry.name);
            let icon_style = Style::default().fg(icon.color);
            buffer.set_string(curr_x, row_y, icon.glyph, icon_style);
            let icon_w = unicode_width::UnicodeWidthStr::width(icon.glyph) as u16;

            let name_style = if is_selected {
                Style::default().fg(config.palette.accent)
            } else if entry.name.starts_with('.') {
                Style::default().fg(config.palette.subtext0)
            } else {
                Style::default().fg(config.palette.text)
            };
            (curr_x + icon_w + 1, name_style)
        };

        // 3. Nombre del archivo/directorio con truncamiento seguro
        if text_start_x < max_right {
            let available_chars = (max_right - text_start_x) as usize;
            let name_chars_count = entry.name.chars().count();
            if name_chars_count > available_chars {
                let limit = available_chars.saturating_sub(1);
                let truncated: String = entry.name.chars().take(limit).collect();
                buffer.set_string(text_start_x, row_y, format!("{truncated}…"), name_style);
            } else {
                buffer.set_string(text_start_x, row_y, &entry.name, name_style);
            }
        }
    }
}

fn render_git_flow_content(
    buffer: &mut Buffer,
    x: u16,
    y: u16,
    rows: u16,
    width: u16,
    config: &ClientShellConfig,
    _context: Option<&WorkspaceContext>,
    git_flow_tab: &mut GitFlowTab,
) {
    if rows < 3 || width < 15 || y >= buffer.area.bottom() || x >= buffer.area.right() {
        return;
    }

    let actual_w = width.min(buffer.area.right().saturating_sub(x));
    let bottom_limit = y + rows;

    // 1. Cabecera superior fija: "◆ GIT FLOW & WORKTREE MAP (LIVE)"
    let title = "◆ GIT FLOW & WORKTREE MAP (LIVE)";
    let title_style = Style::default()
        .fg(config.palette.blue)
        .add_modifier(ratatui::style::Modifier::BOLD);
    buffer.set_stringn(x + 1, y, title, actual_w.saturating_sub(2) as usize, title_style);

    let top_y = y + 1;
    if top_y >= bottom_limit {
        return;
    }

    // TOP indicator
    let base_br = &git_flow_tab.canvas.base_branch;
    buffer.set_string(x + 1, top_y, &format!("TOP (Base {base_br})"), Style::default().fg(config.palette.overlay0));
    if actual_w > 8 {
        buffer.set_string(x + actual_w.saturating_sub(6), top_y, "▲ OOT", Style::default().fg(config.palette.blue));
    }

    // Preparar líneas visuales del riel continuo
    let mut visual_lines: Vec<(String, Style)> = Vec::new();

    // 0. Radar de Conflicto Cruzado en Caliente (si dos agentes tocan el mismo archivo)
    if !git_flow_tab.canvas.cross_session_conflicts.is_empty() {
        let files_str = git_flow_tab.canvas.cross_session_conflicts.join(", ");
        visual_lines.push((
            format!("⚠️ ALERTA: Conflicto en caliente ({files_str})"),
            Style::default().fg(config.palette.red).add_modifier(ratatui::style::Modifier::BOLD),
        ));
        visual_lines.push(("             │".to_string(), Style::default().fg(config.palette.red)));
    }

    // 1. Render de la Sesión Activa Enfocada (Detalle completo: Commits + Sección en Trabajo)
    if let Some(active_sess) = &git_flow_tab.canvas.active_session {
        let agent_label = active_sess.agent_name.as_deref().unwrap_or("pi");
        let branch_name = &active_sess.branch;

        visual_lines.push((
            format!("├───╮ 🌲 WORKTREE [{branch_name}]"),
            Style::default().fg(config.palette.peach).add_modifier(ratatui::style::Modifier::BOLD),
        ));
        visual_lines.push((
            format!("│   ├─▶ [SESIÓN ACTIVA: {agent_label}]"),
            Style::default().fg(config.palette.blue).add_modifier(ratatui::style::Modifier::BOLD),
        ));

        // Commits guardados recientes
        for (sha, msg) in &active_sess.last_commits {
            let msg_short = if msg.len() > 22 {
                format!("{}…", &msg[..21])
            } else {
                msg.clone()
            };
            visual_lines.push((
                format!("│   ●  {sha} [COMMIT] {msg_short}"),
                Style::default().fg(config.palette.blue),
            ));
        }

        // Archivos congelados en commit anterior
        if !active_sess.committed_files.is_empty() {
            let cf_str = active_sess.committed_files.iter().take(3).cloned().collect::<Vec<_>>().join(", ");
            visual_lines.push((
                format!("│       └─📂 {cf_str}"),
                Style::default().fg(config.palette.overlay0),
            ));
        }

        // Sección EN TRABAJO EN VIVO (archivos modificados ahora mismo)
        if !active_sess.working_files.is_empty() {
            let wf_str = active_sess.working_files.iter().take(3).cloned().collect::<Vec<_>>().join(", ");
            let count = active_sess.working_files.len();
            visual_lines.push((
                format!("│   ⚡ [EN TRABAJO VIVO] ({count} mod)"),
                Style::default().fg(config.palette.peach).add_modifier(ratatui::style::Modifier::BOLD),
            ));
            visual_lines.push((
                format!("│       └─✏️ {wf_str}"),
                Style::default().fg(config.palette.peach),
            ));
        } else {
            visual_lines.push((
                "│   ✓ [working tree clean]".to_string(),
                Style::default().fg(config.palette.green),
            ));
        }

        // Estado de PR asociado si existe
        if let Some((pr_num, pr_title, pr_app)) = &active_sess.pr_info {
            let app_str = if *pr_app { "Approved" } else { "Review" };
            visual_lines.push((
                format!("│   └───> ↗ PR #{pr_num} ({pr_title}) [{app_str}]"),
                Style::default().fg(config.palette.green),
            ));
        }

        visual_lines.push(("│   │".to_string(), Style::default().fg(config.palette.overlay0)));
    }

    // 2. Render de Sesiones Secundarias / Worktrees Hermanos (Doble Carril en Paralelo)
    for sibling in &git_flow_tab.canvas.sibling_sessions {
        let agent_label = sibling.agent_name.as_deref().unwrap_or("pi");
        let branch_name = &sibling.branch;
        let st_icon = match sibling.agent_status {
            Some(crate::api::schema::AgentStatus::Working) => "🟠 working",
            Some(crate::api::schema::AgentStatus::Blocked) => "🔴 blocked",
            _ => "🟢 idle",
        };

        visual_lines.push((
            format!("├───┬─▶ [SESIÓN HERMANA: {agent_label}] {st_icon}"),
            Style::default().fg(config.palette.teal).add_modifier(ratatui::style::Modifier::BOLD),
        ));
        visual_lines.push((
            format!("│   │   Rama: {branch_name}"),
            Style::default().fg(config.palette.subtext0),
        ));

        if !sibling.working_files.is_empty() {
            let files = sibling.working_files.iter().take(2).cloned().collect::<Vec<_>>().join(", ");
            visual_lines.push((
                format!("│   │   ⚡ Editando: {files}"),
                Style::default().fg(config.palette.peach),
            ));
        }
        visual_lines.push(("│   │".to_string(), Style::default().fg(config.palette.overlay0)));
    }

    // 3. Retorno proyectado hacia main
    visual_lines.push((
        format!("├───◀─── Retorno proyectado hacia {base_br} (git merge)"),
        Style::default().fg(config.palette.overlay0),
    ));
    visual_lines.push(("│".to_string(), Style::default().fg(config.palette.overlay0)));

    // Viewport de renderizado
    let viewport_y = top_y + 1;
    let available_content_rows = bottom_limit.saturating_sub(viewport_y + 1) as usize;
    let offset = git_flow_tab.canvas.scroll_offset.min(visual_lines.len().saturating_sub(1));

    for (idx, (line, style)) in visual_lines.iter().skip(offset).take(available_content_rows).enumerate() {
        let line_y = viewport_y + idx as u16;
        if line_y >= bottom_limit.saturating_sub(1) {
            break;
        }
        buffer.set_stringn(x + 1, line_y, line, actual_w.saturating_sub(2) as usize, *style);
    }

    // BOTTOM indicator y scroll thumb
    let bottom_row = bottom_limit.saturating_sub(1);
    if bottom_row > viewport_y {
        buffer.set_string(x + 1, bottom_row, "BOTTOM", Style::default().fg(config.palette.overlay0));
        if actual_w > 4 {
            buffer.set_string(x + actual_w.saturating_sub(2), bottom_row, "▼", Style::default().fg(config.palette.blue));
        }
    }

    // Scrollbar lateral thumb en la columna derecha
    if visual_lines.len() > available_content_rows && available_content_rows > 0 {
        let thumb_col = x + actual_w.saturating_sub(1);
        let progress = (offset * available_content_rows) / visual_lines.len();
        let thumb_y = viewport_y + progress as u16;
        if thumb_y < bottom_row {
            if let Some(cell) = buffer.cell_mut((thumb_col, thumb_y)) {
                cell.set_symbol("█").set_style(Style::default().fg(config.palette.overlay1));
            }
        }
    }
}

/// Renderizado del Quick Peek Modal centrado y no intrusivo (Punto 6)
fn render_quick_peek_modal(
    buffer: &mut Buffer,
    area: Rect,
    config: &ClientShellConfig,
    peek: &QuickPeekState,
) {
    let modal_w = (area.width.saturating_sub(4)).max(20);
    let modal_h = (area.height.saturating_sub(6)).max(8);
    let modal_x = area.x + (area.width.saturating_sub(modal_w)) / 2;
    let modal_y = area.y + (area.height.saturating_sub(modal_h)) / 2;
    let modal_rect = Rect::new(modal_x, modal_y, modal_w, modal_h);

    // Fondo del modal con borde
    buffer.set_style(modal_rect, Style::default().bg(config.palette.surface0));

    // Borde perimetral
    for x in modal_rect.x..modal_rect.right() {
        if let Some(cell) = buffer.cell_mut((x, modal_rect.y)) {
            cell.set_symbol("─");
        }
        if let Some(cell) = buffer.cell_mut((x, modal_rect.bottom().saturating_sub(1))) {
            cell.set_symbol("─");
        }
    }
    for y in modal_rect.y..modal_rect.bottom() {
        if let Some(cell) = buffer.cell_mut((modal_rect.x, y)) {
            cell.set_symbol("│");
        }
        if let Some(cell) = buffer.cell_mut((modal_rect.right().saturating_sub(1), y)) {
            cell.set_symbol("│");
        }
    }

    // Título y hint de escape
    let title_text = format!(" 👁 {} [Esc=Cerrar] ", peek.title);
    buffer.set_string(
        modal_rect.x + 2,
        modal_rect.y,
        title_text,
        Style::default()
            .fg(config.palette.accent)
            .bg(config.palette.surface0),
    );

    // Contenido del archivo/log
    let content_area_rows = modal_h.saturating_sub(2) as usize;
    for (idx, line) in peek
        .lines
        .iter()
        .skip(peek.scroll)
        .take(content_area_rows)
        .enumerate()
    {
        let line_y = modal_rect.y + 1 + (idx as u16);
        let display_line = format!("{:3} │ {}", idx + 1 + peek.scroll, line);
        let max_len = (modal_w as usize).saturating_sub(4);
        let truncated = if display_line.len() > max_len {
            &display_line[..max_len]
        } else {
            &display_line
        };
        buffer.set_string(
            modal_rect.x + 2,
            line_y,
            truncated,
            Style::default().fg(config.palette.text),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TreeFixture(PathBuf);

    impl TreeFixture {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "herdr-dock-tree-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            std::fs::create_dir(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TreeFixture {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }

    #[test]
    fn tree_expansion_dotfiles_sorting_and_nested_preview() {
        let root = TreeFixture::new();
        for dir in [".config", ".git", "z-dir"] {
            std::fs::create_dir(root.0.join(dir)).unwrap();
        }
        for file in [".bashrc", "a.txt", ".config/nested.txt"] {
            std::fs::write(root.0.join(file), "hello\nworld").unwrap();
        }
        let mut dock = RightDockState::default();
        dock.files_tab.refresh_if_needed(&root.0);
        assert_eq!(
            dock.files_tab
                .entries
                .iter()
                .map(|e| e.name.as_str())
                .collect::<Vec<_>>(),
            [".config", "z-dir", ".bashrc", "a.txt"]
        );
        dock.files_tab.set_selected_expanded(true);
        assert!(dock.files_tab.entries[0].is_expanded);
        assert_eq!(
            dock.files_tab.entries[1].relative_path,
            PathBuf::from(".config/nested.txt")
        );
        assert_eq!(dock.files_tab.entries[1].depth, 1);
        dock.files_tab.selected_index = 1;
        dock.activate_file_selection();
        assert_eq!(dock.quick_peek.as_ref().unwrap().lines, ["hello", "world"]);
        dock.files_tab.selected_index = 0;
        dock.files_tab.set_selected_expanded(false);
        assert_eq!(dock.files_tab.entries.len(), 4);
        dock.files_tab.refresh_if_needed(&root.0);
        assert!(!dock.files_tab.entries[0].is_expanded);
    }

    #[test]
    fn tree_navigation_scrolls_and_root_change_resets_tree() {
        let root = TreeFixture::new();
        for i in 0..40 {
            std::fs::write(root.0.join(format!("file-{i:02}")), "").unwrap();
        }
        let mut files = FilesTab::default();
        files.refresh_if_needed(&root.0);
        files.set_visible_rows(3);
        for _ in 0..39 {
            files.select_next();
        }
        assert_eq!(files.entries.len(), 40);
        assert_eq!(files.selected_index, 39);
        assert_eq!(files.scroll_offset, 37);
        files.select_next();
        assert_eq!(files.selected_index, 39);
        for _ in 0..40 {
            files.select_prev();
        }
        assert_eq!((files.selected_index, files.scroll_offset), (0, 0));
        let empty = TreeFixture::new();
        files.refresh_if_needed(&empty.0);
        assert!(files.entries.is_empty());
        assert!(files.expanded_dirs.is_empty());
        assert_eq!((files.selected_index, files.scroll_offset), (0, 0));
    }

    #[test]
    fn status_renders_rail_sections_and_footer() {
        let config = ClientShellConfig::from_config(&crate::config::Config::default());
        let area = Rect::new(0, 0, 45, 30);
        let mut buffer = Buffer::empty(area);
        let mut status_tab = StatusTab::default();
        let context = WorkspaceContext {
            workspace_id: "one",
            workspace_name: "telar",
            root_path: Path::new("/"),
            branch: Some("main"),
            git_ahead_behind: None,
            is_preview: false,
            agent_status: Some(crate::api::schema::AgentStatus::Working),
            agent_name: Some("pi"),
            agent_tokens: None,
            siblings: Vec::new(),
        };
        render_status_content(
            &mut buffer,
            0,
            0,
            30,
            45,
            &config,
            Some(&context),
            &mut status_tab,
            None,
        );
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(text.contains("TODOS"));
        assert!(text.contains("AGENTS"));
        assert!(text.contains("CHANGES"));
        assert!(text.contains("STATS"));
        assert!(text.contains("INTEGRATIONS"));
        assert!(text.contains("ctrl+shift+t todos"));
    }

    #[test]
    fn status_shows_workspace_agent_and_stats() {
        let config = ClientShellConfig::from_config(&crate::config::Config::default());
        let area = Rect::new(0, 0, 50, 30);
        let mut buffer = Buffer::empty(area);
        let context = WorkspaceContext {
            workspace_id: "one",
            workspace_name: "telar",
            root_path: Path::new("/"),
            branch: Some("main"),
            git_ahead_behind: None,
            is_preview: false,
            agent_status: Some(crate::api::schema::AgentStatus::Working),
            agent_name: Some("pi"),
            agent_tokens: None,
            siblings: Vec::new(),
        };
        let mut status_tab = StatusTab::default();
        render_status_content(
            &mut buffer,
            0,
            0,
            30,
            50,
            &config,
            Some(&context),
            &mut status_tab,
            None,
        );
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(text.contains("telar"));
        assert!(text.contains("pi"));
        assert!(text.contains("working"));
        assert!(text.contains("not reported"));
    }

    #[test]
    fn status_sections_can_be_hidden_independently() {
        let mut config = ClientShellConfig::from_config(&crate::config::Config::default());
        let area = Rect::new(0, 0, 50, 30);
        let context = WorkspaceContext {
            workspace_id: "one",
            workspace_name: "project-one",
            root_path: Path::new("/"),
            branch: Some("branch-one"),
            git_ahead_behind: None,
            is_preview: false,
            agent_status: None,
            agent_name: None,
            agent_tokens: None,
            siblings: Vec::new(),
        };
        config.right_dock_show_git = false;
        let mut buffer = Buffer::empty(area);
        let mut status_tab = StatusTab::default();
        render_status_content(
            &mut buffer,
            0,
            0,
            30,
            50,
            &config,
            Some(&context),
            &mut status_tab,
            None,
        );
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(!text.contains("CHANGES"));

        config.right_dock_show_git = true;
        let mut buffer = Buffer::empty(area);
        render_status_content(
            &mut buffer,
            0,
            0,
            30,
            50,
            &config,
            Some(&context),
            &mut status_tab,
            None,
        );
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(text.contains("CHANGES"));
    }

    #[test]
    fn test_status_tab_collapse_toggle_and_shortcuts() {
        let mut status = StatusTab::default();
        // Por defecto todas las secciones están expandidas en el rail Gentle Shell
        assert!(!status.is_collapsed(1));
        assert!(!status.is_collapsed(3));
        assert!(!status.is_collapsed(5));

        // Alternar colapso de sección 1
        status.toggle_section(1);
        assert!(status.is_collapsed(1));
        status.toggle_section(1);
        assert!(!status.is_collapsed(1));

        // Alternar colapso de sección 3
        status.toggle_section(3);
        assert!(status.is_collapsed(3));

        // Colapsar todo
        status.collapse_all();
        for s in 1..=5 {
            assert!(status.is_collapsed(s));
        }

        // Expandir todo
        status.expand_all();
        for s in 1..=5 {
            assert!(!status.is_collapsed(s));
        }
    }

    #[test]
    fn test_dock_tab_lifecycle_hibernation_and_dirty() {
        let mut dock = RightDockState::default();
        assert_eq!(dock.active_tab, DockTabId::Files);

        // Al cambiar a Status, Status pasa a dirty
        dock.select_tab(DockTabId::Status);
        assert_eq!(dock.active_tab, DockTabId::Status);
        assert!(dock.status_tab.is_dirty());

        // Al cambiar a GitFlow, Status se hiberna y GitFlow pasa a dirty
        dock.select_tab(DockTabId::GitFlow);
        assert_eq!(dock.active_tab, DockTabId::GitFlow);
        assert!(dock.git_flow_tab.is_dirty());
    }

    #[test]
    fn test_workspace_synchronization_invalidates_caches() {
        let mut dock = RightDockState::default();
        dock.quick_peek = Some(QuickPeekState::new("test.txt", vec!["hello".into()]));
        assert!(dock.quick_peek.is_some());

        // Primera sincronización con workspace A
        dock.sync_workspace("ws-alpha");
        assert_eq!(dock.last_workspace_id.as_deref(), Some("ws-alpha"));
        assert!(dock.quick_peek.is_none()); // Quick peek se cerró

        dock.files_tab.is_dirty = false;
        // Misma sincronización no invalida
        dock.sync_workspace("ws-alpha");
        assert!(!dock.files_tab.is_dirty);

        // Cambio a workspace B invalida inmediatamente todos los cachés
        dock.sync_workspace("ws-beta");
        assert_eq!(dock.last_workspace_id.as_deref(), Some("ws-beta"));
        assert!(dock.files_tab.is_dirty);
        assert!(dock.status_tab.is_dirty);
    }

    #[test]
    fn test_workspace_synchronization_persists_and_restores_state() {
        let mut dock = RightDockState::default();

        // Inicializamos workspace A
        dock.sync_workspace("ws-A");
        dock.files_tab.selected_index = 4;
        dock.files_tab.scroll_offset = 2;
        dock.files_tab
            .expanded_dirs
            .insert(PathBuf::from("src/client"));

        // Conmutamos a workspace B
        dock.sync_workspace("ws-B");
        assert_eq!(dock.last_workspace_id.as_deref(), Some("ws-B"));
        // El nuevo workspace B debe iniciar limpio
        assert_eq!(dock.files_tab.selected_index, 0);
        assert_eq!(dock.files_tab.scroll_offset, 0);
        assert!(dock.files_tab.expanded_dirs.is_empty());

        // Modificamos estado en workspace B
        dock.files_tab.selected_index = 1;
        dock.files_tab.expanded_dirs.insert(PathBuf::from("docs"));

        // Conmutamos de regreso a workspace A
        dock.sync_workspace("ws-A");
        assert_eq!(dock.last_workspace_id.as_deref(), Some("ws-A"));
        // Estado de workspace A debe haber sido restaurado íntegramente
        assert_eq!(dock.files_tab.selected_index, 4);
        assert_eq!(dock.files_tab.scroll_offset, 2);
        assert!(
            dock.files_tab
                .expanded_dirs
                .contains(&PathBuf::from("src/client"))
        );
        assert!(
            !dock
                .files_tab
                .expanded_dirs
                .contains(&PathBuf::from("docs"))
        );

        // Conmutamos de nuevo a workspace B
        dock.sync_workspace("ws-B");
        assert_eq!(dock.last_workspace_id.as_deref(), Some("ws-B"));
        assert_eq!(dock.files_tab.selected_index, 1);
        assert!(
            dock.files_tab
                .expanded_dirs
                .contains(&PathBuf::from("docs"))
        );
    }

    #[test]
    fn test_prune_dead_workspaces_evicts_closed_workspaces() {
        let mut dock = RightDockState::default();
        dock.sync_workspace("ws-1");
        dock.sync_workspace("ws-2");
        dock.sync_workspace("ws-3");

        // En este punto ws-1 y ws-2 deben estar en workspace_cache
        assert!(dock.workspace_cache.contains_key("ws-1"));
        assert!(dock.workspace_cache.contains_key("ws-2"));

        // Simulamos que ws-1 fue cerrado en el servidor y solo ws-2 y ws-3 siguen activos
        let mut active = std::collections::HashSet::new();
        active.insert("ws-2");
        active.insert("ws-3");
        dock.prune_dead_workspaces(&active);

        // ws-1 debe haber sido liberado de la memoria, ws-2 conservado
        assert!(!dock.workspace_cache.contains_key("ws-1"));
        assert!(dock.workspace_cache.contains_key("ws-2"));
    }

    #[test]
    fn test_toggle_hidden_dotfiles() {
        let root = TreeFixture::new();
        std::fs::create_dir(root.0.join(".config")).unwrap();
        std::fs::write(root.0.join(".bashrc"), "content").unwrap();
        std::fs::write(root.0.join("visible.txt"), "content").unwrap();

        let mut files = FilesTab::default();
        assert!(files.show_hidden);

        files.refresh_if_needed(&root.0);
        assert!(files.entries.iter().any(|e| e.name == ".config"));
        assert!(files.entries.iter().any(|e| e.name == ".bashrc"));
        assert!(files.entries.iter().any(|e| e.name == "visible.txt"));

        // Ocultar dotfiles
        files.toggle_hidden();
        assert!(!files.show_hidden);
        assert!(!files.entries.iter().any(|e| e.name == ".config"));
        assert!(!files.entries.iter().any(|e| e.name == ".bashrc"));
        assert!(files.entries.iter().any(|e| e.name == "visible.txt"));

        // Volver a mostrar dotfiles
        files.toggle_hidden();
        assert!(files.show_hidden);
        assert!(files.entries.iter().any(|e| e.name == ".config"));
        assert!(files.entries.iter().any(|e| e.name == ".bashrc"));
    }

    #[test]
    fn test_files_search_filter_and_cancellation() {
        let root = TreeFixture::new();
        std::fs::write(root.0.join("main.rs"), "fn main() {}").unwrap();
        std::fs::write(root.0.join("dock.rs"), "// dock").unwrap();
        std::fs::write(root.0.join("render.rs"), "// render").unwrap();

        let mut files = FilesTab::default();
        files.refresh_if_needed(&root.0);
        assert_eq!(files.entries.len(), 3);

        // Iniciar búsqueda
        files.start_search();
        assert!(files.is_searching);
        assert_eq!(files.search_query.as_deref(), Some(""));

        // Escribir "dock"
        for c in "dock".chars() {
            files.search_push(c);
        }
        assert_eq!(files.search_query.as_deref(), Some("dock"));
        let filtered = files.filtered_indices();
        assert_eq!(filtered.len(), 1);
        assert_eq!(files.entries[filtered[0]].name, "dock.rs");
        assert_eq!(files.selected_entry().unwrap().name, "dock.rs");

        // Borrar última letra con backspace -> "doc"
        files.search_backspace();
        assert_eq!(files.search_query.as_deref(), Some("doc"));
        assert_eq!(files.filtered_indices().len(), 1);

        // Cancelar búsqueda restaura todo
        files.cancel_search();
        assert!(!files.is_searching);
        assert!(files.search_query.is_none());
        assert_eq!(files.filtered_indices().len(), 3);
    }

    #[test]
    fn test_quick_peek_scrolling() {
        let mut peek = QuickPeekState::new(
            "sample.rs",
            vec!["line 1".into(), "line 2".into(), "line 3".into()],
        );
        assert_eq!(peek.scroll, 0);

        peek.scroll_down();
        assert_eq!(peek.scroll, 1);
        peek.scroll_down();
        assert_eq!(peek.scroll, 2);
        // Límite inferior
        peek.scroll_down();
        assert_eq!(peek.scroll, 2);

        peek.scroll_up();
        assert_eq!(peek.scroll, 1);
        peek.scroll_up();
        assert_eq!(peek.scroll, 0);
        // Límite superior
        peek.scroll_up();
        assert_eq!(peek.scroll, 0);
    }

    #[test]
    fn test_dock_layout_mode_toggle() {
        let mut dock = RightDockState::default();
        assert_eq!(dock.layout_mode, DockLayoutMode::SingleTab);

        dock.layout_mode = dock.layout_mode.toggle();
        assert_eq!(dock.layout_mode, DockLayoutMode::SplitVertical);

        dock.layout_mode = dock.layout_mode.toggle();
        assert_eq!(dock.layout_mode, DockLayoutMode::SingleTab);
    }

    #[test]
    fn test_render_right_dock_records_tab_and_file_hits() {
        let root = TreeFixture::new();
        std::fs::write(root.0.join("sample.txt"), "hello").unwrap();

        let mut dock = RightDockState::default();
        dock.select_tab(DockTabId::Files);
        dock.files_tab.refresh_if_needed(&root.0);

        let config = ClientShellConfig::from_config(&crate::config::Config::default());
        let area = Rect::new(50, 0, 40, 25);
        let mut buffer = Buffer::empty(area);
        let mut hits = crate::client::shell::state::ShellHitMap::default();

        render_right_dock(&mut buffer, area, &config, &mut dock, None, &mut hits);

        assert_eq!(hits.right_dock_area, area);
        assert_eq!(hits.right_dock_tabs.len(), 3);
        assert_eq!(hits.right_dock_tabs[0].1, DockTabId::Files);
        assert_eq!(hits.right_dock_tabs[1].1, DockTabId::Status);
        assert_eq!(hits.right_dock_tabs[2].1, DockTabId::GitFlow);
        assert_ne!(hits.right_dock_split_toggle, Rect::default());
        assert!(!hits.right_dock_file_rows.is_empty());
        assert_eq!(hits.right_dock_file_rows[0].1, 0);
    }

    #[test]
    fn test_compact_rail_records_tab_hits() {
        let mut dock = RightDockState::default();
        let config = ClientShellConfig::from_config(&crate::config::Config::default());
        let area = Rect::new(77, 0, 3, 20);
        let mut buffer = Buffer::empty(area);
        let mut hits = crate::client::shell::state::ShellHitMap::default();

        render_right_dock(&mut buffer, area, &config, &mut dock, None, &mut hits);

        assert_eq!(hits.right_dock_area, area);
        assert_eq!(hits.right_dock_tabs.len(), 3);
        assert_eq!(hits.right_dock_tabs[0].1, DockTabId::Files);
        assert_eq!(hits.right_dock_tabs[1].1, DockTabId::Status);
        assert_eq!(hits.right_dock_tabs[2].1, DockTabId::GitFlow);
    }

    #[test]
    fn test_mouse_clicks_and_scroll_in_right_dock() {
        use crate::client::shell::state::{ClientShellInput, ClientShellMode, ClientShellState};
        use crossterm::event::{KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

        let config = ClientShellConfig::from_config(&crate::config::Config::default());
        let mut state = ClientShellState::new(config);

        state.hits.right_dock_area = Rect::new(60, 0, 50, 20);
        state.hits.right_dock_tabs = vec![
            (Rect::new(61, 0, 11, 1), DockTabId::Status),
            (Rect::new(72, 0, 11, 1), DockTabId::Files),
            (Rect::new(83, 0, 14, 1), DockTabId::GitFlow),
        ];
        state.hits.right_dock_split_toggle = Rect::new(100, 0, 10, 1);
        state.hits.right_dock_file_rows = vec![(Rect::new(61, 2, 48, 1), 0)];

        // 1. Click on Files tab
        let mut outcome = ClientShellInput::default();
        state.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: 73,
                row: 0,
                modifiers: KeyModifiers::NONE,
            },
            &mut outcome,
        );
        assert_eq!(state.right_dock_state.active_tab, DockTabId::Files);
        assert_eq!(state.mode, ClientShellMode::Terminal);

        // 2. Click on Split toggle
        state.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: 102,
                row: 0,
                modifiers: KeyModifiers::NONE,
            },
            &mut outcome,
        );
        assert_eq!(
            state.right_dock_state.layout_mode,
            DockLayoutMode::SplitVertical
        );
        assert_eq!(state.mode, ClientShellMode::Terminal);

        // 3. Scroll Down
        state.right_dock_state.files_tab.entries = vec![
            FileTreeEntry {
                name: "folder".into(),
                relative_path: PathBuf::from("folder"),
                is_dir: true,
                depth: 0,
                is_expanded: false,
            },
            FileTreeEntry {
                name: "file1".into(),
                relative_path: PathBuf::from("file1"),
                is_dir: false,
                depth: 0,
                is_expanded: false,
            },
        ];
        state.right_dock_state.files_tab.set_visible_rows(1);
        state.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::ScrollDown,
                column: 65,
                row: 5,
                modifiers: KeyModifiers::NONE,
            },
            &mut outcome,
        );
        assert_eq!(state.right_dock_state.files_tab.scroll_offset, 1);
    }

    #[test]
    fn test_render_files_seti_icons_and_chevrons() {
        let root = TreeFixture::new();
        std::fs::create_dir(root.0.join("src")).unwrap();
        std::fs::write(root.0.join("package.json"), "{}").unwrap();
        std::fs::write(root.0.join("tsconfig.json"), "{}").unwrap();

        let mut dock = RightDockState::default();
        dock.select_tab(DockTabId::Files);
        dock.files_tab.refresh_if_needed(&root.0);

        let config = ClientShellConfig::from_config(&crate::config::Config::default());
        let area = Rect::new(0, 0, 40, 10);
        let mut buffer = Buffer::empty(area);
        let mut hits = crate::client::shell::state::ShellHitMap::default();

        render_right_dock(&mut buffer, area, &config, &mut dock, None, &mut hits);

        // Validamos que se hayan detectado filas
        assert_eq!(hits.right_dock_file_rows.len(), 3);

        // Verificamos que el buffer contiene el chevron cerrado › para la carpeta src
        // y los iconos Seti para package.json ({}) y tsconfig.json (TS)
        let mut row_contents = Vec::new();
        for row in 4..7 {
            let mut line = String::new();
            for col in 0..40 {
                line.push_str(buffer[(col, row)].symbol());
            }
            row_contents.push(line);
        }

        let full_text = row_contents.join("\n");
        assert!(
            full_text.contains('›'),
            "Debe contener el chevron de carpeta cerrada ›"
        );
        assert!(full_text.contains("src"), "Debe contener el directorio src");
        assert!(
            full_text.contains("{}"),
            "Debe contener el icono {{}} para package.json"
        );
        assert!(
            full_text.contains("TS"),
            "Debe contener el icono TS para tsconfig.json"
        );
    }

    #[test]
    fn test_dock_mouse_interactions_preserve_terminal_mode_and_no_footer() {
        use crate::client::shell::render;
        use crate::client::shell::state::{ClientShellInput, ClientShellMode, ClientShellState};
        use crossterm::event::{KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

        let config = ClientShellConfig::from_config(&crate::config::Config::default());
        let mut state = ClientShellState::new(config);
        assert_eq!(state.mode, ClientShellMode::Terminal);

        let area = Rect::new(60, 0, 40, 30);
        state.hits.right_dock_area = area;
        state.hits.right_dock_tabs = vec![
            (Rect::new(61, 0, 10, 1), DockTabId::Status),
            (Rect::new(72, 0, 10, 1), DockTabId::Files),
        ];
        state.hits.right_dock_split_toggle = Rect::new(90, 0, 10, 1);
        state.hits.right_dock_section_toggles = vec![(Rect::new(95, 2, 3, 1), 1)];

        let mut outcome = ClientShellInput::default();

        // Click on Tab
        state.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: 73,
                row: 0,
                modifiers: KeyModifiers::NONE,
            },
            &mut outcome,
        );
        assert_eq!(state.right_dock_state.active_tab, DockTabId::Files);
        assert_eq!(state.mode, ClientShellMode::Terminal, "Mouse tab click must preserve Terminal mode");

        // Click on Section Toggle
        state.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: 96,
                row: 2,
                modifiers: KeyModifiers::NONE,
            },
            &mut outcome,
        );
        assert_eq!(state.mode, ClientShellMode::Terminal, "Section accordion toggle must preserve Terminal mode");

        // Click on Background
        state.handle_mouse(
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: 70,
                row: 15,
                modifiers: KeyModifiers::NONE,
            },
            &mut outcome,
        );
        assert_eq!(state.mode, ClientShellMode::Terminal, "Dock background click must preserve Terminal mode");

        // Verify mode bar returns None for Terminal and RightDock modes
        let mut buffer = Buffer::empty(Rect::new(0, 0, 100, 30));
        let palette = crate::app::state::Palette::catppuccin();

        let terminal_footer = render::render_mode_bar(
            &mut buffer,
            Rect::new(0, 0, 60, 30),
            ClientShellMode::Terminal,
            None,
            None,
            false,
            &state.config.keybinds,
            &palette,
        );
        assert_eq!(terminal_footer, None, "Terminal mode must have no footer bar");

        let dock_footer = render::render_mode_bar(
            &mut buffer,
            Rect::new(0, 0, 60, 30),
            ClientShellMode::RightDock,
            None,
            None,
            false,
            &state.config.keybinds,
            &palette,
        );
        assert_eq!(dock_footer, None, "RightDock mode must never render intrusive footer bar across terminal");
    }

    #[test]
    fn test_status_tab_refresh_if_needed_updates_live_git_head_and_branch() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut status = StatusTab::new();
        status.refresh_if_needed(root, Some("feature-test"));
        assert_eq!(status.telemetry.git.branch, "feature-test");
        if let Some(head) = &status.telemetry.git.head_commit {
            assert_eq!(head.len(), 7);
            assert!(head.chars().all(|c| c.is_ascii_hexdigit()));
        }
    }

    #[test]
    fn test_status_tab_sync_agent_telemetry_parses_tokens() {
        let mut status = StatusTab::new();
        let tokens = vec![
            ("profile".to_string(), "claude-3-7-sonnet".to_string()),
            ("cost".to_string(), "0.412".to_string()),
            ("turns".to_string(), "14".to_string()),
            ("tok_in".to_string(), "45000".to_string()),
            ("tok_out".to_string(), "3200".to_string()),
            ("elapsed".to_string(), "2m 15s".to_string()),
            ("engram_obs".to_string(), "42".to_string()),
            ("todo_1".to_string(), "done:Preparar entorno".to_string()),
            ("todo_2".to_string(), "prog:Implementar sincronización".to_string()),
            ("todo_3".to_string(), "open:Verificar tests".to_string()),
        ];

        status.sync_agent_telemetry(Some(&tokens));

        assert_eq!(status.telemetry.model_name, "claude-3-7-sonnet");
        assert_eq!(status.telemetry.live_stats.spent_cost, 0.412);
        assert_eq!(status.telemetry.live_stats.turns, 14);
        assert_eq!(status.telemetry.live_stats.tokens_in, 45000);
        assert_eq!(status.telemetry.live_stats.tokens_out, 3200);
        assert_eq!(status.telemetry.live_stats.elapsed_str, "2m 15s");
        assert_eq!(status.telemetry.engram.observations_count, 42);
        assert!(status.telemetry.engram.is_online);

        assert_eq!(status.telemetry.todos.len(), 3);
        assert_eq!(status.telemetry.todos[0].title, "Preparar entorno");
        assert!(status.telemetry.todos[0].is_done);
        assert!(!status.telemetry.todos[0].is_in_progress);

        assert_eq!(status.telemetry.todos[1].title, "Implementar sincronización");
        assert!(!status.telemetry.todos[1].is_done);
        assert!(status.telemetry.todos[1].is_in_progress);

        assert_eq!(status.telemetry.todos[2].title, "Verificar tests");
        assert!(!status.telemetry.todos[2].is_done);
        assert!(!status.telemetry.todos[2].is_in_progress);
    }

    #[test]
    fn test_status_tab_sync_agent_telemetry_handles_context_and_clears_on_none() {
        let mut status = StatusTab::new();
        let tokens = vec![
            ("profile".to_string(), "claude-3-7-sonnet".to_string()),
            ("ctx_pct".to_string(), "42".to_string()),
            ("ctx_tok".to_string(), "84000".to_string()),
            ("ctx_win".to_string(), "200000".to_string()),
            ("cost".to_string(), "0.50".to_string()),
        ];

        status.sync_agent_telemetry(Some(&tokens));
        assert_eq!(status.telemetry.context.percent, 42.0);
        assert_eq!(status.telemetry.context.used_tokens, 84000);
        assert_eq!(status.telemetry.context.limit_tokens, 200000);
        assert_eq!(status.telemetry.live_stats.spent_cost, 0.50);

        // Al cambiar de pane o pasar None, debe resetear a estado neutral
        status.sync_agent_telemetry(None);
        assert_eq!(status.telemetry.context.percent, 0.0);
        assert_eq!(status.telemetry.context.used_tokens, 0);
        assert_eq!(status.telemetry.context.limit_tokens, 0);
        assert_eq!(status.telemetry.live_stats.spent_cost, 0.0);
        assert_eq!(status.telemetry.model_name, "");
    }

    #[test]
    fn test_render_status_content_shows_blocked_agent_in_agents_section() {
        let area = Rect::new(0, 0, 50, 30);
        let mut buffer = Buffer::empty(area);
        let config = ClientShellConfig::from_config(&crate::config::Config::default());
        let mut status_tab = StatusTab::new();

        let context = WorkspaceContext {
            workspace_id: "test-ws",
            workspace_name: "telar",
            root_path: Path::new("/"),
            branch: Some("feat/login"),
            git_ahead_behind: None,
            is_preview: false,
            agent_status: Some(crate::api::schema::AgentStatus::Blocked),
            agent_name: Some("gentle-ai"),
            agent_tokens: None,
            siblings: Vec::new(),
        };

        render_status_content(
            &mut buffer,
            0,
            0,
            30,
            50,
            &config,
            Some(&context),
            &mut status_tab,
            None,
        );

        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(text.contains("AGENTS"), "Must contain AGENTS header");
        assert!(text.contains("blocked"), "Must display blocked tag");
        assert!(text.contains("gentle-ai"), "Must display agent name gentle-ai");
        assert!(text.contains("▲"), "Must display warning triangle for blocked agent");
    }

    #[test]
    fn test_git_flow_tab_renders_live_timeline_and_scroll() {
        let mut dock = RightDockState::default();
        dock.select_tab(DockTabId::GitFlow);
        assert_eq!(dock.active_tab, DockTabId::GitFlow);

        let config = ClientShellConfig::from_config(&crate::config::Config::default());
        let area = Rect::new(0, 0, 50, 30);
        let mut buffer = Buffer::empty(area);
        let mut hits = crate::client::shell::state::ShellHitMap::default();

        let context = WorkspaceContext {
            workspace_id: "test-ws",
            workspace_name: "telar",
            root_path: Path::new("/"),
            branch: Some("feat/login"),
            git_ahead_behind: Some((2, 0)),
            is_preview: false,
            agent_status: None,
            agent_name: None,
            agent_tokens: None,
            siblings: Vec::new(),
        };

        render_right_dock(&mut buffer, area, &config, &mut dock, Some(&context), &mut hits);
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();

        assert!(text.contains("GIT FLOW & WORKTREE MAP (LIVE)"));
        assert!(text.contains("TOP"));
        assert!(text.contains("WORKTREE"));
        assert!(text.contains("SESIÓN ACTIVA"));
    }

    #[test]
    fn test_homologated_status_card_renders_unified_project_engram_and_mcp() {
        let config = ClientShellConfig::from_config(&crate::config::Config::default());
        let area = Rect::new(0, 0, 50, 25);
        let mut buffer = Buffer::empty(area);
        let mut status_tab = StatusTab::default();

        let context = WorkspaceContext {
            workspace_id: "one",
            workspace_name: "investigaciones",
            root_path: Path::new("/projects/investigaciones"),
            branch: Some("main"),
            git_ahead_behind: None,
            is_preview: false,
            agent_status: None,
            agent_name: None,
            agent_tokens: None,
            siblings: Vec::new(),
        };

        render_status_content(
            &mut buffer,
            0,
            0,
            25,
            50,
            &config,
            Some(&context),
            &mut status_tab,
            None,
        );

        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(text.contains("TODOS"));
        assert!(text.contains("AGENTS"));
        assert!(text.contains("CHANGES"));
        assert!(text.contains("STATS"));
        assert!(text.contains("INTEGRATIONS"));
        assert!(text.contains("investigaciones"));
    }

    #[test]
    fn test_selected_dir_path_extracts_directory_for_workspace_creation() {
        let root = TreeFixture::new();
        let sub_dir = root.0.join("subproject-alpha");
        std::fs::create_dir_all(&sub_dir).unwrap();
        std::fs::write(root.0.join("readme.md"), "# Hello").unwrap();

        let mut dock = RightDockState::default();
        dock.files_tab.refresh_if_needed(&root.0);

        // Encontrar índice del directorio
        let dir_idx = dock
            .files_tab
            .entries
            .iter()
            .position(|e| e.name == "subproject-alpha")
            .expect("subproject directory must exist in tree");
        dock.files_tab.selected_index = dir_idx;

        let (path, name) = dock
            .files_tab
            .selected_dir_path()
            .expect("must return directory path and name");
        assert_eq!(path, sub_dir);
        assert_eq!(name, "subproject-alpha");

        // Encontrar índice del archivo
        let file_idx = dock
            .files_tab
            .entries
            .iter()
            .position(|e| e.name == "readme.md")
            .expect("readme file must exist in tree");
        dock.files_tab.selected_index = file_idx;
        assert_eq!(
            dock.files_tab.selected_dir_path(),
            None,
            "selecting a file must return None for directory path"
        );
    }

    #[test]
    fn test_inaccessible_root_renders_gracefully() {
        let fake_path = PathBuf::from("/nonexistent/remote/path/192.168.10.150/projects");
        let mut dock = RightDockState::default();
        dock.select_tab(DockTabId::Files);
        dock.files_tab.refresh_if_needed(&fake_path);

        let config = ClientShellConfig::from_config(&crate::config::Config::default());
        let area = Rect::new(0, 0, 40, 10);
        let mut buffer = Buffer::empty(area);
        let mut hits = crate::client::shell::state::ShellHitMap::default();

        render_files_content(
            &mut buffer,
            0,
            0,
            10,
            40,
            &config,
            &mut dock.files_tab,
            &mut hits,
        );

        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(text.contains("(inaccessible or disconnected)"));
    }

    #[test]
    fn test_dock_files_w_key_creates_workspace_for_selected_directory() {
        use crate::client::shell::state::{ClientShellAction, ClientShellInput, ClientShellMode, ClientShellState};
        use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

        let root = TreeFixture::new();
        let sub_dir = root.0.join("remote-app");
        std::fs::create_dir_all(&sub_dir).unwrap();

        let config = ClientShellConfig::from_config(&crate::config::Config::default());
        let mut state = ClientShellState::new(config);
        state.set_snapshot(Box::new(crate::client::shell::tests::snapshot()));
        state.right_dock_state.active_tab = DockTabId::Files;
        state.right_dock_state.files_tab.refresh_if_needed(&root.0);

        // Seleccionar el subdirectorio
        let dir_idx = state
            .right_dock_state
            .files_tab
            .entries
            .iter()
            .position(|e| e.name == "remote-app")
            .expect("remote-app directory should exist");
        state.right_dock_state.files_tab.selected_index = dir_idx;
        state.mode = ClientShellMode::RightDock;

        let mut outcome = ClientShellInput::default();
        state.handle_key(
            KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE).into(),
            &mut outcome,
        );

        assert_eq!(state.mode, ClientShellMode::Terminal);
        let has_workspace_create = outcome.actions.iter().any(|action| {
            if let ClientShellAction::Endpoint { request, .. } = action {
                if let crate::api::schema::Method::WorkspaceCreate(params) = &request.method {
                    return params.label.as_deref() == Some("remote-app")
                        && params.cwd.as_deref() == Some(sub_dir.to_string_lossy().as_ref());
                }
            }
            false
        });
        assert!(
            has_workspace_create,
            "pressing 'w' on a directory must dispatch WorkspaceCreate for that folder"
        );
    }

    #[test]
    fn test_status_tab_sync_agent_telemetry_parses_git_changes_tokens() {
        let mut status = StatusTab::new();
        let tokens = vec![
            ("git_files".to_string(), "3".to_string()),
            ("git_lines".to_string(), "48".to_string()),
        ];
        status.sync_agent_telemetry(Some(&tokens));

        assert_eq!(status.telemetry.git.agent_files_changed, 3);
        assert_eq!(status.telemetry.git.agent_lines_added, 48);
    }

    #[test]
    fn test_render_right_dock_toggle_in_rail_and_expanded_modes() {
        let config = ClientShellConfig::from_config(&crate::config::Config::default());
        let mut dock = RightDockState::default();

        // 1. Modo Rail (ancho 3)
        let rail_area = Rect::new(97, 0, 3, 20);
        let mut rail_buffer = Buffer::empty(rail_area);
        let mut rail_hits = crate::client::shell::state::ShellHitMap::default();

        render_right_dock(
            &mut rail_buffer,
            rail_area,
            &config,
            &mut dock,
            None,
            &mut rail_hits,
        );

        assert_eq!(rail_hits.right_dock_toggle, Rect::new(98, 19, 1, 1));
        assert_eq!(rail_buffer[(98, 19)].symbol(), "«");
        assert_eq!(rail_hits.right_dock_header_toggle, Rect::new(98, 0, 1, 1));
        assert_eq!(rail_buffer[(98, 0)].symbol(), "«");

        // 2. Modo Expandido (ancho 30)
        let expanded_area = Rect::new(70, 0, 30, 20);
        let mut expanded_buffer = Buffer::empty(expanded_area);
        let mut expanded_hits = crate::client::shell::state::ShellHitMap::default();

        render_right_dock(
            &mut expanded_buffer,
            expanded_area,
            &config,
            &mut dock,
            None,
            &mut expanded_hits,
        );

        assert_eq!(expanded_hits.right_dock_toggle, Rect::new(98, 19, 1, 1));
        assert_eq!(expanded_buffer[(98, 19)].symbol(), "»");
        assert_eq!(expanded_hits.right_dock_header_toggle, Rect::new(96, 0, 3, 1));
        assert_eq!(expanded_buffer[(96, 0)].symbol(), "[");
        assert_eq!(expanded_buffer[(97, 0)].symbol(), "»");
        assert_eq!(expanded_buffer[(98, 0)].symbol(), "]");
    }

    #[test]
    fn test_status_tab_renders_cliproxy_accounts_and_model_quotas() {
        let area = Rect::new(0, 0, 48, 40);
        let mut buffer = Buffer::empty(area);
        let config = ClientShellConfig::default();
        let mut status_tab = StatusTab::new();

        status_tab.telemetry.accounts.push(super::telemetry::AccountQuota {
            name: "can-claude".to_string(),
            percent: 85,
            is_leader: true,
            role_symbol: "⚡",
            quota_desc: "85% · reset 4h 59m".to_string(),
        });

        status_tab.telemetry.model_quotas.push(super::telemetry::ModelQuota {
            model_name: "Claude 3.7".to_string(),
            window_type: "5h".to_string(),
            indicator: "85%".to_string(),
            reset_in: "4h 59m".to_string(),
        });

        render_status_tab_gentle_hud(
            &mut buffer,
            0,
            0,
            48,
            40,
            &config,
            &mut status_tab,
            None,
            &mut None,
        );

        let mut rendered_text = String::new();
        for y in 0..40 {
            for x in 0..48 {
                rendered_text.push_str(buffer[(x, y)].symbol());
            }
            rendered_text.push('\n');
        }

        assert!(rendered_text.contains("INTEGRATIONS"));
        assert!(rendered_text.contains("can-claude"));
        assert!(rendered_text.contains("Claude 3.7"));
    }
}
