## Workflow Status
- Status: READY
- Blockers: None

## Goal
Plan the first increment of a native, configurable Telar sidebar: existing, confirmed State and Git facts only, with persistent visibility preferences. This is planning, not authorization to implement or modify configuration.

## Scope & Exclusions
- State: project and active agent state only when supported by existing data. Git: branch and clean/dirty only when supported by existing data. Show unconfirmed or unavailable fields as unavailable; never substitute synthetic values. Associate facts with the correct active workspace/session rather than guessing.
- Sidebar, State, and Git default enabled. Visibility preferences persist across restarts. Hiding is presentation only and must not control services or accounts.
- The phase-0 inventory identifies Context, Engram, Integrations/accounts, Antigravity/quotas, Tools, and MCP as pending sections; CPAMC/Auto Switcher presentation is future work. Do not implement those sections, their controls, telemetry or adapters in this increment.
- Exclude new dependencies, services, provider/credential access, management actions, protocol changes, Git mutations, unrelated cleanup and copied external code/assets. Preserve existing dirty work. Full sidebar parity and CPAMC are later work, not acceptance criteria.

## Execution Scope
- Root: /home/wilsonlavio/projects/telar-dev
- Allowed Paths:
  - /home/wilsonlavio/projects/telar-dev/src/**
- Writable Paths:
  - /home/wilsonlavio/projects/telar-dev/src/**
- Allowed Bash:
  - None
- Notes: Proposed future implementation boundary only; no apply or shell execution is authorized by this plan. The current delegation permits writing only this plan. Assigned authority artifacts and skills are separately authorized inputs. Configuration changes, even within src, require explicit separate authorization before implementation; stop if necessary work falls outside src. Validation commands below are proposed, not authorized by this delegation.

## MINI Items

### MINI-001: Native sidebar and truthful projections
- Contract: Reuse existing sidebar and client presentation patterns. Display the approved State and Git fields from existing correctly scoped data, with unavailable for any unconfirmed or absent field. Keep rendering pure; use existing available state rather than adding an integration or inventing a data source.
- Acceptance: With representative existing data, each field displays its corresponding real value; missing or unconfirmed project, agent state, branch, and clean/dirty independently show unavailable. Switching active context does not attribute another context's data to it. Existing sidebar functionality and narrow-terminal behavior remain usable; no service, account, or Git mutation is triggered by display.
- Canonical sources: AGENTS.md, docs/architecture/telar-canon.md, investigaciones/2026-11-02-1900-sidebar-fase-0/report.md and sources.md; approved delegated user decisions.
- Validation: Inspect existing owning tests and source first; use test-first coverage for populated, missing, context-switching and narrow-layout cases, and negative tests for unintended control or fabricated values. Run the relevant existing focused tests and broader regression checks after separate authorization.
- Depends on: None.

### MINI-002: Visibility and persistence
- Contract: Provide independent visibility controls for sidebar, State and Git using existing settings interaction and persistence mechanisms, subject to separate explicit authorization for configuration changes. Sidebar and both sections default enabled; choices survive restart. Visibility changes affect presentation only.
- Acceptance: Fresh/default configuration enables all three; independently toggling each changes only its intended visibility, and restoring the sidebar retains section preferences. Persisted choices reload after restart; existing unrelated settings and defaults remain intact. Neither toggling nor restarting starts/stops services or changes accounts or Git state.
- Canonical sources: AGENTS.md, docs/architecture/telar-canon.md, investigaciones/2026-11-02-1900-sidebar-fase-0/report.md and sources.md; approved delegated user decisions.
- Validation: Characterize existing settings and persistence tests; apply RED-GREEN-REFACTOR with default, independent toggles, restart round-trip, unrelated-settings preservation and no-service-control cases. Run focused tests and relevant settings/sidebar regressions after separate authorization.
- Depends on: MINI-001; explicit separate authorization for configuration changes before apply.

## Risks & Constraints
- Phase-0 source observations are inspection, not proof of live field availability. If no existing reliable source is confirmed, show unavailable; do not create one. Do not interpret disconnected as idle, or unknown Git status as clean.
- Keep session/runtime facts and client presentation ownership separate; preserve generation-1 codecs, method shapes, identity, restore and existing sidebar behavior. If satisfying acceptance requires a protocol change, migration, dependency, or scope expansion, stop for a user decision.
- No filesystem/process/network work in rendering or pane-scaled layout loops. Identify frequency and cardinality of added work; if render/layout hot paths are widened, profile fixed geometry with 1 and at least 15 populated panes and report the scaling delta, using `just bench-render-scale` when applicable. Deterministic operation/architecture tests supplement performance evidence.
- Reuse the established test framework and colocated owning tests; baseline the narrowest relevant existing tests before changes, distinguish pre-existing failures, then run focused and broader checks. Proposed project checks include `just check` and, when applicable, `just ui-hot-path-architecture-test`; execution and any resulting writes require separate authorization. Do not claim tests or benchmarks ran during planning.
- CPAMC research reports synthetic quota defaults and unverified compatibility: never surface these as real data. Future work requires evidenced source contracts, freshness, security, licensing and explicit scope; it is not implicitly authorized here.

## Open Decisions
None for this bounded first increment. Field availability remains an implementation evidence question with the approved unavailable fallback, not permission to add a source. Configuration implementation requires explicit separate authorization.

## Next Permitted Action
Request explicit authorization for implementation within the stated boundary, including changes to the existing configuration mechanism and necessary validation commands; do not implement before authorization.
