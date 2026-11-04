# Sidebar and Dock Border Contrast Investigation

## Executive Summary

Visual inspection of application screenshot `6.png` revealed that under the Tokyo Night theme, the user interface appears completely flat: the left sidebar vertical divider line and right dock card boundaries (`╭───╮`, `│`, `╰───╯`) are virtually invisible. Code inspection in `src/client/shell/render.rs`, `src/client/shell/dock.rs`, and `src/app/state.rs` confirmed the root cause: the borders and dividers are rendered with `fg(palette.surface_dim)`, but in Tokyo Night (and seven other themes), `surface_dim` is assigned an RGB value 100% identical to `panel_bg` (`Color::Rgb(26, 27, 38)`). The resulting color difference ($\Delta E = 0.0$) produces a 1.0:1 contrast ratio, rendering separators invisible against the panel background. Furthermore, the right dock lacks a continuous vertical structural boundary dividing it from the terminal area. Remedying this requires re-tokenizing border styles, re-calibrating `surface_dim` across affected themes, and introducing a formal left divider for the dock.

## Research Question

What causes the severe lack of visual contrast and missing borders in the sidebar and dock (as shown in `6.png`), how widespread is this issue across Telar's theme definitions, and what structural and semantic changes are required to establish robust visual hierarchy and elevation?

## Recommendation or Answer

1. **Immediate Theme Fix (`Palette` Palette Definitions):** Recalibrate `surface_dim` across all 8 affected themes in `src/app/state.rs` so that it establishes at least a perceptible lightness step (e.g., Tokyo Night `#292e42` / `Color::Rgb(41, 46, 66)`) over `panel_bg` (`#1a1b26`).
2. **Semantic Border Token Alignment:** In `src/client/shell/render.rs` and `src/client/shell/dock.rs`, update card borders and structural separator lines to draw using `palette.surface1` (or `palette.surface0`), reserving `surface_dim` strictly for low-contrast subtle grid lines or subtle backgrounds. Optionally fill dock card interiors with `palette.surface0` to establish genuine card elevation over `panel_bg`.
3. **Structural Enclosure for Right Dock:** Render a dedicated vertical separator column at `dock_area.x` using the same structural separator styling as the left sidebar (`src/client/shell/render.rs:26`), framing the central terminal workspace symmetrically.

## Key Findings

1. **Root Cause in `tokyo_night` Theme Token Mapping:**
   - In `src/app/state.rs:162`, `Palette::tokyo_night()` defines:
     * `panel_bg: Color::Rgb(26, 27, 38)` (`#1a1b26`)
     * `surface_dim: Color::Rgb(26, 27, 38)` (`#1a1b26`)
   - Left sidebar divider (`src/client/shell/render.rs:26`) draws `│` with `fg(palette.surface_dim)`.
   - Right dock cards (`src/client/shell/dock.rs:1092`) draw border blocks with `card_border_style = Style::default().fg(config.palette.surface_dim)`.
   - Because foreground and background colors are identical, borders fail to produce any pixel luminance difference.

2. **Systemic Theme Flaw Across 8 Themes:**
   - A full audit of all 20+ theme definitions in `src/app/state.rs` indicates that 8 themes share this identical bug (`surface_dim == panel_bg`): `tokyo_night`, `dracula`, `nord`, `gruvbox`, `one_dark`, `solarized`, `kanagawa`, and `nightfly`.
   - In these 8 themes, borders rendered with `surface_dim` are completely invisible on `panel_bg`.
   - Other themes feature minimal or suboptimal contrast steps (e.g., Catppuccin with only a 6-unit RGB delta between 24 and 30).

3. **Asymmetric Structural Frame:**
   - The left sidebar is bounded by an explicit single-column vertical separator (`│`) at line 26 of `render.rs`.
   - The right dock has no dedicated vertical column separator between `terminal_area` and `dock_area`. It relies solely on card box glyphs (`╭───╮`, `│`, `╰───╯`). When borders disappear or when cards do not occupy the full vertical span, the terminal content and dock content visually bleed into one another.

## Evidence Review

### Component Border Styling Inspection

- **Left Sidebar Separator** (`src/client/shell/render.rs:26` `[S-001]`):
  The vertical divider between sidebar and main viewport is rendered as:
  ```rust
  let sep_style = Style::default().fg(palette.surface_dim);
  // Draws vertical separator glyph '│' along sidebar bounds
  ```
- **Right Dock Card Borders** (`src/client/shell/dock.rs:1092` `[S-002]`):
  Each dock card widget initializes its border style as:
  ```rust
  let card_border_style = Style::default().fg(config.palette.surface_dim);
  let card_block = Block::default()
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .border_style(card_border_style);
  ```
- **Theme Definition in `state.rs`** (`src/app/state.rs:162` `[S-003]`):
  ```rust
  // tokyo_night
  panel_bg: Color::Rgb(26, 27, 38),     // #1a1b26
  surface_dim: Color::Rgb(26, 27, 38),  // #1a1b26 (Identical to panel_bg)
  surface0: Color::Rgb(36, 40, 59),     // #24283b
  surface1: Color::Rgb(41, 46, 66),     // #292e42
  ```

### Full Theme Palette Audit (`src/app/state.rs`)

The table below catalogs theme values across `panel_bg`, `surface_dim`, `surface0`, and `surface1`, evaluating separator visibility:

