use std::path::{Path, PathBuf};

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;

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
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct StatusTab {
    is_dirty: bool,
    pub active_agent_status: Option<String>,
    pub running_jobs_count: usize,
    pub todos_summary: Option<String>,
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

/// Pestaña 2: Files (Explorador perezoso con caché bajo demanda)
#[derive(Debug, Default)]
pub struct FilesTab {
    is_dirty: bool,
    cached_root: Option<PathBuf>,
    pub entries: Vec<(String, bool)>, // (Nombre, es_directorio)
    pub selected_index: usize,
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
    pub fn refresh_if_needed(&mut self, root: &Path) {
        if !self.is_dirty && self.cached_root.as_deref() == Some(root) {
            return;
        }

        self.cached_root = Some(root.to_path_buf());
        self.entries.clear();

        if let Ok(dir_entries) = std::fs::read_dir(root) {
            let mut list = Vec::new();
            for entry in dir_entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with('.') && name != ".agents" && name != ".github" {
                    continue;
                }
                let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                list.push((name, is_dir));
            }
            // Ordenar: primero carpetas alfabéticamente, luego archivos
            list.sort_by(|(name_a, dir_a), (name_b, dir_b)| {
                dir_b.cmp(dir_a).then_with(|| name_a.cmp(name_b))
            });
            list.truncate(30); // Limitar a las primeras 30 entradas por seguridad de render
            self.entries = list;
        }
        self.is_dirty = false;
    }

    pub fn selected_entry(&self) -> Option<&(String, bool)> {
        self.entries.get(self.selected_index)
    }

    pub fn select_next(&mut self) {
        if !self.entries.is_empty() && self.selected_index + 1 < self.entries.len() {
            self.selected_index += 1;
        }
    }

    pub fn select_prev(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
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

/// Estado global del Right Dock persistido en ClientShellState
#[derive(Debug, Default)]
pub struct RightDockState {
    pub active_tab: DockTabId,
    pub layout_mode: DockLayoutMode,
    pub status_tab: StatusTab,
    pub files_tab: FilesTab,
    pub changes_tab: ChangesTab,
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

    /// Sincronización reactiva ante cambio de workspace (Punto 7)
    pub fn sync_workspace(&mut self, current_workspace_id: &str) {
        if self.last_workspace_id.as_deref() != Some(current_workspace_id) {
            self.last_workspace_id = Some(current_workspace_id.to_string());
            self.status_tab.mark_dirty();
            self.files_tab.mark_dirty();
            self.changes_tab.mark_dirty();
            self.files_tab.selected_index = 0;
            self.quick_peek = None; // Cierra cualquier preview si cambiaste de workspace
        }
    }
}

pub(super) fn render_right_dock(
    buffer: &mut Buffer,
    area: Rect,
    config: &ClientShellConfig,
    dock_state: &mut RightDockState,
    context: Option<&WorkspaceContext>,
) {
    if area.is_empty() {
        return;
    }

    // Sincronización atómica de contexto si cambia el workspace (Punto 7)
    if let Some(ctx) = context {
        dock_state.sync_workspace(ctx.workspace_id);
    }

    // Fondo del dock
    buffer.set_style(area, Style::default().bg(config.palette.panel_bg));

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
            let status_color = if active == DockTabId::Status { config.palette.accent } else { config.palette.subtext0 };
            let files_color = if active == DockTabId::Files { config.palette.accent } else { config.palette.subtext0 };
            let changes_color = if active == DockTabId::Changes { config.palette.accent } else { config.palette.subtext0 };

            if area.height >= 2 {
                buffer.set_string(icon_x, area.y + 1, "⚡", Style::default().fg(status_color));
            }
            if area.height >= 4 {
                buffer.set_string(icon_x, area.y + 3, "📁", Style::default().fg(files_color));
            }
            if area.height >= 6 {
                buffer.set_string(icon_x, area.y + 5, "±", Style::default().fg(changes_color));
            }
        }
        return;
    }

    // Modo Expandido: Tab Header en la primera fila con chips diferenciados (Punto 8)
    let content_x = area.x + 1;
    let tab_y = area.y;

    let active_style = Style::default()
        .fg(config.palette.panel_bg)
        .bg(config.palette.accent);
    let inactive_style = Style::default()
        .fg(config.palette.subtext0)
        .bg(config.palette.surface0);

    let status_style = if dock_state.active_tab == DockTabId::Status { active_style } else { inactive_style };
    let files_style = if dock_state.active_tab == DockTabId::Files { active_style } else { inactive_style };
    let changes_style = if dock_state.active_tab == DockTabId::Changes { active_style } else { inactive_style };

    buffer.set_string(content_x, tab_y, " ⚡ Status ", status_style);
    if content_x + 11 < area.right() {
        buffer.set_string(content_x + 11, tab_y, " 📁 Files ", files_style);
    }
    if content_x + 22 < area.right() {
        buffer.set_string(content_x + 22, tab_y, " ± Changes ", changes_style);
    }

    // Indicador Split View en el extremo derecho del header (Punto 5)
    if area.width > 36 {
        let split_label = match dock_state.layout_mode {
            DockLayoutMode::SingleTab => "[⊞ Split]",
            DockLayoutMode::SplitVertical => "[⊟ Single]",
        };
        let split_style = if dock_state.layout_mode == DockLayoutMode::SplitVertical {
            Style::default().fg(config.palette.peach).bg(config.palette.surface0)
        } else {
            Style::default().fg(config.palette.subtext0)
        };
        buffer.set_string(area.right().saturating_sub(10), tab_y, split_label, split_style);
    }

    // Línea separadora horizontal del header
    let header_divider_y = tab_y + 1;
    if header_divider_y < area.bottom() {
        for x in content_x..area.right() {
            if let Some(cell) = buffer.cell_mut((x, header_divider_y)) {
                cell.set_symbol("─");
                cell.set_style(Style::default().fg(config.palette.surface_dim));
            }
        }
    }

    // Renderizado según layout_mode (Punto 5: Split View opcional)
    let body_y = header_divider_y + 1;
    let available_rows = area.bottom().saturating_sub(body_y);

    if dock_state.layout_mode == DockLayoutMode::SplitVertical && available_rows >= 14 {
        let half_rows = available_rows / 2;
        render_status_content(buffer, content_x, body_y, half_rows, area.width, config, context);

        let split_divider_y = body_y + half_rows;
        for x in content_x..area.right() {
            if let Some(cell) = buffer.cell_mut((x, split_divider_y)) {
                cell.set_symbol("─");
                cell.set_style(Style::default().fg(config.palette.surface_dim));
            }
        }
        let lower_body_y = split_divider_y + 1;
        let lower_rows = area.bottom().saturating_sub(lower_body_y);
        if let Some(ctx) = context {
            dock_state.files_tab.refresh_if_needed(ctx.root_path);
        }
        render_files_content(buffer, content_x, lower_body_y, lower_rows, area.width, config, &dock_state.files_tab);
    } else {
        match dock_state.active_tab {
            DockTabId::Status => {
                render_status_content(buffer, content_x, body_y, available_rows, area.width, config, context);
            }
            DockTabId::Files => {
                if let Some(ctx) = context {
                    dock_state.files_tab.refresh_if_needed(ctx.root_path);
                }
                render_files_content(buffer, content_x, body_y, available_rows, area.width, config, &dock_state.files_tab);
            }
            DockTabId::Changes => {
                render_changes_content(buffer, content_x, body_y, available_rows, area.width, config, context);
            }
        }
    }

    // Si hay un Quick Peek activo, renderizar el modal flotante (Punto 6)
    if let Some(peek) = &dock_state.quick_peek {
        render_quick_peek_modal(buffer, area, config, peek);
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
) {
    if rows < 4 || width < 12 {
        return;
    }

    let ws_name = context.map(|c| c.workspace_name).unwrap_or("Local");
    let branch_name = context.and_then(|c| c.branch).unwrap_or("main");

    buffer.set_string(x + 1, y, "• GENTLE HUD", Style::default().fg(config.palette.accent));
    buffer.set_string(x + 1, y + 1, format!("Workspace: {ws_name}"), Style::default().fg(config.palette.text));
    buffer.set_string(x + 1, y + 2, format!("Branch:    {branch_name}"), Style::default().fg(config.palette.mauve));

    if rows >= 7 {
        buffer.set_string(x + 1, y + 4, "▾ Jobs (0 running)", Style::default().fg(config.palette.subtext0));
        buffer.set_string(x + 1, y + 5, "▾ Todos (100% idle)", Style::default().fg(config.palette.green));
    }
}

fn render_files_content(
    buffer: &mut Buffer,
    x: u16,
    y: u16,
    rows: u16,
    width: u16,
    config: &ClientShellConfig,
    files_tab: &FilesTab,
) {
    if rows < 2 || width < 12 {
        return;
    }

    buffer.set_string(x + 1, y, "• EXPLORER [Space=Peek]", Style::default().fg(config.palette.accent));

    let visible_rows = (rows.saturating_sub(2)) as usize;
    for (i, (name, is_dir)) in files_tab.entries.iter().take(visible_rows).enumerate() {
        let row_y = y + 2 + (i as u16);
        let is_selected = i == files_tab.selected_index;
        let icon = if *is_dir { "📁" } else { "📄" };
        let style = if is_selected {
            Style::default().fg(config.palette.accent).bg(config.palette.surface0)
        } else if *is_dir {
            Style::default().fg(config.palette.blue)
        } else {
            Style::default().fg(config.palette.text)
        };

        let label = format!("{icon} {name}");
        let truncated = if label.chars().count() > (width as usize).saturating_sub(3) {
            let limit = (width as usize).saturating_sub(4);
            format!("{}…", &label[..limit.min(label.len())])
        } else {
            label
        };
        buffer.set_string(x + 1, row_y, truncated, style);
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

    buffer.set_string(x + 1, y, "• GIT CHANGES", Style::default().fg(config.palette.accent));
    if let Some(ctx) = context {
        if let Some((ahead, behind)) = ctx.git_ahead_behind {
            buffer.set_string(
                x + 1,
                y + 2,
                format!("Tracking: ↑{ahead} ↓{behind}"),
                Style::default().fg(config.palette.subtext0),
            );
        } else {
            buffer.set_string(x + 1, y + 2, "Working tree clean", Style::default().fg(config.palette.green));
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
        Style::default().fg(config.palette.accent).bg(config.palette.surface0),
    );

    // Contenido del archivo/log
    let content_area_rows = modal_h.saturating_sub(2) as usize;
    for (idx, line) in peek.lines.iter().skip(peek.scroll).take(content_area_rows).enumerate() {
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
    fn test_quick_peek_scrolling() {
        let mut peek = QuickPeekState::new("sample.rs", vec!["line 1".into(), "line 2".into(), "line 3".into()]);
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
}
