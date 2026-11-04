# Sources: Body/Terminal Pane Scrollbar Auto-Hide on Scroll

## Code Sources

- `src/ui/scrollbar.rs:27-31` `[S-001]`:
  Defines `should_show_scrollbar(metrics)` conditioned on `max_offset_from_bottom > 0`.
- `src/ui/panes.rs:36-48` `[S-002]`:
  Defines `terminal_inner_rect_for(pane_inner, scrollbar_gutter)` which decouples gutter reservation from scrollbar drawing.
- `src/ui/panes.rs:290-310` `[S-003]`:
  `stable_scrollbar_gutter` computes `(inner_rect, scrollbar_rect)`.
- `src/server/headless/retained_surface.rs:93-130` `[S-004]`:
  `retained_scrollbar_patch` handles incremental patch updates, writing blank cells when `next_rect` is `None`.
- `src/client/shell/mouse.rs:38-51` `[S-005]`:
  `pane_scrollbar_offset` handles click and drag hits on `scrollbar_rect`.
