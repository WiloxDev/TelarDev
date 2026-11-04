# Sources

## Source Index

### S-001: Left Sidebar Vertical Separator Rendering
- Family: IMPLEMENTATION
- URL or locator: `src/client/shell/render.rs:26`
- Date/version: v0.1.0-current
- Access method: Codebase inspection
- Used for: Section "Evidence Review" and "Key Findings" (Left sidebar separator styling)
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Identifies that the vertical sidebar dividing line `│` is explicitly styled with `fg(palette.surface_dim)`.

### S-002: Right Dock Card Border Construction
- Family: IMPLEMENTATION
- URL or locator: `src/client/shell/dock.rs:1092`
- Date/version: v0.1.0-current
- Access method: Codebase inspection
- Used for: Section "Evidence Review" and "Key Findings" (Right dock card borders)
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Confirms `card_border_style` relies on `config.palette.surface_dim` for `Borders::ALL` on `Block` widgets.

### S-003: Telar Theme Palette Definitions
- Family: IMPLEMENTATION
- URL or locator: `src/app/state.rs:162`
- Date/version: v0.1.0-current
- Access method: Codebase inspection
- Used for: Section "Evidence Review", "Full Theme Palette Audit", and "Root Cause"
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Demonstrates that `tokyo_night` assigns identical RGB tuples `(26, 27, 38)` to both `panel_bg` and `surface_dim`, with 7 additional themes (`dracula`, `nord`, `gruvbox`, `one_dark`, `solarized`, `kanagawa`, `nightfly`) displaying the identical issue.

### S-004: Visual UI Snapshot (Tokyo Night Theme)
- Family: IMPLEMENTATION
- URL or locator: `6.png`
- Date/version: Current UI capture
- Access method: Visual inspection
- Used for: Section "Executive Summary" and "Key Findings"
- Usefulness: MATERIAL
- Confidence impact: HIGH
- Notes: Visual evidence showing flat, borderless dock cards and missing sidebar divider line under Tokyo Night theme.
