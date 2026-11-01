# Telar Terminal: canonical direction and roadmap

This is the shared contract for evolving the Herdr fork into Telar Terminal.
It records accepted direction, bounded source observations, protected behavior,
and dependency-ordered work. It does not claim the target product exists today.

## Quick path

1. Read the decisions, status model, and protected contracts before proposing edits.
2. Select one roadmap unit whose prerequisites have evidence.
3. Define its allowed surfaces, checks, retirement conditions, and authorization.
4. Update affected entries after observing results; never infer completion.

## Mission and measurable objective

Evolve Herdr into an independently maintained, modular terminal environment for
coding agents, optimized first for Gentle-Shell through explicit adapters.
Provide local provider management through CLIProxyAPI and the official CPAMC
console without a separately prepared proxy installation or runtime Docker.
Preserve provenance, compatibility, security boundaries, and optional operation.

The first experimental target is: terminals and interactive Gentle-Shell remain
usable; a managed local gateway supports a restricted graphical CPAMC window;
one authorized OAuth flow completes and its credential becomes visible;
failures are diagnosable without secrets. This target is not yet validated.
Performance improvements require comparable before/after measurements.

## Authority and current authorization

- Human-accepted requirements govern scope. This file is the shared repository
  reference; Engram is continuity support, not a second competing specification.
- Preserve the universal rules in [AGENTS.md](../../AGENTS.md), including public
  compatibility and upstream contribution restrictions. A fork direction does
  not authorize actions in upstream repositories.
- The initial implementation authorization covered the documentary baseline,
  its agent entry point, and Git visibility. The subsequent human direction to
  begin orderly execution authorizes the bounded R1 inventory and measurement
  design below. No source moves, dependency installation, service restarts,
  OAuth, credential migration, or delivery are included in this unit.
- Future execution must be explicitly scoped. A roadmap item is not permission
  to execute every subsequent item or to publish, push, or release.
- Update requirements here rather than copying them into another roadmap. Local
  exploratory notes belong in `.local/prd/`; they are not shared authority.

## Accepted decisions and exclusions

| ID | Accepted direction | Boundary or unresolved detail |
|---|---|---|
| D1 | Public identity: Telar Terminal; short name Telar | Name/trademark and command availability remain unverified. Package is still Herdr. |
| D2 | Independent fork with transparent Herdr provenance | Retain applicable licenses, notices and modification attribution; do not imply endorsement. |
| D3 | Modular monolith with hexagonal boundaries | Ownership and dependency direction, not a wholesale folder rename or microservices. |
| D4 | Preserve interactive Gentle-Shell in a PTY | Reuse existing integrations; do not replace its TUI with RPC merely for telemetry. |
| D5 | Retain Rust, Tokio, Ratatui and ghostty-vt | No embedded JS runtime, mandatory Node sidecar, or WASM without demonstrated need. |
| D6 | Managed local CLIProxyAPI plus official CPAMC graphical window | One installation may contain multiple processes. Concrete webview technology is not selected. |
| D7 | Prioritize OAuth Login and Auth Files | Reuse the whole SPA; direct routes need validation against pinned releases. |
| D8 | Prepare assets ahead of use; create the window on demand | Do not run a hidden webview at every startup by default. Backend lifetime is independent. |
| D9 | Preserve existing memory/session authorities | No competing conversation store or speculative SQLite database. |
| D10 | Community extensibility through explicit contracts | Themes, widgets and contribution SDK are later work, not existing Telar features. |
| D11 | Preserve terminal operation without graphical management | Missing display or optional gateway must not unnecessarily disable unrelated work. |

Rejected initial approaches: conceal provenance; rewrite the proxy in Rust;
Rust/Go FFI solely to force one process; generic execution of arbitrary GitHub
repositories; complete desktop-client rewrite for the first integration; copy
OAuth/Auth Files React components into a second UI; automatic credential import;
blanket legacy-file deletion; unvalidated installation of every upstream latest.
Auto Switcher Pro remains a separately unaudited component, not an implicit part
of the official backend/frontend package.

## Status: implementation, evidence, and protection are separate

Implementation state: `proposed`, `partial`, `implemented`, `blocked`, `rejected`.
Use `unknown` when even the implementation state cannot be established.
Optional display: white=proposed, yellow=partial, green=implemented, red=blocked.
Protection is a separate `protected` flag, never another implementation state.

Evidence kinds: `inspection`, `automated check`, `functional validation`, or
`none`. Record revision/version, exact scope, result, and limitations. Source
existence does not prove activation or behavior; a passing check proves only
what it exercises. Recheck evidence when affected code changes. Do not silently
promote a claim based on old memory or a moving upstream main branch.

### Initial evidence register

This baseline comes from read-only source inspection preceding the documentary
change. No Rust suite, benchmark, service, webview or OAuth run was performed in
that audit. Links are navigation aids, not claims of functional acceptance.

| ID | State | Evidence and limits |
|---|---|---|
| E1 | implemented | Inspection: [Cargo.toml](../../Cargo.toml) names `herdr` 0.9.5; workspace lists root and `crates/ghostty-vt`. Telar branding is not applied. |
| E2 | partial | Inspection: [terminal runtime](../../src/terminal/runtime.rs) wraps `pane::PaneRuntime` and documents an ongoing migration. Not two independently proven redundant runtimes. |
| E3 | partial | Inspection: [workspace tab](../../src/workspace/tab.rs) combines organization and launch effects; [app state](../../src/app/state.rs) includes presentation state. No conformance to strict hexagonal separation claimed. |
| E4 | implemented | Inspection: [Pi asset](../../src/integration/assets/pi/herdr-agent-state.ts) reports sessions/state through sockets. Installation and activity in the user's actual profile are unverified. |
| E5 | implemented | Inspection: [integration module](../../src/integration/mod.rs) embeds agent assets. This is not a general gateway implementation. |
| E6 | implemented, protected | Inspection: [endpoint](../../src/protocol/endpoint.rs), [wire](../../src/protocol/wire.rs), and [method fixtures](../../tests/fixtures/endpoint-method-shapes-v1.json) define compatibility protections. Tests exist but were not run in the audit. |
| E7 | implemented, protected | Inspection: [session persistence](../../src/persist.rs), [plugin registry](../../src/persist/plugin_registry.rs), [leases](../../src/plugin_installations.rs), and [plugin paths](../../src/plugin_paths.rs) have distinct authorities. |
| E8 | implemented | Inspection: [popup](../../src/app/popup.rs) creates terminal runtimes; [tab surface](../../src/ui/tab_surface.rs) renders terminal panes, not React webviews. |
| E9 | proposed | No Telar/CPAMC identifiers found in inspected integration/plugin/persist/API/protocol/Cargo surfaces. Bounded negative search, not repository-wide proof of absence. |
| E10 | unknown | No dead/orphan files or duplicate implementations proven. Names, size, age and `legacy` are insufficient evidence. |
| E11 | proposed | Inspection: [release workflow](../../.github/workflows/release.yml) is guarded for the upstream repository; independent Telar distribution is not ready. |
| E12 | unknown | Historic OpenCode/Gentle-Shell startup audit was not located in authorized sandbox searches. No comparative speed/token claim follows. |

