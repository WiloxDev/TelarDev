# Investigation: Body/Terminal Pane Scrollbar Auto-Hide on Scroll

## Executive Summary

Visual inspection and user feedback identified that the main body/terminal pane scrollbar remains permanently visible in Telar, drawing vertical track and thumb lines (`▕` and `▐`) along the right edge of the terminal pane even when sitting at the prompt with no active scroll. Code inspection in `src/ui/scrollbar.rs:27` and `src/ui/panes.rs:305-310` confirmed the root cause: `should_show_scrollbar(metrics)` is conditioned solely on `metrics.max_offset_from_bottom > 0`. Once any command outputs more lines than the viewport height, `max_offset_from_bottom` remains positive indefinitely, causing the scrollbar to be permanently painted.

Crucially, **Telar's layout architecture already completely decouples gutter space reservation from scrollbar rendering**. The 1-column scrollbar gutter (`terminal_inner_rect_for`) is stably reserved on the primary screen regardless of whether the scrollbar is actively rendered. Therefore:
1. Hiding the scrollbar when `offset_from_bottom == 0` **does NOT** trigger terminal column reflow, does not change PTY dimensions, and does not emit `SIGWINCH`.
2. The retained headless surface renderer (`src/server/headless/retained_surface.rs:93-130`) already natively supports transitioning `scrollbar_rect` to `None`, automatically emitting blank patch cells to erase the scrollbar.
3. Conditionally displaying the scrollbar when `offset_from_bottom > 0` (or during active scrolling) provides an immediate, clean, flicker-free auto-hide experience with zero downtime and full regression safety.

## Research Question

Why does the terminal pane scrollbar remain continuously visible after any command produces scrollback, and how can Telar auto-hide the scrollbar so it is invisible while sitting at the bottom prompt and automatically appears only when the user scrolls back into history?

## Recommendation

1. **Introduce `should_show_pane_scrollbar(metrics)`**:
   In `src/ui/scrollbar.rs`, define:
   ```rust
   pub(crate) fn should_show_pane_scrollbar(metrics: crate::pane::ScrollMetrics) -> bool {
       metrics.offset_from_bottom > 0
   }
   ```
   Retain `should_show_scrollbar` for non-terminal modal views (such as `release_notes_scrollbar_rect`).

2. **Connect Pane Layout & Headless Retained Surface**:
   - In `src/ui/panes.rs:307`, filter `stable_scrollbar_gutter` with `should_show_pane_scrollbar`.
   - In `src/server/headless/retained_surface.rs:98`, update `retained_scrollbar_patch` to use `should_show_pane_scrollbar`.

3. **Verify Gutter Stability & Tests**:
   Ensure `terminal_inner_rect_for` continues to stably preserve the 1-column margin on the primary screen so PTY widths never wobble, and update unit tests to verify both auto-hide at the bottom and appearance upon scrolling up.

## Key Findings

1. **Root Cause in `src/ui/scrollbar.rs:27`**:
   `should_show_scrollbar` previously checked `metrics.max_offset_from_bottom > 0`. Any terminal session with history retained this condition forever.
2. **Decoupled Architecture (`terminal_inner_rect_for`)**:
   Terminal geometry is computed from `pane_scrollbars && !rt.alternate_screen_active()`, not from whether the scrollbar is currently drawn. Toggling visibility on scroll does not trigger terminal resize or `SIGWINCH`.
3. **Headless Retained Clean Patches**:
   When `next_rect` is `None` after being `Some`, `retained_scrollbar_patch` renders blank cells over `patch_rect`, properly clearing the scrollbar in client sessions.
4. **Mouse Hit Safety**:
   Wheel events (`ScrollUp`) hit the pane directly, incrementing `offset_from_bottom`, which immediately makes the scrollbar visible and interactive for dragging.