| Theme | `panel_bg` | `surface_dim` | `surface0` | `surface1` | Status / Contrast |
|---|---|---|---|---|---|
| `tokyo_night` | `(26, 27, 38)` | `(26, 27, 38)` | `(36, 40, 59)` | `(41, 46, 66)` | **Zero (1:1) - Broken** |
| `dracula` | `(40, 42, 54)` | `(40, 42, 54)` | `(68, 71, 90)` | `(98, 114, 164)` | **Zero (1:1) - Broken** |
| `nord` | `(46, 52, 64)` | `(46, 52, 64)` | `(59, 66, 82)` | `(67, 76, 94)` | **Zero (1:1) - Broken** |
| `gruvbox` | `(40, 40, 40)` | `(40, 40, 40)` | `(60, 56, 54)` | `(80, 73, 69)` | **Zero (1:1) - Broken** |
| `one_dark` | `(40, 44, 52)` | `(40, 44, 52)` | `(50, 56, 66)` | `(62, 68, 81)` | **Zero (1:1) - Broken** |
| `solarized` | `(0, 43, 54)` | `(0, 43, 54)` | `(7, 54, 66)` | `(88, 110, 117)` | **Zero (1:1) - Broken** |
| `kanagawa` | `(31, 31, 40)` | `(31, 31, 40)` | `(42, 42, 55)` | `(54, 54, 70)` | **Zero (1:1) - Broken** |
| `nightfly` | `(1, 22, 39)` | `(1, 22, 39)` | `(11, 41, 66)` | `(18, 59, 92)` | **Zero (1:1) - Broken** |
| `catppuccin` | `(24, 24, 37)` | `(30, 30, 46)` | `(49, 50, 68)` | `(69, 71, 90)` | Marginal (~1.1:1) |
| `rose_pine` | `(25, 23, 36)` | `(31, 29, 46)` | `(38, 35, 58)` | `(57, 53, 82)` | Marginal (~1.2:1) |
| `monokai_pro` | `(45, 42, 46)` | `(58, 53, 57)` | `(64, 62, 65)` | `(90, 88, 89)` | Low-Medium (~1.4:1) |

## Trade-offs and Risks

| Strategy | Trade-offs | Risks |
|---|---|---|
| **A: Fix `surface_dim` in `Palette`** | Directly repairs all components relying on `surface_dim`. Low code churn. | If other UI areas rely on `surface_dim` being identical to `panel_bg` for stealth/invisible padding, changing it may introduce unintended lines. |
| **B: Re-tokenize to `surface0`/`surface1`** | Adheres to modern design system elevation layers (`surface0` for card fill, `surface1` for border strokes). High aesthetic clarity. | Requires modifying call sites in `dock.rs` and `render.rs`. Must verify contrast across all 20+ themes to avoid overly harsh borders in high-contrast themes. |
| **C: Add Vertical Dock Separator Line** | Symmetrically balances left sidebar; prevents terminal output from overflowing visually into dock space. | Consumes 1 column of horizontal terminal space (`terminal_area.width -= 1`). Terminal layout math must be verified. |

## Alternatives Considered

1. **Theme-level Only (`surface_dim` adjustments):**
   - *Pros:* Zero layout modifications; solves immediate zero-contrast bug.
   - *Cons:* Leaves the right dock unpartitioned from the terminal workspace; single-color borders without card background elevation still feel flat in dense TUI layouts.
2. **Terminal Margin / Spacer Column:**
   - *Pros:* Avoids drawing a glyph; uses empty space.
   - *Cons:* Does not provide explicit structural containment. In text-heavy terminal sessions, whitespace alone easily blurs boundary edges.
3. **Card Elevation with Contrasting Background:**
   - Set dock card background to `surface0` with border `surface1` on `panel_bg`.
   - Highly recommended as an enhancement alongside the separator line.

## Unknowns and Limits

- Exact horizontal column budgets in constrained terminal widths (e.g., 80x24 TUI viewports): adding 1 column for the right dock separator reduces the terminal width by 1 character, which could wrap 80-column lines if horizontal space is strictly tight.
- Whether any user-defined or runtime themes bypass `src/app/state.rs`.

## Recommended Next Actions

1. **Phase 1: Token Recalibration in `src/app/state.rs` `[S-003]`:**
   - For all 8 broken themes (`tokyo_night`, `dracula`, `nord`, `gruvbox`, `one_dark`, `solarized`, `kanagawa`, `nightfly`), update `surface_dim` to have distinct lightness (e.g., Tokyo Night `Color::Rgb(41, 46, 66)` or `Color::Rgb(36, 40, 59)`).
2. **Phase 2: Semantic Border Token Upgrade `[S-001]`, `[S-002]`:**
   - Update `card_border_style` in `src/client/shell/dock.rs:1092` to use `config.palette.surface1`.
   - Update left sidebar separator in `src/client/shell/render.rs:26` to use `palette.surface1` (or `palette.surface0`).
3. **Phase 3: Right Dock Separator Framing:**
   - In `src/client/shell/render.rs`, draw a vertical column separator `│` at `dock_area.x` when the dock is visible, matching the sidebar styling.
4. **Phase 4: Visual Regression Verification:**
   - Re-render Tokyo Night and Dracula views to confirm crisp border delineation without visual vibration or clipping.
