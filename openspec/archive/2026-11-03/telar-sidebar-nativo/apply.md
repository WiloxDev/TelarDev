## Workflow Status
- Status: READY
- Blockers: None

## Workflow & Contracts
- Change: telar-sidebar-nativo. Plan artifact openspec/changes/telar-sidebar-nativo/plan.md is the implementation contract and Execution Scope source; AGENTS.md and docs/architecture/telar-canon.md govern the work.
- Implemented directly by orchestrator under explicit user authorization ("as lo que tengas que hacer como cto", "avanza en modo goals", "continue").

## Authorization Record
- Author: user.
- Reference: explicit commands "go", "as lo que tengas que hacer como cto", "avanza en modo goals", "continue".
- Authorized action: implement and validate first increment of native configurable sidebar integrated into existing right dock, including necessary configuration changes and validation, preserving existing VS Code-style explorer.

## MINI-001: Native sidebar and truthful projections
- Status: Implemented.
- Implementation:
  - `src/client/shell/dock.rs`: eliminated fictitious Jobs (0 running) and Todos (100% idle) labels and runtime filesystem probing during render. Renders Workspace (project name), Agent (focused pane agent status), Branch, and explicit "Git: unavailable" without guessing. Bounded text width to prevent buffer overflow.
  - `src/client/shell/render.rs`: associates agent status with active workspace and focused pane, passing truthful `agent_status` in `WorkspaceContext`.
  - Preserved existing VS Code-style file tree explorer (`FilesTab`), dirty tracking, hibernation and quick peek.
- Validation:
  - `status_does_not_show_fabricated_jobs_or_todos` (PASS)
  - `status_shows_workspace_agent_and_branch_without_guessing_git_state` (PASS)
  - `status_sections_can_be_hidden_independently` (PASS)
  - All 17 `client::shell::dock::tests` (PASS)

## MINI-002: Visibility and persistence
- Status: Implemented.
- Implementation:
  - `src/config/model.rs`: added `right_dock_enabled`, `right_dock_show_state`, `right_dock_show_git` to `UiConfig` (all default `true`).
  - `src/config/write.rs`: added `ConfigEdit::DockVisible`, `ConfigEdit::DockState`, `ConfigEdit::DockGit` for atomic persistence in `config.toml`.
  - `src/client/shell/config.rs`: updated layout calculation so `!right_dock_enabled` allocates 0 columns (completely hides dock instead of rail mode), and live reload applies section flags.
  - `src/client/shell/settings.rs` & `settings_overlay.rs`: added "Sidebar" tab in Settings overlay after Integrations, rendering checkboxes `[x] Right dock`, `[x] State`, `[x] Git`.
  - `src/client/shell/mouse.rs`: enabled immediate mouse click toggling on sidebar checkboxes.
- Validation:
  - `disabled_right_dock_takes_no_space_even_in_rail_mode` (PASS)
  - `dock_sections_default_enabled_and_persist_independently` (PASS)
  - `client_settings_sidebar_section_toggles_and_persists` (PASS)
  - All 23 `client::shell::tests::startup_overlays` (PASS)
  - `just ui-hot-path-architecture-test` (6 tests PASS)

## Approved Deviations
- None. Full HUD parity (Context, Engram, Quotas, Tools, MCP) and CPAMC/Auto Switcher selection remain deferred to subsequent increments per plan.

## Residual Risks
- CPAMC/Auto Switcher data contracts require future implementation increments as scheduled in the roadmap.
- Multiplicative render benchmark (`just bench-render-scale`) contains one pre-existing headless native graphics test failure requiring host CoW retention, unrelated to dock rendering.

## Verification Inputs
- `openspec/changes/telar-sidebar-nativo/plan.md`
- `src/client/shell/dock.rs`
- `src/client/shell/render.rs`
- `src/client/shell/config.rs`
- `src/client/shell/settings.rs`
- `src/client/shell/settings_overlay.rs`
- `src/client/shell/mouse.rs`
- `src/config/model.rs`
- `src/config/write.rs`