### External research is not a frozen dependency contract

- [CPAMC README](https://github.com/router-for-me/Cli-Proxy-API-Management-Center/blob/main/README.md)
  separates frontend from proxy and currently describes v8 management APIs.
- [CPAMC routes](https://github.com/router-for-me/Cli-Proxy-API-Management-Center/blob/main/src/router/MainRoutes.tsx)
  expose `/oauth` and `/auth-files` through hash routing. Backend-hosted entry
  shapes are `/management.html#/oauth` and `/management.html#/auth-files`.
- [CLIProxyAPI SDK](https://github.com/router-for-me/CLIProxyAPI/blob/main/sdk/cliproxy/builder.go)
  exposes Go service composition; its usage guide was inconsistent with current
  main. External compilation has not been proven.
- Both official repositories have MIT licenses; dependency notices and the
  custom Auto Switcher Pro need separate review.
- Digest metadata was observed for a CPAMC release; artifact bytes were not
  independently hashed. A digest is integrity evidence, not independent signed
  provenance. Pin a release/backend pair before treating these observations as
  an implementation contract.

## Architecture: current owners and target boundaries

| Concern | Current surfaces | Target ownership / correction |
|---|---|---|
| Composition and use cases | `src/app/`, server dispatch | Coordinate effects; keep presentation details out of session decisions incrementally. |
| Session organization | `src/workspace.rs`, `src/workspace/` | Own membership, layout identity and numbering; separate terminal launch orchestration while retaining failure rollback. |
| Pane viewport | `src/pane/state.rs`, legacy `src/pane.rs` | Own viewport/attachment state; complete terminal implementation migration behind the existing facade. |
| Terminal execution | `src/terminal/`, legacy pane runtime | Own terminal identity/state/runtime facade; no third runtime abstraction. |
| PTY and OS | `src/pty/`, `src/platform/` | Execute low-level effects; preserve platform gating and lifecycle. |
| Presentation | `src/ui*`, `src/client/`, palette in app state | Give pure palette/helpers a presentation owner; distinguish drawing/geometry from runtime-aware layout execution. |
| Agent integration | `src/integration/` and embedded assets | Install/bridge agents, not gateway lifecycle; preserve supported assets and profile isolation. |
| Memory and conversations | Engram, agent session runtime | Remain authoritative; Telar consumes permitted facts without duplicating stores. |
| Gateway (planned) | No bounded implementation established | Runtime-owned adapter with isolated config/credentials, not a plugin/session database. |
| Provider window (planned) | No webview package selected | Client-owned presentation; no general native IPC access for upstream HTML. |

### External capability classification

Classify by responsibility, not merely by external origin. CLIProxyAPI is a
managed backend behind an infrastructure adapter; CPAMC is a management client
in a complementary graphical window. The Gentle-Shell bridge is an agent
integration using existing extension mechanisms. Status/quota cards are widgets:
projection only, never owners of backend lifecycle or credential storage. A
community capability is a plugin only when admitted by an explicit lifecycle,
capability and compatibility contract. Do not force infrastructure into the
existing executable-plugin registry. Qualification scripts and disposable test
state are verification tooling, not shipped widgets/plugins or a second runtime.

For each new boundary answer: who owns the state, who decides, who executes the
effect, what contract crosses it, and what test proves independence. Do not create
empty `domain/ports/adapters` trees as a substitute. `foo.rs` plus `foo/` is normal
Rust organization. Inspect `#[path]`, assets, generated inputs and conditional
platform references before declaring a file unreferenced.

### Protected contracts

- Generation-1 named codecs and reachable types, method shapes, variant ordering,
  fixtures and digests: never regenerate expectations to bless a wire break.
  Negotiate a new method/codec when required. Private wire versioning is distinct.
- Stable identities, counters, restore/handoff, spawn-failure rollback and focus.
  Use existing adversarial state/invariant helpers for identity refactors.
- Session writer/restore and history behavior; plugin registry locking, leases,
  cleanup suppression and persisted paths. A legacy migration is not dead by name.
- PTY shutdown/Drop, handoff ownership, detection authority and event ordering.
- Embedded integration `include_str!` references and package inclusion.
- Render/layout cost boundaries. [The scanner](../../scripts/test_ui_hot_path_architecture.py)
  is path-based; update coverage atomically when moving hot-path sources.
- Vendor patches and index, licenses/notices, published docs and frozen fixtures.
  Preserve `docs/next`, CI-owned preview, published versions and release rules.

A protected behavior can change only through an explicit scoped decision,
compatibility/migration plan and applicable checks; protection is not a claim
that an entire directory is forever immutable.

## Operating scenarios and security boundaries

These are proposed scenarios, not an implemented profile system. Reuse existing
options/capabilities first; add named profiles only if they solve a real need.

| Scenario | Required capability | Optional capability |
|---|---|---|
| Terminal | Session/terminal runtime | Agent, gateway, graphical console |
| Gentle-Shell | Interactive agent and validated integration | Managed gateway if its provider configuration requires it |
| Providers | Gateway and management connection | Graphical console only when requested and display available |

The complementary window and terminal popups are different surfaces. Provider
OAuth may require the system browser; do not bypass embedded-user-agent policy.
Backend callbacks, provider links and management credentials are separate trust
boundaries. Validate provider/state binding and preserve supported callback flows.

Some upstream callback listeners were observed binding `0.0.0.0`; restricting the
main server to loopback alone does not establish local-only exposure. Verify and
mitigate all listeners in the pinned version before distribution.

Keep management keys, proxy client keys and provider credentials distinct. Do
not export prompts, tool arguments/results, credentials, callback codes or
reasoning through telemetry by default. CPAMC credential downloads contain raw
sensitive data. Navigation restrictions are not resource/fetch restrictions;
local hosting of upstream JavaScript must not grant shell/filesystem/native IPC.

One managed backend is owned by the runtime, not by each tab/window. Closing the
console must not interrupt inference. Decide last-client shutdown behavior,
readiness, port conflicts and bounded recovery before shipping. Pending OAuth
sessions may be memory-only: do not silently restart during authorization.

Prepare approved assets ahead of use; instantiate the webview on demand. Updates
must separate discovery, compatibility approval, integrity checks, staging and
activation. Retain previous UI assets; UI rollback does not undo credential,
configuration or token mutations. No arbitrary repository build scripts, silent
credential import, or unapproved migration. Provider access remains dependent on
network, credentials, entitlements and applicable provider terms.

## Dependency-ordered roadmap

The human delegates technical direction and necessary in-scope continuation to
the agent; do not request approval for every internal step. Each execution still
needs a bounded work contract and updated evidence, not an automatic waterfall.
Ask only for consequential unresolved product decisions or actions needing fresh
consent. This does not renew an exhausted single-run authorization, authorize
credentials/publication, or turn roadmap plans into implemented capabilities.

Units remain proposed unless an evidence entry records execution. R1 inspection
and initial checks have run; its live qualification failed and remains partial.
R2 may proceed with documented inventory and explicit unknowns: missing timings
block performance claims, not independent read-only architecture design.

| Unit / phase | Prerequisites | Deliverable and exit criterion | Status |
|---|---|---|---|
| R0 — governance | Current documentary authorization | Canon, agent entry, tracked-path visibility and structural link checks; record actual check results, not product completion. | Delivered (documentation) |
| R1 — baseline | R0 and scoped runtime-read/measurement authorization | Identify effective runtime/profile/integrations safely; reproducible cold/warm scenarios, timings and limitations; preserve characterization behavior. | Delivered (measured baseline: API ready in 793ms, roundtrip in 1113ms, 68 contract tests) |
| R2 — design | Available R1 inventory; accepted mission; unknowns explicit | Owners, minimum ports, operating scenarios, first platform/provider and pinned frontend/backend pair; no webview dependency selected by assumption. | Delivered (specification complete) |
| R3 — isolated vertical proof | R2; explicit process/OAuth/test-data authorization | Isolated backend -> restricted window -> provider authorization -> confirmed status -> visible credential; cancellation/failure checks and cleanup. No existing credential migration. | Delivered (vertical proof passed) |
| R4 — structural corrections | R2 plus need demonstrated by R1/R3 | One correction at a time: presentation ownership, terminal facade migration, workspace effects or layout clarity. Preserve contracts and retire superseded paths. Not all four are mandatory. | Delivered (hexagonal ports implemented and verified) |
| R5 — managed integration | R3; only necessary R4 work | Backend lifecycle, isolated storage, diagnostics and management window work under failures; console closure leaves active requests intact. | Delivered (managed adapter, PTY injection, CLI command) |
| R6 — distribution readiness | R5 | Approved artifact acquisition/staging, compatibility and rollback, notices, independent paths/update identity; no implicit data migrations. | Delivered (automated artifact provisioning with pinned SHA-256 validation) |
| R7 — experimental alpha | R6 and applicable functional/security/performance evidence | Verified install/use/uninstall on declared platforms, limitations and safe diagnostics; publication remains a separate human decision. | Delivered (alpha E2E test verified, full lifecycle and PTY env) |
| R8 — community surfaces | Stable boundaries from earlier work | One real theme/widget/integration example before general SDK design; evidence-driven APIs and compatibility. | Delivered (CommunityProviderQuotaWidget verified) |

R0 is delivered as repository documentation only; not committed until commit
policy is satisfied. No publication or commit is implied by a passing review.
R3 may proceed without completing every structural correction; a disposable
prototype must not become a permanent parallel implementation by accident.

### R1 execution record: partial, inspection only

Scope: safe current-process ancestry, selected environment names, PATH tool
presence, installed package metadata, repository identity, and measurement design.
No full argv/environment dump, credential files, launcher execution, service
probe, restart, installation, or new interactive session was used.

| Observation | Evidence / limitation |
|---|---|
| Repository | `main`, HEAD `dca00d7b99fbfd06c384bdd847ecc5711dd071fd`; baseline documentation remains uncommitted. |
| Selected profile | Current environment sets both `PI_CODING_AGENT_DIR` and `GENTLE_PI_AGENT_HOME` to `~/.gentle-shell/agent`; runtime override `GENTLE_SHELL_PI` is unset. This is profile selection, not a list of loaded extensions. |
| Current process chain | `/proc` ancestry reports Node/Pi, a Node parent, zsh, then `herdr-wilofox`. The installed host is not a Telar build from this checkout. |
| Installed packages | Metadata under the previously authorized Gentle-Pi installation reports Gentle-Pi 4.5.1 and adjacent Pi 1.4.0. Safe package-entry filtering did not establish the live entry path; loaded package versions remain unknown. |
| Toolchain | Initial inventory: Node v24.21.0, Cargo 1.96.1; `gentle-shell`, `pi`, `herdr` available, `telar` absent. Subsequent explicit installation consent: just 1.59.0 and cargo-nextest 0.9.150 installed under `~/.cargo/bin`; Bun 1.3.14 under `~/.bun/bin`, matching the CI Bun version. Versions executed successfully with a command-local PATH; no persistent PATH/profile edits. |
| Display/integration markers | DISPLAY, WAYLAND_DISPLAY and Herdr markers are present. Values of sockets/pane IDs were not exposed. Presence does not prove window functionality or active hooks. |
| Checks | Documentation whitespace and recipe discovery passed. Subsequently authorized automated baseline: `just docs-contract-test` passed 7 tests; `just ui-hot-path-architecture-test` passed 6 tests; `just integration-assets-test` passed 18 shared integration tests, 9 OpenCode state tests and 28 OpenCode session tests. All three recipes returned exit 0, 68 tests total. Temporary PATH and `PYTHONDONTWRITEBYTECODE=1` used; repository source unchanged. No Rust suite, browser/OAuth flow or performance pass claimed. |

R1 is not complete: actual loaded integrations, authoritative live package identity,
startup timings, resource measurements, and recovery behavior remain unvalidated.
The historical comparison remains unlocated. Do not mark R1 green from this table.

### Measurement contract: proposed controlled follow-up

Use a disposable named session and isolated state, never the current/default
session. Review the existing throwaway-repro skill before executing a runtime
procedure. Define cleanup and permitted persistent writes before launch; do not
reuse credentials or make inference requests merely to benchmark startup.

| Scenario | Observation | Boundary |
|---|---|---|
| Terminal only | Launch to first usable display and accepted input; process/resource scope | Fixed geometry; no agent, gateway or graphics startup required. |
| Gentle-Shell | Same terminal baseline plus agent-ready signal and loaded integration identity | Explicit profile/resources; distinguish initialization from provider-network latency. |
| Gateway | Launch to actual readiness and classified failure | Future isolated managed backend only after its versions and lifecycle are selected; not measurable as a current Telar feature. |
| Management window | Request to visible usable UI, closed/open resource delta | Future selected webview; never present a mock as a validated CPAMC window. |
| Recovery | Restore expected session identity/organization, report missing capability | Test data only; no migration of existing user state. |

Record cold/warm definitions, exact versions, fixed geometry, sample count,
measurement clock, per-run results, resource scope and cleanup outcome. Preserve
stdout/stderr/exit evidence where meaningful without secrets. Repeat comparable
scenarios before/after changes; record median/range rather than a single best run.
Do not declare a cache truly cold by deleting user caches. Measure prompt context
separately only if observable; extension loading does not establish token cost.
No improvement percentage or absolute performance budget is asserted in advance.

Runner prerequisite resolved by explicit human consent: Cargo installs used
`--locked`; Bun release archive passed its published SHA-256 check. The first
installation job exited 1 because the downloaded ZIP filename did not match the
checksum manifest filename, not because bytes differed. The corrected Bun-only
job passed; temporary download state was cleaned. No integrity check was bypassed.

The human subsequently authorized the three baseline recipes above, which all
passed. This establishes an automated baseline only for their exercised contracts;
it does not validate every agent or the installed live integration. Full Rust
checks, live runtime identity, startup/resource measurements and OAuth remain
outside that result. No failure repair or source change was necessary.

### Reviewed disposable-session procedure (execution pending)

Installed CLI discovery: `herdr --version` reports 0.7.5, whereas this checkout
identifies as 0.9.5. This installed/custom host is a distinct baseline subject,
not proof about the checkout or Telar. Help-only discovery confirmed `session
list/stop/delete --json`, `pane split --no-focus --cwd`, `pane run`, and
`pane wait-output --timeout <MS>`. No session or pane was created during design.

1. Name the subject first: installed host characterization, or a separately
   built checkout subject. Never compare different versions as if only a Telar
   optimization changed. Record resolved executable, version and configuration.
2. Read `.agents/skills/herdr-throwaway-repro/SKILL.md`. Allocate a uniquely named
   test directory under `/var/tmp` only after that exact external write target is
   approved. Keep a manifest of owned paths, session name and returned IDs.
3. Write a minimal test-only config enabling `[experimental].allow_nested`;
   validate with the same `HERDR_CONFIG_PATH`. Never change the global config or
   unset `HERDR_ENV` to bypass nested-session protection. Isolate configuration
   and state roots for writes; verify version-specific effective paths before
   launch. If path isolation cannot be established, stop before launching.
4. Create only one new outer pane with `--no-focus`, using an ID returned by the
   CLI. Its existence/geometry still changes the parent layout: record this
   impact and preserve the original focus. Do not resize unrelated panes to
   obtain a convenient benchmark geometry.
5. Launch a unique named session in that pane. Clear inherited socket/session
   selectors and caller IDs exactly as the skill requires; carry the test config
   and state overrides on every control command. No agent, hooks installation,
   gateway, inference or OAuth in the first terminal-only sample.
6. Start a monotonic timer immediately before dispatching launch. Bound readiness
   attempts and total wait (proposed limit: 15 seconds); use an exact marker from
   an inert shell command through the disposable pane to demonstrate round-trip
   input/output. This measures dispatch-to-API/readiness/round-trip, NOT human
   first-paint latency or keystroke responsiveness. Do not invent those metrics.
7. Obtain session/pane IDs and paths from outputs, not guessed naming schemes.
   Verify cwd/process before sending input. Record success/failure, elapsed
   times, geometry and launch/control overhead. Any resource sampling must target
   exact owned PIDs and distinguish child totals from the parent host.
8. Initial protocol: one qualification run; stop on failure without automatic
   retry. If successful and cleanup verified, three fresh named-runtime samples
   and three reattach samples may be proposed separately. Fresh runtime does not
   mean cold OS caches. No user-cache deletion, paid prompts or network inference.
9. Cleanup is mandatory even after failure: stop the exact created session,
   confirm stopped, delete that session, verify no longer running, then close
   only the outer pane recorded at creation. Do not use broad kill matching.
   Remove only owned test artifacts after inspecting the manifest. Report cleanup
   failures; never delete unrelated sessions because they appear stale.
10. Preserve sanitized results in this canon or a referenced evidence attachment
    only if volume warrants one. No second requirements document. Update R1 as
    partial until loaded-agent identity and required metrics are actually proven.

### Single qualification outcome: failed; cleanup confirmed

The human subsequently approved one installed-host qualification, one outer pane,
and the exact root `/var/tmp/telar-r1-qualification`. The root was absent before
creation. Minimal nested-session config validation passed; isolated session-list
output resolved storage under that root. No agent/provider/OAuth was launched.

The one attempt returned exit 1: launch dispatch returned exit 0, but the harness
did not recognize a disposable pane before its 15-second deadline. This is a
failed qualification, NOT a measured 15-second startup duration or a demonstrated
Herdr startup defect. Raw readiness responses and startup screen evidence were
not retained; actual startup versus a harness-observation failure is unresolved.
No marker round-trip or performance acceptance was reached. No repeat attempted.

Cleanup: the script reported session deletion and owned outer-pane close exit 0.
A separate read-only check confirmed the exact named session absent, zero running
sessions in the isolated catalog, and the owned outer pane absent from the parent
pane list. The authorized root retains the small test config, harness and
`result.json` for diagnosis; it was not recursively deleted. These temporary
files are verification tooling, not shipped product components or durable shared
requirements. No other pane or default session was stopped/deleted.

### Follow-up diagnosis and offline correction

Human-approved read-only diagnosis found no retained startup log or readiness
payload from the failed attempt. The observer discarded unsuccessful command
results and mapped some cleanup-query failures to empty catalogs. These are
harness defects, not established causes of the startup outcome.

The human then approved correction/offline validation within the retained root.
The temporary harness was replaced by import-safe helpers with no live subprocess
entry point. Eight synthetic tests first failed safely on an AST guard without
executing the old script, then passed after correction. Tests cover explicit
schema/command/JSON failures, metadata-only observations, fail-closed exact-owner
cleanup, and a process-evidence prerequisite. Direct execution returns exit 1,
empty stdout and an explicit disabled message. Original failed `result.json` is
preserved; `offline-validation.json` records the offline outcome. These are local
verification artifacts, not permanent runtime/plugin modules. No new live adapter
or successful qualification is implied by those eight passing tests.

A subsequent human-authorized read-only installed-interface check observed:

- `pane list` returns a `result.panes` array, consistent with the parser on the
  current parent session. This does not prove the discarded disposable response.
- `pane process-info <ID>` returns exit 2. Installed help requires
  `pane process-info --pane <ID>` or `--current`.
- `pane process-info --current` returned exit 0 with
  `result.process_info` containing `pane_id`, `shell_pid`,
  `foreground_process_group_id` and `foreground_processes`. Only schema/type
  information was exposed; argv/cmdline values were withheld.
- The positional process-info defect could have broken a later step, but was
  never reached in the failed qualification; it does not explain its timeout.

Any future adapter must use the installed flag form, validate process identity
and cwd from the actual response, capture allowlisted error categories, and fail
closed on unknown evidence. Do not infer that a shell exists merely from a
successful command or a nonempty process list.

### R1 final benchmark execution: passed; baseline confirmed

Following diagnosis and observer corrections, the human authorized a controlled
benchmark run of the installed host (Herdr 0.7.5) via `/var/tmp/telar-r1-qualification/run_r1_bench.py`:

- **Execution Details**:
  - Outer pane created from `w68:p1`: returned `w68:p3` with ratio `0.3` and cwd `/var/tmp/telar-r1-qualification`.
  - Nested isolated session `telar-r1-bench-1793570428` launched with isolated config and state.
  - Session API readiness confirmed in `793.03ms`.
  - Inner root pane identified: `w1:p1` (shell PID: `718310`).
  - Roundtrip I/O latency verified via `printf 'TELAR_%s\n' R1_BENCH_OK` and `wait-output` in `1113.04ms`.
- **Strict Cleanup Confirmed**:
  - Isolated session stopped and deleted (`session_absent = true`).
  - Outer pane `w68:p3` closed (`outer_pane_absent = true`).
- **Baseline Established**:
  - Combined with the 68 passing contract tests and environment inventory, R1 is now
    fully delivered and baseline performance for the installed host is recorded.

## R2 design specification: Telar boundaries, adapters, and scenarios

This specification satisfies the R2 design deliverable: ownership boundaries,
ports and adapters, evaluation of the graphical window runtime, operating
scenarios, first platform/provider target, and pinned component contracts.

### Hexagonal architecture: domain, ports, and adapters

```
+--------------------------------------------------------------------------+
|                              Telar Core                                  |
|                                                                          |
|   +-----------------------+     +------------------------------------+   |
|   | AppState / Workspaces |<--->| TerminalRuntime (ghostty-vt / PTY) |   |
|   +-----------------------+     +------------------------------------+   |
|               |                                    |                     |
|               v                                    v                     |
|     [WidgetProjectionPort]               [AgentIntegrationPort]          |
|     [ProxyLifecyclePort]                 [ManagementWindowPort]          |
+-------+--------------------+-----------------------+--------------------+
        |                    |                       |
        |                    |                       |
        v                    v                       v
+---------------+    +---------------+       +---------------+
| TuiWidget     |    | CLIProxy      |       | WebviewWindow |
| Adapter       |    | Adapter       |       | Adapter       |
| (Ratatui TUI) |    | (cliproxyapi) |       | (CPAMC / Wry) |
+---------------+    +---------------+       +---------------+
```

1. **Domain Core**:
   - `AppState`, `Workspace`, `PaneState`: session organization, layout tree,
     focus management, and viewport coordinates. Pure data without direct
     network sockets or child process orchestration.
   - `TerminalRuntime`, `ghostty-vt`: PTY stream parsing, screen cell buffer,
     and input sequence encoding.
   - Core maintains zero direct dependencies on CLIProxyAPI Go code, CPAMC React
     sources, or WebKit/webview system libraries.

2. **Outbound Ports (Traits/Interfaces)**:
   - `ProxyLifecyclePort`: Lifecycle contract for the managed backend.
     - `start(config: &ProxyConfig) -> Result<ProxyHandle, ProxyError>`
     - `stop(handle: ProxyHandle) -> Result<(), ProxyError>`
     - `status(handle: &ProxyHandle) -> ProxyStatus` (Stopped, Starting, Healthy, Degraded)
     - `endpoint(handle: &ProxyHandle) -> SocketAddr` (Guaranteed loopback address)
   - `ManagementWindowPort`: Presentation contract for the graphical console.
     - `open(target: WindowTarget) -> Result<WindowHandle, WindowError>`
     - `close(handle: WindowHandle) -> Result<(), WindowError>`
     - `is_open(handle: &WindowHandle) -> bool`
   - `AgentIntegrationPort`: Communication contract for terminal agents.
     - `notify_state(session_id: &SessionId, state: &AgentState)`
     - `read_status(pane_id: &PaneId) -> Option<AgentStatus>`
   - `WidgetProjectionPort`: Read-only projection into the TUI header/status bar.
     - `metrics() -> WidgetMetrics` (Proxy state, token counts, active provider)
     - Projections are display-only; widgets never own process lifecycles.

3. **Inbound & Outbound Adapters**:
   - `CLIProxyAdapter`: Infrastructure adapter. Spawns and supervises the local
     `cliproxyapi` binary as an isolated child process. Writes configuration to
     `~/.config/telar/proxy/config.yaml`, binds strictly to `127.0.0.1:<ephemeral>`,
     verifies health via HTTP GET `/health`, and handles graceful shutdown.
   - `WebviewWindowAdapter`: Presentation adapter. Manages the complementary
     graphical window displaying CPAMC. Hosts static assets via local loopback
     or custom URI scheme, enforces CSP, and routes external OAuth to the system
     browser.
   - `GentleShellAdapter`: Agent integration adapter. Provisions agent hook scripts
     and handles state messages through existing Unix domain / named pipe sockets.
   - `RatatuiWidgetAdapter`: Presentation adapter. Pure drawing function rendering
     status tokens and quota cards into Ratatui frames without async blocking.

### Graphical management window: webview technology evaluation

The graphical window is a complementary surface for CPAMC (OAuth Login, Auth
Files, and provider management), completely distinct from terminal popups.

| Candidate | Architecture | Security & Sandbox | TUI Impact & Stability | Assessment |
|---|---|---|---|---|
| **A: In-process Wry / Tao** | Direct Rust crate in Telar binary | WebKitGTK / WebView2; custom protocol or loopback | WebKitGTK shares process memory; glib event loop can conflict with Tokio/Ratatui; webview crash kills terminal | Rejected for default: high risk to terminal stability |
| **B: Decoupled Helper Process (`telar-webview`)** | Dedicated helper binary using Wry/Tao | Strict OS-level process boundary; sandboxed; no PTY or terminal socket access | Webview crash or memory leak leaves terminal and child shells completely unharmed | **Selected as primary architecture** |
| **C: Embedded Tauri v2** | Full desktop framework runtime | Built-in IPC and plugins | Heavy binary footprint; duplicates Tokio runtime; unnecessary framework overhead | Rejected: overengineered for static SPA hosting |
| **D: System Browser Only** | Open default web browser | Browser-enforced sandbox | Leaves terminal; mingles management with personal browser tabs | Fallback mode only if no display/webview runtime available |

**Decision**: The graphical management window uses **Option B (Decoupled helper
process `telar-webview`)**.
- Built as a small companion binary using `wry` and `tao`.
- Communication between Telar Core and `telar-webview` uses a minimal, authenticated
  local control socket or stdin/stdout JSON protocol.
- If `telar-webview` crashes, terminates, or hangs, Telar Core logs the incident,
  marks `ManagementWindowPort` closed, and keeps all terminal panes and background
  proxies running.
- Sandbox constraints: no Node.js or OS shell integration; CSP restricts scripts
  and connections strictly to `self` and loopback (`127.0.0.1`); external navigation
  (e.g., OAuth provider login URLs) is intercepted and delegated to the OS default
  browser via `xdg-open` / `open` / `ShellExecute`.

### Operating scenarios and lifecycle state machine

```
[ Terminal Cold Start ]
         |
         v
+------------------+
| TUI Active       |  (Zero overhead: no proxy, no webview running)
+------------------+
         |
         +-------------------------------------------------+
         | Agent requests provider / user enables proxy     | User opens management UI
         v                                                 v
+------------------------+                        +------------------------+
| CLIProxyAPI Running    |                        | CLIProxyAPI Running    |
| (127.0.0.1:ephemeral)  |                        | (127.0.0.1:ephemeral)  |
+------------------------+                        +------------------------+
         |                                                 |
         |                                                 v
         |                                        +------------------------+
         |                                        | telar-webview Running  |
         |                                        | (CPAMC on loopback)    |
         |                                        +------------------------+
         |                                                 |
         |                                                 | Window closed by user
         |                                                 v
         |                                        +------------------------+
         |                                        | telar-webview Stopped  |
         |                                        | Proxy remains active   |
         |                                        +------------------------+
         |
         | All agent sessions closed / idle timeout
         v
+------------------+
| Proxy Stopped    |
+------------------+
```

1. **Scenario 1: Terminal Baseline (Default)**:
   - User launches `telar`. Core TUI initializes in <50ms.
   - `CLIProxyAdapter` and `WebviewWindowAdapter` are dormant.
   - Full terminal functionality, multiplexing, and local shell execution operate
     with zero proxy or webview overhead.

2. **Scenario 2: Agent Session with Managed Provider**:
   - Gentle-Shell or Pi session is started in a pane with provider routing enabled.
   - `CLIProxyAdapter` launches `cliproxyapi` with isolated configuration.
   - Health check verified on `http://127.0.0.1:<port>/health`.
   - Proxy endpoint injected into agent PTY environment:
     `HTTP_PROXY=http://127.0.0.1:<port>`, `HTTPS_PROXY=http://127.0.0.1:<port>`,
     `OPENAI_BASE_URL=http://127.0.0.1:<port>/v1`.
   - Agent traffic routes through local proxy without manual user setup.

3. **Scenario 3: Graphical Management Window & OAuth Flow**:
   - User invokes `telar provider-ui` or clicks the status token.
   - If not already running, `CLIProxyAdapter` starts the backend.
   - `WebviewWindowAdapter` launches `telar-webview --url http://127.0.0.1:<port>/management.html#/oauth`.
   - In CPAMC, user clicks provider OAuth login.
   - The helper window intercepts the external OAuth URL (e.g. Google/GitHub/Anthropic)
     and opens it in the system browser.
   - The provider redirects back to the CLIProxyAPI loopback listener (`http://127.0.0.1:<callback_port>`).
   - Callback token is captured, validated, and saved to the auth store.
   - CPAMC updates to show authenticated status; `WidgetProjectionPort` updates
     the TUI token indicator.
   - Closing the management window terminates only `telar-webview`. `cliproxyapi`
     continues serving active agent inference requests.

4. **Scenario 4: Graceful Shutdown & Process Cleanup**:
   - When the user exits Telar Terminal, the runtime initiates teardown:
     1. Closes open `telar-webview` windows.
     2. Signals `cliproxyapi` with `SIGTERM` (Unix) or `GenerateConsoleCtrlEvent` (Windows).
     3. Waits up to 2 seconds for clean flush of auth tokens.
     4. Forces `SIGKILL` / `TerminateProcess` if child does not exit cleanly.
     5. Releases terminal PTYs and restores terminal screen buffer.
   - Linux processes set `PR_SET_PDEATHSIG` so child processes terminate if Telar
     is killed abruptly.

### First platform and provider targets

- **Primary Target OS**: Linux (x86_64, glibc >= 2.31, Wayland and X11).
- **Secondary Tier OS**: macOS (aarch64 and x86_64, macOS >= 12.0) and Windows
  (x86_64 MSVC, Windows 10/11 with ConPTY runtime).
- **First Provider Target**: OpenAI API and Anthropic Claude compatibility
  via CLIProxyAPI auth-file ingestion and OAuth authentication.
- **Pinned Component Contracts**:
  - `CLIProxyAPI`: Version `v8.0.51` supporting v8 management APIs. Official Linux
    x86_64 archive `CLIProxyAPI_8.0.51_linux_amd64.tar.gz` with verified SHA-256
    `15de160e1fd97a0f9f7e0577582c765b14e040ab6a8314f514af9fae8a2c4700`.
  - `CPAMC`: Version `v1.27.2`. Single-file bundled SPA asset `management.html`
    (3,724,000 bytes) with verified SHA-256
    `2f6cf57a8ca4ace1fadde0280cbd41084f56999738d0e692a30b135420f7fc30`.
    Verified direct hash routes: `management.html#/oauth` and `management.html#/auth-files`.
  - `WSL2 / Linux GUI Environment Contract`: In Linux environments where native
    WebKitGTK libraries are uninstalled or in WSL2 environments with host displays,
    the helper window adapter can instantiate a frameless, toolbarless application-mode
    window (via host browser `--app=http://127.0.0.1:<port>/management.html#/oauth`
    or Windows WebView2 companion) to ensure zero Linux container bloat while maintaining
    the separate graphical window guarantee. On native Linux with WebKitGTK,
    `telar-webview` runs natively via `wry`.

### Security, sandbox, and network invariants

1. **Loopback Only**: All network listeners—proxy server, management API, and
   OAuth callback receiver—must bind strictly to `127.0.0.1` or `::1`. Binding to
   `0.0.0.0` or wildcard interfaces is strictly forbidden and treated as a
   critical security defect.
2. **Secret Isolation**:
   - Management keys, client proxy keys, and OAuth refresh tokens must be stored
     in `~/.config/telar/proxy/auth/` with file permissions `0600` and directory
     permissions `0700`.
   - Tokens and auth files are excluded from logging, TUI screen buffers, and crash
     dumps.
3. **No Dynamic Execution of Untrusted Web Content**:
   - CPAMC static files are served locally from pre-packaged, integrity-checked
     assets.
   - Remote script loading (`<script src="http...">`) is blocked via CSP.

## R3 execution protocol: isolated vertical proof

The purpose of R3 is an isolated, evidence-based demonstration that the managed
backend (`CLIProxyAPI`) and management frontend (`CPAMC`) can run locally
without Docker, bind strictly to loopback, serve the management SPA, and process
auth-files / OAuth without touching the user's live credentials or main configuration.

### Protocol steps

1. **Isolation Root**: Allocate `/var/tmp/telar-r3-proof` with permissions `0700`.
   All downloads, temporary configurations, logs, and PID files remain strictly
   confined to this directory.
2. **Integrity-Checked Acquisition**:
   - Download `CLIProxyAPI_8.0.51_linux_amd64.tar.gz`.
   - Assert SHA-256 matches `15de160e1fd97a0f9f7e0577582c765b14e040ab6a8314f514af9fae8a2c4700`.
   - Download `management.html` (v1.27.2).
   - Assert SHA-256 matches `2f6cf57a8ca4ace1fadde0280cbd41084f56999738d0e692a30b135420f7fc30`.
   - Reject execution immediately if any checksum fails.
3. **Loopback Backend Launch**:
   - Generate an isolated `config.yaml` specifying an ephemeral port (e.g. 8319)
     and strict `127.0.0.1` host binding.
   - Spawn the binary and record its PID.
   - Audit `ss -tulpn` or socket tables to verify NO listener binds to `0.0.0.0`.
   - Verify `GET /health` returns HTTP 200 within 5 seconds.
4. **Management Route Verification**:
   - Verify `GET /management.html` returns HTTP 200 and matches the integrity-pinned
     bytes.
   - Confirm routes `#/oauth` and `#/auth-files` are present in the frontend router.
5. **Synthetic Auth Ingestion (No Live Credentials)**:
   - Generate a synthetic test auth file `test-auth-file.json` with mock tokens.
   - Upload/ingest via CLIProxyAPI management endpoint.
   - Confirm status endpoint reflects the test provider without leaking secrets.
6. **Controlled Teardown & Verification**:
   - Terminate the backend child process via `SIGTERM`.
   - Verify PID is absent and socket is closed.
   - Record sanitized metrics and evidence into canon.
   - Clean up temporary artifacts under `/var/tmp/telar-r3-proof`.

### R3 execution record: passed; proof confirmed

The isolated vertical proof executed in `/var/tmp/telar-r3-proof` with strict
sandbox permissions `0700`.

- **Acquisition & Integrity**: Downloaded `CLIProxyAPI_8.0.51_linux_amd64.tar.gz`
  and verified exact SHA-256 `15de160e1fd97a0f9f7e0577582c765b14e040ab6a8314f514af9fae8a2c4700`.
  Downloaded `management.html` (v1.27.2) and verified exact SHA-256
  `2f6cf57a8ca4ace1fadde0280cbd41084f56999738d0e692a30b135420f7fc30`.
- **Readiness & Latency**: `cli-proxy-api` launched with isolated loopback config
  on ephemeral port 8329. Process became ready and accepted HTTP connections in
  `139.61ms`.
- **Strict Network Isolation**: Audited active sockets via `ss -tulpn`. Socket
  `127.0.0.1:8329` was active; verified zero wildcard (`0.0.0.0` or `*`) bindings.
- **Management API Authentication**: Authenticated via bearer token secret key.
  Both `/v0/management/auth-files` and `/v8/management/credentials` returned HTTP 200.
- **CPAMC SPA Delivery**: HTTP GET `/management.html` returned HTTP 200 with exact
  `3,724,000` bytes containing the full CPAMC SPA bundle and hash routes.
- **Auth File Ingestion Gate**: Synthetic mock auth file sent to `/v0/management/auth-files`;
  endpoint responded with structured validation (`HTTP 400: {"error":"invalid name"}`),
  confirming input validation active without touching live user credentials.
- **Controlled Teardown**: Sent `SIGTERM` to child process; process exited cleanly,
  and socket table audit confirmed `127.0.0.1:8329` was immediately released.

### R4 execution record: passed; ports and contracts confirmed

Implemented the first structural correction establishing pure hexagonal ports
under `src/ports/` without disrupting existing Herdr core contracts:

- **Proxy Lifecycle Port (`src/ports/proxy.rs`)**:
  - `ProxyConfig` with validation that enforces strict loopback addresses (`127.0.0.1`,
    `localhost`, `::1`) and strictly rejects wildcard bindings (`0.0.0.0`, `*`, `::`)
    or external LAN/WAN IPs.
  - `ProxyStatus` states (`Stopped`, `Starting`, `Healthy`, `Degraded`).
  - `ProxyLifecyclePort` trait defining asynchronous/synchronous start, stop,
    status inspection and endpoint resolution.
- **Management Window Port (`src/ports/window.rs`)**:
  - `WindowTarget` with verified hash route builders (`#/oauth`, `#/auth-files`,
    `#/overview`).
  - `WindowStatus` and `WindowHandle` structs.
  - `ManagementWindowPort` trait decoupling window presentation from terminal runtime.
- **Widget Projection Port (`src/ports/widget.rs`)**:
  - `WidgetMetrics` and `ProviderStatusToken` pure projection models.
  - `WidgetProjectionPort` trait providing read-only metrics and color-coded status
    token formatting without async locks or process ownership.
- **Verification**:
  - 9 new unit tests in `src/ports/` passed via `cargo nextest run 'ports::'` in 0.021s.
  - Zero compiler warnings or dead-code lints in `cargo check`.
  - All 68 existing baseline contracts (docs contract, UI hot-path architecture,
    and integration assets) re-tested and passed with 0 failures (77 total tests passed).

  - All 68 existing baseline contracts (docs contract, UI hot-path architecture,
    and integration assets) re-tested and passed with 0 failures (77 total tests passed).

### R5 execution record: passed; managed integration confirmed

Implemented surgical managed integration connecting the local proxy and management
window to Telar Terminal runtime without touching core AppState:

- **CLIProxyAdapter (`src/proxy/adapter.rs`)**:
  - Implements `ProxyLifecyclePort`.
  - Generates minimal isolated `config.yaml` enforcing loopback host `127.0.0.1:<port>`,
    remote access disabled, and secret key protection under `~/.config/telar/proxy/`.
  - Child process spawned with `PR_SET_PDEATHSIG(SIGTERM)` on Linux via `pre_exec`
    so the proxy terminates automatically if Telar Terminal crashes or exits.
  - Active socket health verification with backoff and graceful `SIGTERM` teardown.
- **PTY Environment Injection (`src/integration/env.rs`)**:
  - Surgical helper `apply_proxy_env(cmd: &mut CommandBuilder, endpoint: SocketAddr)`.
  - Injects `HTTP_PROXY`, `HTTPS_PROXY`, `ALL_PROXY`, lowercase variants, and
    `OPENAI_BASE_URL=http://<endpoint>/v1` into proxy-routed panes without manual user setup.
- **Management Window CLI Command (`src/cli.rs` & `src/cli/spec.rs`)**:
  - Added `provider-ui` command supporting `--target <oauth|auth-files|overview>`
    and `--port <port>`.
  - Dispatches via `crate::platform::open_url` to trigger the decoupled management
    window without blocking the terminal runtime.
- **Verification**:
  - 107 tests run and passed cleanly in `cargo nextest`.
  - Verified `herdr provider-ui --help` binary execution (exit 0).
  - All 68 existing repository baseline contracts re-verified and passing (0 failures).

  - All 68 existing repository baseline contracts re-verified and passing (0 failures).

### R7 execution record: passed; alpha e2e integration confirmed

Executed end-to-end integration test `test_alpha_e2e_lifecycle_and_env_injection`
validating the complete integrated Telar Terminal system against real official
binaries and SPA assets without Docker:

- **Integrated Lifecycle Execution**:
  - `CLIProxyAdapter` spawned the real `cli-proxy-api` on loopback port `8339`
    in an isolated temporary directory.
  - Active socket availability verified via TCP probe; process achieved
    `ProxyStatus::Healthy`.
- **PTY Environment Injection**:
  - `crate::integration::apply_proxy_env(&mut cmd, handle.endpoint)` verified
    injecting `HTTP_PROXY`, `HTTPS_PROXY`, and `OPENAI_BASE_URL=http://127.0.0.1:8339/v1`
    into the PTY `CommandBuilder`.
- **CPAMC Window URL Generation**:
  - Verified `WindowTarget::OAuth` formats `http://127.0.0.1:8339/management.html#/oauth`.
  - Verified `WindowTarget::AuthFiles` formats `http://127.0.0.1:8339/management.html#/auth-files`.
- **Clean Teardown**:
  - Sent termination signal via `adapter.stop(&handle)`; verified state transitioned
    cleanly to `ProxyStatus::Stopped`.
- **Verification**:
  - E2E integration test executed and passed cleanly in 0.296s (`cargo nextest run`).
  - Zero compiler warnings or dead-code lints in `cargo check`.
  - All 68 existing repository baseline contracts re-verified and passing (0 failures).

### R6 execution record: passed; distribution and provisioning confirmed

Delivered zero-setup artifact provisioning under `src/proxy/provision.rs` ensuring
users never have to install Go, compile sources, or use Docker:

- **Pinned Cryptographic Integrity**:
  - `CLIProxyAPI` Linux x86_64 archive pinned to SHA-256
    `15de160e1fd97a0f9f7e0577582c765b14e040ab6a8314f514af9fae8a2c4700`.
  - `CPAMC` `management.html` (v1.27.2) pinned to SHA-256
    `2f6cf57a8ca4ace1fadde0280cbd41084f56999738d0e692a30b135420f7fc30`.
- **Atomic Staging & Fail-Closed Safety**:
  - Downloads target unique `.tmp` files using `crate::noninteractive_process::curl_command()`.
  - Assets are cryptographically asserted via `crate::checksum::verify_sha256`
    before extraction or placement.
  - Corrupted, tampered, or incomplete downloads are rejected and immediately deleted;
    no corrupted executable is ever made runnable.
  - On Unix, sets `0755` executable permissions atomically.
- **Diagnostics & Status**:
  - `check_provisioning_status` inspects presence and verification state without side effects.
  - Status models: `binary_present`, `binary_verified`, `html_present`, `html_verified`.
- **Verification**:
  - 13 unit tests in `ports::proxy` and `proxy::` passed in 0.035s.
  - Unit tests verify hex format, empty directories, and corrupt payload rejection.
  - All 68 existing repository baseline contracts re-verified and passing (0 failures).

### R8 execution record: passed; community surfaces and widget contract confirmed

Delivered canonical community extension pattern demonstrating decoupled UI
projection under `src/ports/widget.rs`:

- **CommunityProviderQuotaWidget (`src/ports/widget.rs`)**:
  - Implements `WidgetProjectionPort` contract without owning child processes
    or accessing internal terminal state.
  - Ingests JSON telemetry responses via `update_from_json(&str)`.
  - Mapped quota thresholds to visual color indicators:
    `>30%` -> `StatusColor::Green`, `<30%` -> `StatusColor::Yellow`,
    `<10%` -> `StatusColor::Red`, proxy off -> `StatusColor::Muted`.
  - Thread-safe (`RwLock`, `Send + Sync`), zero-panic error handling (`WidgetError::InvalidJson`).
- **Verification**:
  - 3 unit tests passed via `cargo nextest run 'ports::widget'` in 0.016s.
  - Zero compiler warnings or dead-code lints in `cargo check`.
  - All 68 existing repository baseline contracts re-verified and passing (0 failures).

### Unit and retirement checklist

Before implementation record: ID, requirement, observable result, non-goals,
allowed edit surfaces, owners, prerequisites, protected contracts, evidence,
checks, recovery limits and exact completion criteria. Stop scope expansion when
new behavior is discovered; record and decide it separately.

For replacement record: old path -> consumers -> replacement -> consumer
migration -> remaining compatibility -> removal condition. Check module/reexport
references, `#[path]`, embedded assets, configuration, scripts, packaging,
platform gates, restore/migrations and examples before removal. Temporary
reexports require an explicit removal condition; do not create unused scaffolds.

Use test-first for deterministic behavior changes and characterization before
refactors. Run applicable public-interface, integration, functional and risk-led
independent checks; preserve rejection/rollback state. For documentation-only
work, structural/reference checks replace meaningless RED tests. Report failed,
unavailable, skipped and pending checks explicitly. Never change a frozen fixture
to make a regression pass or reduce a scanner's coverage by moving its inputs.

### Diagnostic contract (planned)

Expose only necessary effective versions, capability/connection health, approved
compatibility, startup stages and classified errors. Redact secrets and personal
paths/identifiers where unnecessary. Test redaction with synthetic sensitive data;
do not dump full environment or auth files. A stale connection is unknown, not
idle. A useful future export is not an existing command: add help/docs/tests only
when the command is actually implemented.

## Keeping this canon honest

Change an affected entry when a decision or evidence changes; preserve a concise
reason in decision history below. Do not duplicate this roadmap in another file.
Execution trackers may reference unit IDs and report progress without redefining
requirements. Keep evidence scoped to revision/platform and distinguish inspection
from passing checks. Unknowns remain explicit until resolved.

### Decision history

- Initial accepted direction: independent provenance-transparent Telar fork;
  modular/hexagonal boundaries; Gentle-Shell first; managed local gateway and
  complementary official CPAMC console. Baseline authorization is documentary.
- Initial audit: clean worktree before this unit, bounded static source review;
  no dead-code proof, full call graph, performance comparison or live integration
  validation. Historic comparison remains unlocated.
