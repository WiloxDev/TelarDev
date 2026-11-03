## Workflow Status
- Status: READY
- Blockers: None

## Goal
Plan Increment 3 of the native Telar sidebar: integrate the remaining two cards from the hudr specification into the right dock:
1. **Herramientas (Tool Calls)**: Real-time aggregated tool call statistics (reads, writes, bash commands, memory ops) for the active agent.
2. **Servidores MCP (MCP Servers)**: Active MCP servers status and connectivity indicators.
Both sections will have independent visibility checkboxes in **Settings -> Sidebar** and atomic persistence in `config.toml`.

## Scope & Exclusions
- **Herramientas Card**: Display tool invocation metrics when reported by the focused agent session. Fall back gracefully to `unavailable` or `0` without fabricating calls or guessing actions.
- **MCP Card**: Display connected MCP servers when reported by session environment or metadata. Fall back to `none` or `unavailable` without fabricating servers or leaking sensitive URLs.
- **Settings Controls**: Add `[x] Tools` and `[x] MCP` checkboxes to **Settings -> Sidebar**, expanding the section list while maintaining the `CPAMC View` toggle.
- **Persistence**: Persist `right_dock_show_tools` and `right_dock_show_mcp` in `config.toml` under `[ui]`.
- **Exclusions**: No external network or filesystem I/O on the render thread; no disruption to the existing VS Code-style explorer (`FilesTab`).

## Execution Scope
- Root: `/home/wilsonlavio/projects/telar-dev`
- Allowed Paths:
  - `/home/wilsonlavio/projects/telar-dev/src/**`
- Writable Paths:
  - `/home/wilsonlavio/projects/telar-dev/src/**`
- Allowed Bash:
  - `just test-one *`
  - `just check`
  - `just ui-hot-path-architecture-test`

## MINI Items

### MINI-001: Herramientas (Tools) & MCP Telemetry Cards in Dock
- Contract: Extend `render_status_content` in `src/client/shell/dock.rs` to render:
  - `• TOOLS` with counters for reads, writes, bash, and memory ops when available, otherwise `unavailable`.
  - `• MCP SERVERS` with active server counts/names or `unavailable`.
  Both cards honor bounded row and column constraints.
- Acceptance: Each card renders correctly with real or truthful fallback data and can be hidden independently.
- Validation: Targeted tests in `dock.rs` asserting card rendering and independent visibility.
- Depends on: None.

### MINI-002: Settings -> Sidebar Checkboxes & Persistence
- Contract: Add `right_dock_show_tools` and `right_dock_show_mcp` to `UiConfig` and `ClientShellConfig` (defaults `true`), add `ConfigEdit::DockTools` and `ConfigEdit::DockMcp` in `write.rs`, and update Settings overlay navigation to include 8 checkboxes + 1 CPAMC view selector (9 total choices).
- Acceptance: All checkboxes can be toggled via Enter or mouse clicks; choices persist across config reload and save round-trips.
- Validation: Unit and E2E tests in `src/config/write.rs` and `src/client/shell/tests/startup_overlays.rs`.
- Depends on: MINI-001.

## Risks & Constraints
- Overlay geometry: Expanding to 9 choices requires sufficient vertical content area in Settings overlay so all choices remain visible and navigable.
- Hot-path purity: No allocation or string format loops inside the render pipeline.

## Next Permitted Action
Execute implementation directly per authorization.
