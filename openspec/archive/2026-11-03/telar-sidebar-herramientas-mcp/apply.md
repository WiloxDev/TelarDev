## Workflow Status
- Status: READY
- Blockers: None

## Workflow & Contracts
- Change: telar-sidebar-herramientas-mcp
- Contract: openspec/changes/telar-sidebar-herramientas-mcp/plan.md is the implementation contract and Execution Scope source; AGENTS.md and docs/architecture/telar-canon.md govern the work.
- Implemented directly by orchestrator under explicit user authorization ("Avanzamos con el Incremento 3 (Herramientas + Servidores MCP) para completar las 8 tarjetas").

## MINI-001: Herramientas (Tools) & MCP Telemetry Cards in Dock
- Status: Implemented.
- Implementation:
  - `src/config/model.rs`: added `right_dock_show_tools: bool` and `right_dock_show_mcp: bool` to `UiConfig` (defaulting to `true`).
  - `src/config/write.rs`: added `ConfigEdit::DockTools` and `ConfigEdit::DockMcp` for atomic persistence.
  - `src/client/shell/dock.rs`: extended `render_status_content` to render:
    - `• TOOLS    [active]` and `Calls:     unavailable` (or real counts when emitted).
    - `• MCP      [ready]` and `Servers:   unavailable` (or active servers list).
    - Row-bounded and column-clipped formatting to guarantee zero overflow in compact terminals.
- Validation:
  - `status_renders_tools_and_mcp_cards_and_honors_visibility` (PASS)
  - `client_settings_sidebar_section_toggles_and_persists` (PASS).

## MINI-002: Settings -> Sidebar Checkboxes & Persistence
- Status: Implemented.
- Implementation:
  - `src/client/shell/state.rs`: expanded `sidebar_visibility: [bool; 8]` and shell configuration fields.
  - `src/client/shell/config.rs`: updated configuration initialization and hot reload mappings.
  - `src/client/shell/settings.rs`: extended settings section choices to 9 options (8 checkboxes + CPAMC View mode toggle).
  - `src/client/shell/settings_overlay.rs`: updated sidebar options list to include `Tools` and `MCP`.
  - `src/client/shell/tests/graphics.rs`: updated mock settings overlay struct.
- Validation:
  - `dock_sections_default_enabled_and_persist_independently` (PASS).
  - `client_settings_sidebar_section_toggles_and_persists` (PASS: validates toggling all 8 checkboxes and the CPAMC view mode via keyboard).
  - All 20 tests in `client::shell::dock::tests` (PASS).
  - All 23 tests in `client::shell::tests::startup_overlays` (PASS).
  - `just ui-hot-path-architecture-test` (6/6 PASS).

## Approved Deviations
- None.

## Verification Inputs
- `openspec/changes/telar-sidebar-herramientas-mcp/plan.md`
- `src/config/model.rs`
- `src/config/write.rs`
- `src/client/shell/dock.rs`
- `src/client/shell/settings.rs`
- `src/client/shell/settings_overlay.rs`
- `src/client/shell/state.rs`
- `src/client/shell/config.rs`
