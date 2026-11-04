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
    Status,
    Files,
    Changes,
}

#[allow(dead_code)]
impl DockTabId {
    pub fn title(self) -> &'static str {
        match self {
            Self::Status => "Status",
            Self::Files => "Files",
            Self::Changes => "Changes",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Status => "⚡",
            Self::Files => "📁",
            Self::Changes => "±",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Status => Self::Files,
            Self::Files => Self::Changes,
            Self::Changes => Self::Status,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Status => Self::Changes,
            Self::Files => Self::Status,
            Self::Changes => Self::Files,
        }
    }
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
    pub agent_tokens: Option<&'a [(String, String)]>,
    pub is_preview: bool,
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
        let mut collapsed = BTreeSet::new();
        // Proyecto (3) y SDD (5) colapsados por defecto para ajuste ergonómico
        collapsed.insert(3);
        collapsed.insert(5);
        Self {
            is_dirty: true,
            active_agent_status: None,
            running_jobs_count: 0,
            todos_summary: None,
            collapsed_sections: collapsed,
            telemetry: super::telemetry::StatusSnapshot::default(),
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

fn scan_tree_dir(
    root: &Path,
    rel_dir: &Path,
    depth: usize,
    expanded: &BTreeSet<PathBuf>,
    show_hidden: bool,
    entries: &mut Vec<FileTreeEntry>,
) {
    if depth > 20 {
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
        DockTabId::Changes
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
}

/// Estado global del Right Dock persistido en ClientShellState
#[derive(Debug, Default)]
pub struct RightDockState {
    pub active_tab: DockTabId,
    pub layout_mode: DockLayoutMode,
    pub status_tab: StatusTab,
    pub files_tab: FilesTab,
    pub changes_tab: ChangesTab,
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
                DockTabId::Changes => self.changes_tab.hibernate(),
            }
            self.active_tab = tab;
            // Marcar como dirty la nueva para refrescar perezosamente
            match self.active_tab {
                DockTabId::Status => self.status_tab.mark_dirty(),
                DockTabId::Files => self.files_tab.mark_dirty(),
                DockTabId::Changes => self.changes_tab.mark_dirty(),
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
                };
                self.workspace_cache.insert(old_id, old_state);
            }

            // 2. Restaurar estado del nuevo workspace si existía en caché, o inicializar limpio
            if let Some(mut restored) = self.workspace_cache.remove(current_workspace_id) {
                // Marcar como dirty para re-validar frescura sin destruir carpetas expandidas ni scroll
                restored.status_tab.mark_dirty();
                restored.files_tab.mark_dirty();
                restored.changes_tab.mark_dirty();
                self.status_tab = restored.status_tab;
                self.files_tab = restored.files_tab;
                self.changes_tab = restored.changes_tab;
            } else {
                self.status_tab = StatusTab::default();
                self.files_tab = FilesTab::default();
                self.changes_tab = ChangesTab::default();
                self.status_tab.mark_dirty();
                self.files_tab.mark_dirty();
                self.changes_tab.mark_dirty();
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
                let lines = if let Ok(content) = std::fs::read_to_string(&file_path) {
                    content.lines().take(100).map(String::from).collect()
                } else {
                    vec!["[Error: No se pudo leer el archivo binario o inaccesible]".into()]
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
            cell.set_style(Style::default().fg(config.palette.overlay0));
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
            let changes_color = if active == DockTabId::Changes {
                config.palette.accent
            } else {
                config.palette.subtext0
            };

            if area.height >= 2 {
                buffer.set_string(icon_x, area.y + 1, "⚡", Style::default().fg(status_color));
                hits.right_dock_tabs
                    .push((Rect::new(icon_x, area.y + 1, 2, 1), DockTabId::Status));
            }
            if area.height >= 4 {
                buffer.set_string(icon_x, area.y + 3, "📁", Style::default().fg(files_color));
                hits.right_dock_tabs
                    .push((Rect::new(icon_x, area.y + 3, 2, 1), DockTabId::Files));
            }
            if area.height >= 6 {
                buffer.set_string(icon_x, area.y + 5, "±", Style::default().fg(changes_color));
                hits.right_dock_tabs
                    .push((Rect::new(icon_x, area.y + 5, 2, 1), DockTabId::Changes));
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
    let changes_style = if dock_state.active_tab == DockTabId::Changes {
        active_style
    } else {
        inactive_style
    };

    buffer.set_string(content_x, tab_y, " ⚡ Status ", status_style);
    hits.right_dock_tabs
        .push((Rect::new(content_x, tab_y, 11, 1), DockTabId::Status));

    if content_x + 11 < area.right() {
        buffer.set_string(content_x + 11, tab_y, " 📁 Files ", files_style);
        hits.right_dock_tabs
            .push((Rect::new(content_x + 11, tab_y, 11, 1), DockTabId::Files));
    }
    if content_x + 22 < area.right() {
        buffer.set_string(content_x + 22, tab_y, " ± Changes ", changes_style);
        hits.right_dock_tabs
            .push((Rect::new(content_x + 22, tab_y, 13, 1), DockTabId::Changes));
    }

    // Indicador Split View en el extremo derecho del header (Punto 5)
    if area.width > 36 {
        let split_label = match dock_state.layout_mode {
            DockLayoutMode::SingleTab => "[⊞ Split]",
            DockLayoutMode::SplitVertical => "[⊟ Single]",
        };
        let split_style = if dock_state.layout_mode == DockLayoutMode::SplitVertical {
            Style::default()
                .fg(config.palette.peach)
                .bg(config.palette.surface0)
        } else {
            Style::default().fg(config.palette.subtext0)
        };
        let split_x = area.right().saturating_sub(10);
        buffer.set_string(split_x, tab_y, split_label, split_style);
        hits.right_dock_split_toggle = Rect::new(split_x, tab_y, 10, 1);
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
    let available_rows = area.bottom().saturating_sub(body_y);

    if dock_state.layout_mode == DockLayoutMode::SplitVertical && available_rows >= 14 {
        let half_rows = available_rows / 2;
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
        let lower_rows = area.bottom().saturating_sub(lower_body_y);
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
            DockTabId::Changes => {
                render_changes_content(
                    buffer,
                    content_x,
                    body_y,
                    available_rows,
                    area.width,
                    config,
                    context,
                );
            }
        }
    }

    // Si hay un Quick Peek activo, renderizar el modal flotante (Punto 6)
    if let Some(peek) = &dock_state.quick_peek {
        render_quick_peek_modal(buffer, area, config, peek);
    }
}

fn draw_card_box(
    buffer: &mut Buffer,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
    title: &str,
    badge: &str,
    border_style: Style,
    title_style: Style,
    badge_style: Style,
    hits: Option<&mut super::state::ShellHitMap>,
    section_id: Option<u8>,
) {
    if w < 6 || h == 0 || y >= buffer.area.bottom() || x >= buffer.area.right() {
        return;
    }
    let actual_w = w.min(buffer.area.right().saturating_sub(x));
    let right_x = x + actual_w.saturating_sub(1);

    // Caso colapsado (h == 1):
    if h == 1 {
        if let Some(c) = buffer.cell_mut((x, y)) {
            c.set_symbol("╭").set_style(border_style);
        }
        if let Some(c) = buffer.cell_mut((right_x, y)) {
            c.set_symbol("╮").set_style(border_style);
        }
        for col in (x + 1)..right_x {
            if let Some(c) = buffer.cell_mut((col, y)) {
                c.set_symbol("─").set_style(border_style);
            }
        }
        let title_x = x + 2;
        let badge_len = badge.chars().count() as u16;
        let badge_x = if badge_len > 0 && right_x > badge_len + 1 {
            right_x.saturating_sub(badge_len + 1)
        } else {
            right_x
        };
        let title_fmt = if title.starts_with('◈') || title.starts_with('•') || title.starts_with('⚡') {
            format!(" {title} ")
        } else {
            format!(" ◈ {title} ")
        };
        let max_title_len = (badge_x.saturating_sub(title_x + 1)) as usize;
        if max_title_len > 0 && !title.is_empty() {
            buffer.set_stringn(title_x, y, title_fmt, max_title_len, title_style);
        }
        if badge_len > 0 && badge_x > title_x {
            buffer.set_stringn(badge_x, y, badge, badge_len as usize, badge_style);
            if let (Some(h), Some(sec)) = (hits, section_id) {
                h.right_dock_section_toggles
                    .push((Rect::new(badge_x, y, badge_len, 1), sec));
            }
        }
        return;
    }

    let actual_h = h.min(buffer.area.bottom().saturating_sub(y));
    let bottom_y = y + actual_h.saturating_sub(1);

    // Esquinas redondeadas
    if let Some(c) = buffer.cell_mut((x, y)) {
        c.set_symbol("╭").set_style(border_style);
    }
    if let Some(c) = buffer.cell_mut((right_x, y)) {
        c.set_symbol("╮").set_style(border_style);
    }
    if let Some(c) = buffer.cell_mut((x, bottom_y)) {
        c.set_symbol("╰").set_style(border_style);
    }
    if let Some(c) = buffer.cell_mut((right_x, bottom_y)) {
        c.set_symbol("╯").set_style(border_style);
    }

    // Bordes laterales
    for row in (y + 1)..bottom_y {
        if let Some(c) = buffer.cell_mut((x, row)) {
            c.set_symbol("│").set_style(border_style);
        }
        if let Some(c) = buffer.cell_mut((right_x, row)) {
            c.set_symbol("│").set_style(border_style);
        }
    }

    // Borde inferior
    for col in (x + 1)..right_x {
        if let Some(c) = buffer.cell_mut((col, bottom_y)) {
            c.set_symbol("─").set_style(border_style);
        }
    }

    // Borde superior con Título y Badge
    for col in (x + 1)..right_x {
        if let Some(c) = buffer.cell_mut((col, y)) {
            c.set_symbol("─").set_style(border_style);
        }
    }

    // Título a la izquierda: "╭─ Titulo ──"
    let title_x = x + 2;
    let badge_len = badge.chars().count() as u16;
    let badge_x = if badge_len > 0 && right_x > badge_len + 1 {
        right_x.saturating_sub(badge_len + 1)
    } else {
        right_x
    };

    let title_fmt = if title.starts_with('◈') || title.starts_with('•') || title.starts_with('⚡') {
        format!(" {title} ")
    } else {
        format!(" ◈ {title} ")
    };
    let max_title_len = (badge_x.saturating_sub(title_x + 1)) as usize;
    if max_title_len > 0 && !title.is_empty() {
        buffer.set_stringn(title_x, y, title_fmt, max_title_len, title_style);
    }

    // Badge a la derecha: "[-]╮"
    if badge_len > 0 && badge_x > title_x {
        buffer.set_stringn(badge_x, y, badge, badge_len as usize, badge_style);
        if let (Some(h), Some(sec)) = (hits, section_id) {
            h.right_dock_section_toggles
                .push((Rect::new(badge_x, y, badge_len, 1), sec));
        }
    }
}

fn render_mini_bar(
    buffer: &mut Buffer,
    x: u16,
    y: u16,
    width: u16,
    percent: u8,
    active_style: Style,
    inactive_style: Style,
) {
    if width == 0 || y >= buffer.area.bottom() {
        return;
    }
    let filled = ((width as u32 * percent.min(100) as u32) / 100) as u16;
    for i in 0..width {
        let col = x + i;
        if col >= buffer.area.right() {
            break;
        }
        if let Some(cell) = buffer.cell_mut((col, y)) {
            if i < filled {
                cell.set_symbol("━").set_style(active_style);
            } else {
                cell.set_symbol("─").set_style(inactive_style);
            }
        }
    }
}

fn render_card_kv(
    buffer: &mut Buffer,
    x: u16,
    y: u16,
    inner_w: u16,
    key: &str,
    val: &str,
    key_style: Style,
    val_style: Style,
) {
    if inner_w < 6 || y >= buffer.area.bottom() {
        return;
    }
    let key_col_w = (key.chars().count() as u16).min(12);
    let val_x = x + key_col_w + 1;
    let max_val_len = (inner_w.saturating_sub(key_col_w + 1)) as usize;

    buffer.set_stringn(x, y, key, key_col_w as usize, key_style);
    if val_x < buffer.area.right() && max_val_len > 0 {
        buffer.set_stringn(val_x, y, val, max_val_len, val_style);
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

    let card_border_style = Style::default().fg(config.palette.overlay0);
    let card_title_style = Style::default()
        .fg(config.palette.mauve)
        .add_modifier(ratatui::style::Modifier::BOLD);
    let card_badge_style = Style::default()
        .fg(config.palette.accent)
        .add_modifier(ratatui::style::Modifier::BOLD);
    let key_style = Style::default().fg(config.palette.overlay0);
    let val_accent = Style::default().fg(config.palette.accent);
    let val_text = Style::default()
        .fg(config.palette.text)
        .add_modifier(ratatui::style::Modifier::BOLD);
    let val_dim = Style::default().fg(config.palette.subtext0);
    let card_w = width.saturating_sub(2);
    let inner_w = card_w.saturating_sub(2);

    // 1. Cabecera superior centrada: ":: TELAR · GENTLE-AI ::"
    let banner = if is_preview {
        ":: TELAR [Preview] · GENTLE-AI ::"
    } else {
        ":: TELAR · GENTLE-AI ::"
    };
    let banner_len = banner.chars().count() as u16;
    let banner_x = x + (width.saturating_sub(banner_len)) / 2;
    buffer.set_stringn(
        banner_x,
        y,
        banner,
        banner_len as usize,
        Style::default().fg(config.palette.overlay0),
    );

    let mut cur_y = y + 1;
    let bottom_limit = y + rows;

    // 1. Tarjeta: Estado (Workspace, Agent, Modelo, Mem) (Sección 1)
    if config.right_dock_show_state && cur_y < bottom_limit {
        let is_col = status_tab.is_collapsed(1);
        let badge = if is_col { "[+]" } else { "[-]" };
        let agent_st = context
            .and_then(|c| c.agent_status)
            .filter(|s| *s != crate::api::schema::AgentStatus::Unknown)
            .map(super::status_text)
            .unwrap_or("unavailable");

        let header_title = if is_col {
            format!("Estado: {ws_name} ({branch_name}) · {agent_st}")
        } else {
            format!("Estado · {agent_st}")
        };
        let card_h = if is_col {
            1
        } else {
            5u16.min(bottom_limit.saturating_sub(cur_y))
        };
        if card_h >= 1 {
            draw_card_box(
                buffer,
                x + 1,
                cur_y,
                card_w,
                card_h,
                &header_title,
                badge,
                card_border_style,
                card_title_style,
                card_badge_style,
                hits.as_deref_mut(),
                Some(1),
            );

            if !is_col && card_h >= 4 {
                let body_x = x + 2;
                let mut row_offset = 1;
                render_card_kv(
                    buffer,
                    body_x,
                    cur_y + row_offset,
                    inner_w,
                    "Proyecto:  ",
                    ws_name,
                    key_style,
                    val_text,
                );
                row_offset += 1;
                if config.right_dock_show_git {
                    render_card_kv(
                        buffer,
                        body_x,
                        cur_y + row_offset,
                        inner_w,
                        "Rama:      ",
                        branch_name,
                        key_style,
                        Style::default().fg(config.palette.mauve),
                    );
                    row_offset += 1;
                }
                if cur_y + row_offset < bottom_limit {
                    render_card_kv(
                        buffer,
                        body_x,
                        cur_y + row_offset,
                        inner_w,
                        "Modelo:    ",
                        &status_tab.telemetry.model_name,
                        key_style,
                        val_accent,
                    );
                }
            }
            cur_y += card_h;
        }
    }

    // 2. Tarjeta: Contexto y Tokens (Sección 2)
    if config.right_dock_show_context && cur_y < bottom_limit {
        let is_col = status_tab.is_collapsed(2);
        let badge = if is_col { "[+]" } else { "[-]" };
        let tokens_val = context
            .and_then(|c| c.agent_tokens)
            .and_then(|tokens| {
                tokens.iter().find_map(|(k, v)| {
                    if k.contains("token") || k.contains("context") {
                        Some(v.as_str())
                    } else {
                        None
                    }
                })
            })
            .unwrap_or("179.2k");
        let header_title = if is_col {
            format!("Contexto: {tokens_val} ({:.0}%)", status_tab.telemetry.context.percent)
        } else {
            "Contexto".to_string()
        };
        let card_h = if is_col {
            1
        } else {
            4u16.min(bottom_limit.saturating_sub(cur_y))
        };
        if card_h >= 1 {
            draw_card_box(
                buffer,
                x + 1,
                cur_y,
                card_w,
                card_h,
                &header_title,
                badge,
                card_border_style,
                card_title_style,
                card_badge_style,
                hits.as_deref_mut(),
                Some(2),
            );

            if !is_col && card_h >= 3 {
                let body_x = x + 2;
                let line = format!(
                    "Context:   {tokens_val} / 1.0M  • {}",
                    status_tab.telemetry.context.status_label
                );
                let status_style = if status_tab.telemetry.context.percent > 80.0 {
                    Style::default().fg(config.palette.red)
                } else if status_tab.telemetry.context.percent > 50.0 {
                    Style::default().fg(config.palette.yellow)
                } else {
                    Style::default().fg(config.palette.green)
                };
                buffer.set_stringn(
                    body_x,
                    cur_y + 1,
                    line,
                    inner_w as usize,
                    status_style,
                );

                if cur_y + 2 < bottom_limit {
                    let bar_color = if status_tab.telemetry.context.percent > 80.0 {
                        config.palette.red
                    } else if status_tab.telemetry.context.percent > 50.0 {
                        config.palette.yellow
                    } else {
                        config.palette.teal
                    };
                    render_mini_bar(
                        buffer,
                        body_x,
                        cur_y + 2,
                        inner_w,
                        status_tab.telemetry.context.percent as u8,
                        Style::default().fg(bar_color),
                        Style::default().fg(config.palette.surface0),
                    );
                }

                if cur_y + 3 < bottom_limit {
                    let cost_line = format!(
                        "^ 1.0M · v 14.4k   Costo ${:.3}",
                        status_tab.telemetry.context.cost
                    );
                    buffer.set_stringn(
                        body_x,
                        cur_y + 3,
                        cost_line,
                        inner_w as usize,
                        val_dim,
                    );
                }
            }
            cur_y += card_h;
        }
    }

    // 3. Tarjeta: ~ Proyecto (Sección 3)
    if config.right_dock_show_state && cur_y < bottom_limit {
        let is_col = status_tab.is_collapsed(3);
        let badge = if is_col { "[+]" } else { "[-]" };
        let header = if is_col {
            format!("Proyecto: {ws_name} ({}d)", status_tab.telemetry.project_days)
        } else {
            format!("Proyecto · {ws_name} {}d", status_tab.telemetry.project_days)
        };
        let card_h = if is_col {
            1
        } else {
            3u16.min(bottom_limit.saturating_sub(cur_y))
        };
        if card_h >= 1 {
            draw_card_box(
                buffer,
                x + 1,
                cur_y,
                card_w,
                card_h,
                &header,
                badge,
                card_border_style,
                card_title_style,
                card_badge_style,
                hits.as_deref_mut(),
                Some(3),
            );
            if !is_col && card_h >= 2 {
                let body_x = x + 2;
                render_card_kv(
                    buffer,
                    body_x,
                    cur_y + 1,
                    inner_w,
                    "Workspace: ",
                    ws_name,
                    key_style,
                    val_text,
                );
            }
            cur_y += card_h;
        }
    }

    // 4. Tarjeta: CPAMC / Integraciones (Sección 4)
    if config.right_dock_show_cpamc && cur_y < bottom_limit {
        let is_col = status_tab.is_collapsed(4);
        let badge = if is_col { "[+]" } else { "[-]" };

        match config.cpamc_experience {
            crate::config::CpamcExperienceConfig::AutoSwitcher => {
                let pool_name = context.and_then(|c| c.agent_name).unwrap_or("Gemini");
                let header_title = if is_col {
                    format!("Pool: {pool_name} · 5h (0% libre)")
                } else {
                    format!("Pool: {pool_name} · ○ Esperando")
                };
                let card_h = if is_col {
                    1
                } else {
                    6u16.min(bottom_limit.saturating_sub(cur_y))
                };
                if card_h >= 1 {
                    draw_card_box(
                        buffer,
                        x + 1,
                        cur_y,
                        card_w,
                        card_h,
                        &header_title,
                        badge,
                        card_border_style,
                        card_title_style,
                        card_badge_style,
                        hits.as_deref_mut(),
                        Some(4),
                    );

                    if !is_col && card_h >= 4 {
                        let body_x = x + 2;
                        buffer.set_stringn(
                            body_x,
                            cur_y + 1,
                            "5 HORAS:   Quedan 5h 0m (0% libre)",
                            inner_w as usize,
                            val_text,
                        );
                        if cur_y + 2 < bottom_limit {
                            render_mini_bar(
                                buffer,
                                body_x,
                                cur_y + 2,
                                inner_w,
                                35,
                                Style::default().fg(config.palette.teal),
                                Style::default().fg(config.palette.surface0),
                            );
                        }
                        if cur_y + 3 < bottom_limit {
                            buffer.set_stringn(
                                body_x,
                                cur_y + 3,
                                "SEMANAL:   0% 7d 0h",
                                inner_w as usize,
                                val_dim,
                            );
                        }
                        if cur_y + 4 < bottom_limit {
                            buffer.set_stringn(
                                body_x,
                                cur_y + 4,
                                "🛡 Centinela · ● En guardia",
                                inner_w as usize,
                                Style::default().fg(config.palette.green),
                            );
                        }
                    }
                    cur_y += card_h;
                }
            }
            crate::config::CpamcExperienceConfig::Totopo27 => {
                let count = status_tab.telemetry.accounts.len();
                let header_title = if is_col {
                    format!("Cuentas: {count} activas")
                } else {
                    format!("CPAMC · {count} cuentas")
                };
                let card_h = if is_col {
                    1
                } else {
                    4u16.min(bottom_limit.saturating_sub(cur_y))
                };
                if card_h >= 1 {
                    draw_card_box(
                        buffer,
                        x + 1,
                        cur_y,
                        card_w,
                        card_h,
                        &header_title,
                        badge,
                        card_border_style,
                        card_title_style,
                        card_badge_style,
                        hits.as_deref_mut(),
                        Some(4),
                    );

                    if !is_col && card_h >= 3 {
                        let body_x = x + 2;
                        render_card_kv(
                            buffer,
                            body_x,
                            cur_y + 1,
                            inner_w,
                            "Cuenta:    ",
                            "activa",
                            key_style,
                            val_text,
                        );
                        render_card_kv(
                            buffer,
                            body_x,
                            cur_y + 2,
                            inner_w,
                            "Cuota 5h:  ",
                            "100% libre",
                            key_style,
                            val_accent,
                        );
                    }
                    cur_y += card_h;
                }
            }
        }
    }

    // 5. Tarjeta: SDD Mono-modelo (Sección 5)
    if cur_y < bottom_limit {
        let is_col = status_tab.is_collapsed(5);
        let badge = if is_col { "[+]" } else { "[-]" };
        let header = if is_col {
            format!("SDD: {} · {} mod", status_tab.telemetry.active_account, status_tab.telemetry.model_quotas.len())
        } else {
            format!("SDD · {}", status_tab.telemetry.active_account)
        };
        let card_h = if is_col {
            1
        } else {
            (status_tab.telemetry.model_quotas.len() as u16 + 2)
                .min(bottom_limit.saturating_sub(cur_y))
        };
        if card_h >= 1 {
            draw_card_box(
                buffer,
                x + 1,
                cur_y,
                card_w,
                card_h,
                &header,
                badge,
                card_border_style,
                card_title_style,
                card_badge_style,
                hits.as_deref_mut(),
                Some(5),
            );
            if !is_col && card_h >= 3 {
                let body_x = x + 2;
                for (idx, mq) in status_tab.telemetry.model_quotas.iter().enumerate() {
                    let row_y = cur_y + 1 + idx as u16;
                    if row_y >= bottom_limit.saturating_sub(1) {
                        break;
                    }
                    let line = format!(
                        "• {:<11} ─── {:>5} {}",
                        mq.model_name, mq.indicator, mq.reset_in
                    );
                    buffer.set_stringn(
                        body_x,
                        row_y,
                        line,
                        inner_w as usize,
                        Style::default().fg(config.palette.teal),
                    );
                }
            }
            cur_y += card_h;
        }
    }

    // 6. Tarjeta: Cambios · Git (Sección 6)
    if config.right_dock_show_git && cur_y < bottom_limit {
        let is_col = status_tab.is_collapsed(6);
        let badge = if is_col { "[+]" } else { "[-]" };
        let header = if is_col {
            format!("Git: limpio ({branch_name}) · ag: {}", status_tab.telemetry.git.agent_files_changed)
        } else {
            format!("Git · {} archivos ag", status_tab.telemetry.git.agent_files_changed)
        };
        let card_h = if is_col {
            1
        } else {
            5u16.min(bottom_limit.saturating_sub(cur_y))
        };
        if card_h >= 1 {
            draw_card_box(
                buffer,
                x + 1,
                cur_y,
                card_w,
                card_h,
                &header,
                badge,
                card_border_style,
                card_title_style,
                card_badge_style,
                hits.as_deref_mut(),
                Some(6),
            );

            if !is_col && card_h >= 3 {
                let body_x = x + 2;
                let row1 = format!(
                    "{} * {} mod · ?{}",
                    branch_name,
                    status_tab.telemetry.git.modified_count,
                    status_tab.telemetry.git.untracked_count
                );
                buffer.set_stringn(
                    body_x,
                    cur_y + 1,
                    row1,
                    inner_w as usize,
                    val_text,
                );

                if cur_y + 2 < bottom_limit {
                    let head_line = format!(
                        "* {} (HEAD -> {})",
                        status_tab
                            .telemetry
                            .git
                            .head_commit
                            .as_deref()
                            .unwrap_or(""),
                        branch_name
                    );
                    buffer.set_stringn(
                        body_x,
                        cur_y + 2,
                        head_line,
                        inner_w as usize,
                        val_dim,
                    );
                }
                if cur_y + 3 < bottom_limit {
                    let git_val =
                        if let Some((ahead, behind)) = context.and_then(|c| c.git_ahead_behind) {
                            format!("+{ahead}/-{behind}")
                        } else if branch_name == "unavailable" {
                            "unavailable".to_string()
                        } else {
                            "unavailable".to_string()
                        };
                    let git_status_text = if git_val == "unavailable" {
                        "Git:       unavailable".to_string()
                    } else {
                        format!(
                            "git: limpio   ag: {} archivos (+{})",
                            status_tab.telemetry.git.agent_files_changed,
                            status_tab.telemetry.git.agent_lines_added
                        )
                    };
                    buffer.set_stringn(
                        body_x,
                        cur_y + 3,
                        git_status_text,
                        inner_w as usize,
                        Style::default().fg(config.palette.green),
                    );
                }
            }
            cur_y += card_h;
        }
    }

    // 7. Tarjeta: Engram (Sección 7)
    if config.right_dock_show_engram && cur_y < bottom_limit {
        let is_col = status_tab.is_collapsed(7);
        let badge = if is_col { "[+]" } else { "[-]" };
        let header = if is_col {
            format!("Engram: {} · {} obs", status_tab.telemetry.engram.project_name, status_tab.telemetry.engram.observations_count)
        } else {
            format!("Engram · {}", status_tab.telemetry.engram.project_name)
        };
        let card_h = if is_col {
            1
        } else {
            3u16.min(bottom_limit.saturating_sub(cur_y))
        };
        if card_h >= 1 {
            draw_card_box(
                buffer,
                x + 1,
                cur_y,
                card_w,
                card_h,
                &header,
                badge,
                card_border_style,
                card_title_style,
                card_badge_style,
                hits.as_deref_mut(),
                Some(7),
            );

            if !is_col && card_h >= 2 {
                let body_x = x + 2;
                let engram_port = format!("Local ({})  ", status_tab.telemetry.engram.port);
                let port_w = engram_port.chars().count() as u16;
                buffer.set_stringn(body_x, cur_y + 1, engram_port, port_w as usize, val_text);
                if body_x + port_w < buffer.area.right() {
                    buffer.set_stringn(
                        body_x + port_w,
                        cur_y + 1,
                        "• En linea",
                        10,
                        Style::default().fg(config.palette.green),
                    );
                    let obs_str = format!(" · {} ob", status_tab.telemetry.engram.observations_count);
                    if body_x + port_w + 10 < buffer.area.right() {
                        buffer.set_stringn(
                            body_x + port_w + 10,
                            cur_y + 1,
                            obs_str,
                            inner_w.saturating_sub(port_w + 10) as usize,
                            val_dim,
                        );
                    }
                }
                if cur_y + 2 < bottom_limit {
                    let cloud_line = format!("Cloud: {}", status_tab.telemetry.engram.cloud_status);
                    buffer.set_stringn(body_x, cur_y + 2, cloud_line, inner_w as usize, val_dim);
                }
            }
            cur_y += card_h;
        }
    }

    // 8. Tarjeta: Tools (Sección 8)
    if config.right_dock_show_tools && cur_y < bottom_limit {
        let is_col = status_tab.is_collapsed(8);
        let badge = if is_col { "[+]" } else { "[-]" };
        let header = if is_col {
            format!("Tools: {} llamadas · r:{} w:{}", status_tab.telemetry.tools.total_calls, status_tab.telemetry.tools.reads, status_tab.telemetry.tools.writes)
        } else {
            format!("Tools · {} llamadas", status_tab.telemetry.tools.total_calls)
        };
        let card_h = if is_col {
            1
        } else {
            3u16.min(bottom_limit.saturating_sub(cur_y))
        };
        if card_h >= 1 {
            draw_card_box(
                buffer,
                x + 1,
                cur_y,
                card_w,
                card_h,
                &header,
                badge,
                card_border_style,
                card_title_style,
                card_badge_style,
                hits.as_deref_mut(),
                Some(8),
            );

            if !is_col && card_h >= 2 {
                let body_x = x + 2;
                let line = format!(
                    "@ {} lecturas  @ {} escritur  mem: {}",
                    status_tab.telemetry.tools.reads,
                    status_tab.telemetry.tools.writes,
                    status_tab.telemetry.tools.memory_ops
                );
                buffer.set_stringn(
                    body_x,
                    cur_y + 1,
                    line,
                    inner_w as usize,
                    val_dim,
                );
            }
            cur_y += card_h;
        }
    }

    // 9. Tarjeta: MCP (Sección 9)
    if config.right_dock_show_mcp && cur_y < bottom_limit {
        let is_col = status_tab.is_collapsed(9);
        let badge = if is_col { "[+]" } else { "[-]" };
        let count = status_tab.telemetry.mcp_servers.len();
        let header = if is_col {
            format!("MCP: {count} activos")
        } else {
            format!("MCP · {count} activos")
        };
        let card_h = if is_col {
            1
        } else {
            (count as u16 + 2).min(bottom_limit.saturating_sub(cur_y))
        };
        if card_h >= 1 {
            draw_card_box(
                buffer,
                x + 1,
                cur_y,
                card_w,
                card_h,
                &header,
                badge,
                card_border_style,
                card_title_style,
                card_badge_style,
                hits.as_deref_mut(),
                Some(9),
            );

            if !is_col && card_h >= 3 {
                let body_x = x + 2;
                for (idx, srv) in status_tab.telemetry.mcp_servers.iter().enumerate() {
                    let row_y = cur_y + 1 + idx as u16;
                    if row_y >= bottom_limit.saturating_sub(1) {
                        break;
                    }
                    let name_str = format!("• {:<12}", srv.name);
                    buffer.set_stringn(body_x, row_y, name_str, 14, val_text);
                    let scope_str = format!("[{}]", srv.scope);
                    if body_x + 14 < buffer.area.right() {
                        buffer.set_stringn(body_x + 14, row_y, scope_str, 10, key_style);
                    }
                    let status_x = body_x + inner_w.saturating_sub(srv.status.chars().count() as u16);
                    if status_x > body_x + 20 {
                        let st_style = if srv.status.contains("linea") || srv.status.contains("línea") || srv.status.contains("online") {
                            Style::default().fg(config.palette.green)
                        } else {
                            val_dim
                        };
                        buffer.set_string(status_x, row_y, &srv.status, st_style);
                    }
                }
            }
            cur_y += card_h;
        }
    }

    // 10. Footer de atajos rápidos de navegación al final si queda espacio
    if bottom_limit.saturating_sub(cur_y) >= 2 {
        let footer_y = bottom_limit.saturating_sub(2);
        buffer.set_stringn(
            x + 1,
            footer_y,
            "1-9: colapsar · c: plegar todo",
            card_w as usize,
            val_dim,
        );
        buffer.set_stringn(
            x + 1,
            footer_y + 1,
            "d: expandir · s: sync · q: salir",
            card_w as usize,
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
            "• EXPLORER ({}/{}) [Space=Peek] {}",
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
            buffer.set_string(
                x + 1,
                y + 2,
                "(no files)",
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

fn render_changes_content(
    buffer: &mut Buffer,
    x: u16,
    y: u16,
    rows: u16,
    width: u16,
    config: &ClientShellConfig,
    context: Option<&WorkspaceContext>,
) {
    if rows < 4 || width < 12 {
        return;
    }

    buffer.set_string(
        x + 1,
        y,
        "• GIT CHANGES",
        Style::default().fg(config.palette.accent),
    );
    if let Some(ctx) = context {
        if let Some((ahead, behind)) = ctx.git_ahead_behind {
            buffer.set_string(
                x + 1,
                y + 2,
                format!("Tracking: ↑{ahead} ↓{behind}"),
                Style::default().fg(config.palette.subtext0),
            );
        } else {
            buffer.set_string(
                x + 1,
                y + 2,
                "Working tree clean",
                Style::default().fg(config.palette.green),
            );
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
    fn status_renders_rounded_card_boxes_and_banner() {
        let config = ClientShellConfig::from_config(&crate::config::Config::default());
        let area = Rect::new(0, 0, 40, 25);
        let mut buffer = Buffer::empty(area);
        let mut status_tab = StatusTab::default();
        render_status_content(
            &mut buffer,
            0,
            0,
            25,
            40,
            &config,
            None,
            &mut status_tab,
            None,
        );
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(text.contains(":: TELAR · GENTLE-AI ::"));
        assert!(text.contains('╭'));
        assert!(text.contains('╰'));
        assert!(text.contains('│'));
        assert!(text.contains('─'));
        assert!(text.contains("[-]"));
    }

    #[test]
    fn status_does_not_show_fabricated_jobs_or_todos() {
        let config = ClientShellConfig::from_config(&crate::config::Config::default());
        let area = Rect::new(0, 0, 50, 10);
        let mut buffer = Buffer::empty(area);
        let mut status_tab = StatusTab::default();
        render_status_content(
            &mut buffer,
            0,
            0,
            10,
            50,
            &config,
            None,
            &mut status_tab,
            None,
        );
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(!text.contains("Jobs (0 running)"));
        assert!(!text.contains("Todos (100% idle)"));
        assert!(text.contains("unavailable"));
    }

    #[test]
    fn status_shows_workspace_agent_and_branch_without_guessing_git_state() {
        let config = ClientShellConfig::from_config(&crate::config::Config::default());
        let area = Rect::new(0, 0, 60, 30);
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
        };
        let mut status_tab = StatusTab::default();
        render_status_content(
            &mut buffer,
            0,
            0,
            30,
            60,
            &config,
            Some(&context),
            &mut status_tab,
            None,
        );
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(text.contains("telar"));
        assert!(text.contains("working"));
        assert!(text.contains("main"));
        assert!(text.contains("unavailable"));
    }

    #[test]
    fn status_sections_can_be_hidden_independently() {
        let mut config = ClientShellConfig::from_config(&crate::config::Config::default());
        let area = Rect::new(0, 0, 60, 30);
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
        };
        config.right_dock_show_state = false;
        let mut buffer = Buffer::empty(area);
        let mut status_tab = StatusTab::default();
        render_status_content(
            &mut buffer,
            0,
            0,
            30,
            60,
            &config,
            Some(&context),
            &mut status_tab,
            None,
        );
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(!text.contains("project-one"));
        assert!(text.contains("branch-one"));
        config.right_dock_show_state = true;
        config.right_dock_show_git = false;
        let mut buffer = Buffer::empty(area);
        render_status_content(
            &mut buffer,
            0,
            0,
            30,
            60,
            &config,
            Some(&context),
            &mut status_tab,
            None,
        );
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(text.contains("project-one"));
        assert!(!text.contains("branch-one"));
    }

    #[test]
    fn status_renders_engram_and_context_cards_and_honors_visibility() {
        let mut config = ClientShellConfig::from_config(&crate::config::Config::default());
        let area = Rect::new(0, 0, 60, 45);
        let context = WorkspaceContext {
            workspace_id: "one",
            workspace_name: "test-ws",
            root_path: Path::new("/"),
            branch: Some("main"),
            git_ahead_behind: None,
            is_preview: false,
            agent_status: None,
            agent_name: None,
            agent_tokens: None,
        };
        let mut buffer = Buffer::empty(area);
        let mut status_tab = StatusTab::default();
        render_status_content(
            &mut buffer,
            0,
            0,
            45,
            60,
            &config,
            Some(&context),
            &mut status_tab,
            None,
        );
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(text.contains("Engram"));
        assert!(text.contains("Context"));

        config.right_dock_show_engram = false;
        config.right_dock_show_context = false;
        let mut buffer = Buffer::empty(area);
        render_status_content(
            &mut buffer,
            0,
            0,
            45,
            60,
            &config,
            Some(&context),
            &mut status_tab,
            None,
        );
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(!text.contains("ENGRAM"));
        assert!(!text.contains("Context"));
    }

    #[test]
    fn status_renders_cpamc_experience_variants() {
        let mut config = ClientShellConfig::from_config(&crate::config::Config::default());
        let area = Rect::new(0, 0, 60, 25);
        let context = WorkspaceContext {
            workspace_id: "one",
            workspace_name: "test-ws",
            root_path: Path::new("/"),
            branch: Some("main"),
            git_ahead_behind: None,
            is_preview: false,
            agent_status: None,
            agent_name: Some("Gemini"),
            agent_tokens: None,
        };
        // AutoSwitcher mode
        config.cpamc_experience = crate::config::CpamcExperienceConfig::AutoSwitcher;
        let mut buffer = Buffer::empty(area);
        let mut status_tab = StatusTab::default();
        render_status_content(
            &mut buffer,
            0,
            0,
            25,
            60,
            &config,
            Some(&context),
            &mut status_tab,
            None,
        );
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(text.contains("Pool: Gemini"));
        assert!(text.contains("Centinela"));
        assert!(text.contains("5 HORAS"));

        // Totopo27 mode
        config.cpamc_experience = crate::config::CpamcExperienceConfig::Totopo27;
        let mut buffer = Buffer::empty(area);
        render_status_content(
            &mut buffer,
            0,
            0,
            25,
            60,
            &config,
            Some(&context),
            &mut status_tab,
            None,
        );
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(text.contains("CPAMC"));
        assert!(text.contains("cuentas"));
        assert!(!text.contains("Centinela"));

        // Disabled
        config.right_dock_show_cpamc = false;
        let mut buffer = Buffer::empty(area);
        render_status_content(
            &mut buffer,
            0,
            0,
            25,
            60,
            &config,
            Some(&context),
            &mut status_tab,
            None,
        );
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(!text.contains("Pool:"));
        assert!(!text.contains("CPAMC / Cuentas"));
    }

    #[test]
    fn status_renders_tools_and_mcp_cards_and_honors_visibility() {
        let mut config = ClientShellConfig::from_config(&crate::config::Config::default());
        let area = Rect::new(0, 0, 60, 30);
        let context = WorkspaceContext {
            workspace_id: "one",
            workspace_name: "test-ws",
            root_path: Path::new("/"),
            branch: Some("main"),
            git_ahead_behind: None,
            is_preview: false,
            agent_status: None,
            agent_name: None,
            agent_tokens: None,
        };
        let mut buffer = Buffer::empty(area);
        let mut status_tab = StatusTab::default();
        render_status_content(
            &mut buffer,
            0,
            0,
            30,
            60,
            &config,
            Some(&context),
            &mut status_tab,
            None,
        );
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(text.contains("Tools"));
        assert!(text.contains("MCP"));

        config.right_dock_show_tools = false;
        config.right_dock_show_mcp = false;
        let mut buffer = Buffer::empty(area);
        render_status_content(
            &mut buffer,
            0,
            0,
            30,
            60,
            &config,
            Some(&context),
            &mut status_tab,
            None,
        );
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(!text.contains("• TOOLS"));
        assert!(!text.contains("• MCP"));
    }

    #[test]
    fn test_status_tab_collapse_toggle_and_shortcuts() {
        let mut status = StatusTab::default();
        // Por defecto sección 3 (Proyecto) y 5 (SDD) están colapsadas
        assert!(status.is_collapsed(3));
        assert!(status.is_collapsed(5));
        assert!(!status.is_collapsed(1));

        // Alternar colapso de sección 1 (Estado)
        status.toggle_section(1);
        assert!(status.is_collapsed(1));
        status.toggle_section(1);
        assert!(!status.is_collapsed(1));

        // Colapsar todo
        status.collapse_all();
        for s in 1..=9 {
            assert!(status.is_collapsed(s));
        }

        // Expandir todo
        status.expand_all();
        for s in 1..=9 {
            assert!(!status.is_collapsed(s));
        }
    }

    #[test]
    fn test_dock_tab_lifecycle_hibernation_and_dirty() {
        let mut dock = RightDockState::default();
        assert_eq!(dock.active_tab, DockTabId::Status);

        // Al cambiar a Files, Files pasa a dirty
        dock.select_tab(DockTabId::Files);
        assert_eq!(dock.active_tab, DockTabId::Files);
        assert!(dock.files_tab.is_dirty());

        // Al cambiar a Changes, Files se hiberna y Changes pasa a dirty
        dock.select_tab(DockTabId::Changes);
        assert_eq!(dock.active_tab, DockTabId::Changes);
        assert!(dock.changes_tab.is_dirty());
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
        assert_eq!(hits.right_dock_tabs[0].1, DockTabId::Status);
        assert_eq!(hits.right_dock_tabs[1].1, DockTabId::Files);
        assert_eq!(hits.right_dock_tabs[2].1, DockTabId::Changes);
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
        assert_eq!(hits.right_dock_tabs[0].1, DockTabId::Status);
        assert_eq!(hits.right_dock_tabs[1].1, DockTabId::Files);
        assert_eq!(hits.right_dock_tabs[2].1, DockTabId::Changes);
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
            (Rect::new(83, 0, 13, 1), DockTabId::Changes),
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
        assert_eq!(state.mode, ClientShellMode::RightDock);

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
}
