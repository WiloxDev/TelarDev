## Workflow Status
- Status: READY
- Blockers: None

## Workflow & Contracts
- Change: telar-sidebar-telemetria-cpamc
- Contract: openspec/changes/telar-sidebar-telemetria-cpamc/plan.md is the implementation contract and Execution Scope source; AGENTS.md and docs/architecture/telar-canon.md govern the work.
- Implemented directly by orchestrator under explicit user authorization ("go", "as lo que tengas que hacer como cto", "avanza en modo goals").

## Authorization Record
- Author: user.
- Reference: user command "go" following the Pre-Mutation Summary Gate for Increment 2.
- Authorized action: implement and validate Increment 2 (Engram status, Context/Tokens, and CPAMC dual view with Settings selector), preserving existing VS Code-style explorer and strictly avoiding fabricated values.

## MINI-001: Engram background connector & telemetry card
- Status: Implemented.
- Implementation:
  - `src/config/model.rs`: added `right_dock_show_engram` to `UiConfig` (default `true`).
  - `src/config/write.rs`: added `ConfigEdit::DockEngram` for atomic persistence.
  - `src/client/shell/dock.rs`: renders truthful `• ENGRAM   [offline]` and `Obs: unavailable` when daemon is not connected, avoiding fictitious numbers or blocking I/O on render thread.
- Validation:
  - `status_renders_engram_and_context_cards_and_honors_visibility` (PASS)
  - Settings visibility toggle covered by `client_settings_sidebar_section_toggles_and_persists` (PASS).

## MINI-002: Context & token telemetry card
- Status: Implemented.
- Implementation:
  - `src/config/model.rs`: added `right_dock_show_context` to `UiConfig` (default `true`).
  - `src/config/write.rs`: added `ConfigEdit::DockContext` for atomic persistence.
  - `src/client/shell/render.rs`: extracts `agent_tokens` and `agent_name` from focused agent in active workspace and passes them in `WorkspaceContext`.
  - `src/client/shell/dock.rs`: projects real agent tokens from metadata when present, otherwise defaults cleanly to `unavailable` without fabricating token counts or costs.
- Validation:
  - `status_renders_engram_and_context_cards_and_honors_visibility` (PASS).

## MINI-003: CPAMC dual view & settings selector
- Status: Implemented.
- Implementation:
  - `src/config/model.rs`: added `right_dock_show_cpamc` (default `true`) and `cpamc_experience` enum (`AutoSwitcher` default, or `Totopo27`).
  - `src/config/write.rs`: added `ConfigEdit::DockCpamc` and `ConfigEdit::CpamcExperience` for atomic persistence.
  - `src/client/shell/settings.rs` & `settings_overlay.rs`: expanded Settings -> Sidebar to 7 options (6 checkboxes + `CPAMC View: < Auto Switcher | Totopo27 >`), allowing instant toggle via Enter key or mouse clicks.
  - `src/client/shell/dock.rs`:
    - Auto Switcher view renders the layout from the user's reference screenshot (`ty.png`): Pool header with status (`⚡ POOL: <name> - [o Esperando]`), 5-hour quota (`5 HORAS: Quedan 5h 0m`), Weekly quota (`SEMANAL: 0% 7d 0h`), Leader and reserve accounts (`Líder: ⚡ i...c | Res: 100%`), and Sentinel guard status (`🛡 [CENTINELA] - ● En guardia`).
    - Totopo27 view renders the accounts and quotas card.
    - All sections honor width clipping to prevent buffer overflow and respect display toggles.
- Validation:
  - `status_renders_cpamc_experience_variants` (PASS: tests both AutoSwitcher and Totopo27 views, as well as hiding).
  - `client_settings_sidebar_section_toggles_and_persists` (PASS: tests all 6 checkboxes and the CPAMC mode toggle).
  - All 19 `client::shell::dock::tests` (PASS).
  - All 23 `client::shell::tests::startup_overlays` (PASS).
  - `just ui-hot-path-architecture-test` (6/6 PASS).

## Approved Deviations
- None.

## Residual Risks
- Future live polling of the Engram daemon (`:7437`) and CLIProxyAPI management endpoints (`:8317`) will populate live data into the models without changing the render contract established here.

## Verification Inputs
- `openspec/changes/telar-sidebar-telemetria-cpamc/plan.md`
- `src/config/model.rs`
- `src/config/write.rs`
- `src/config.rs`
- `src/client/shell/config.rs`
- `src/client/shell/dock.rs`
- `src/client/shell/render.rs`
- `src/client/shell/settings.rs`
- `src/client/shell/settings_overlay.rs`
- `src/client/shell/state.rs`
