## Workflow Status
- Status: READY
- Blockers: None

## Goal
Plan Increment 2 of the native Telar sidebar: integrate background telemetry for Engram (`:7437`), conversation context and tokens, and a dual-experience CPAMC section with selectable Totopo27 vs Auto Switcher views and a Settings selector. This is planning only; implementation requires separate authorization.

## Scope & Exclusions
- **Engram Telemetry**: Background polling of local daemon (`http://127.0.0.1:7437`) for daemon status, observation counts, and sync state. When daemon is unreachable, gracefully display `offline` or `unavailable` without blocking the TUI.
- **Context & Tokens**: Telemetry projection for session tokens, context limit, and progress for the focused agent/pane. When the agent does not emit metrics, display `unavailable`; never fabricate counts or fake costs.
- **CPAMC Section**: Dual presentation selectable in Settings -> Sidebar:
  1. *Totopo27 View*: Accounts, quota bars, and reset timers from CLIProxyAPI management.
  2. *Auto Switcher View*: Content matching the user's reference screenshot (Pool name/state, 5-Hour and Weekly remaining/used windows, Leader and Reserve accounts, and Sentinel status).
- **Truthful Data Fallback**: In accordance with the Phase 0 contract findings (`investigaciones/2026-11-02-1930-cpamc-contratos/`), synthetic defaults (e.g., 95% or 100% placeholders from unverified pollers) must not be presented as real measured facts. Missing or ambiguous data must display as `unavailable` or `unknown`.
- **Read-Only Separation**: Displaying or switching CPAMC views in the sidebar is purely a presentation choice and must NOT trigger account rotations, failovers, credentials mutation, or background service restarts.
- **Exclusions**: No external network calls during render; no hardcoded credentials or leaking secrets in logs/TUI; no disruption to the existing VS Code-style file tree explorer (`FilesTab`).

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
- Notes: Proposed implementation boundary. Writable permissions apply only after explicit user implementation authorization.

## MINI Items

### MINI-001: Engram background connector & telemetry card
- Contract: Provide an asynchronous client querying Engram daemon (`http://127.0.0.1:7437`). Expose snapshot data (online status, observation count, sync status) to `AppState`/`ClientShellState` without performing I/O on the render thread.
- Acceptance: When daemon is active, displays live status and counts; when daemon is absent, displays `offline` or `unavailable` within 1 tick with 0 ms frame drop. Toggleable in Settings -> Sidebar (`[x] Engram`).
- Validation: Unit tests for connector parser, timeout handling, offline fallback, and render purity.
- Depends on: None.

### MINI-002: Context & token telemetry card
- Contract: Project token consumption, context window limit, and usage percentage for the focused pane/agent from session metadata.
- Acceptance: Real token metrics display when reported by agent telemetry. Missing data displays `unavailable`. Never invents token numbers or estimated costs. Toggleable in Settings -> Sidebar (`[x] Context`).
- Validation: Test-first verification with populated agent metadata and negative tests for missing/unreported metrics.
- Depends on: None.

### MINI-003: CPAMC dual view & settings selector
- Contract: Extend Settings -> Sidebar with a selector `CPAMC Experience: [ Totopo27 | Auto Switcher ]` (persisted in `config.toml`) and section toggle `[x] CPAMC`.
- Acceptance:
  - Switching selector alternates between the Totopo27 quota layout and the Auto Switcher pool/quota/sentinel layout.
  - Auto Switcher view renders: Pool header, 5-hour and weekly remaining bars, leader/reserve indicators, and Sentinel status.
  - Absence or failure of backend endpoints displays `unavailable` without crash.
  - Synthetic 95%/100% default values are treated as unverified/unknown.
  - Switching views does not trigger API rotations or mutate accounts.
- Validation: End-to-end tests for Settings choice selection, persistence round-trip, view rendering under both modes, and error state handling.
- Depends on: MINI-001, MINI-002.

## Risks & Constraints
- Render loop purity: All HTTP/socket communications with Engram (`:7437`) and CLIProxyAPI/Auto Switcher (`:8317`) must happen in background async tasks, updating atomic snapshots in memory.
- UI scaling: Adding new collapsible cards must respect vertical terminal boundaries and narrow layouts without clipping the explorer.
- Concurrency & Authority: Preserving single authority over account switching; Telar remains an observer and HUD, not an unsolicited rotation engine.

## Open Decisions
None. Scope, layout references, and data truthfulness rules are established by user direction and Phase 0 research.

## Next Permitted Action
Request explicit user authorization to implement Increment 2 (`telar-sidebar-telemetria-cpamc`) per the plan.
