# M-017 — Reader Controls Polish

## Milestone ID
M-017

## Title
Reader Controls Polish

## Objective
Refine the recently shipped reader customization experience so its entry point is clearer, keyboard controls are more intuitive, layout changes preview immediately on the current document, and visual shell regressions are removed.

## Included Features
- F-019 — Config menu hotkeys and live reader-layout preview polish
- B-006 — Stray unlabeled checkbox appears in the reader shell

## Dependencies
- M-016 — Reader Customization Controls
- F-018 — Persistent reader appearance controls
- Existing render shell and in-page hotkey handling in `src/render.rs`

## Acceptance Criteria
- The top-right reader control uses a config-style icon and opens a config menu that houses both theme and reader-layout controls.
- `t` toggles only between light and dark themes and no longer cycles through system mode.
- `c` opens the config menu.
- Reader-layout changes immediately apply to the currently viewed document while still generating the save command for persistence.
- Reader width is presented in `rem`, not inches.
- The reader shell background glow is toned down substantially.
- No stray unlabeled checkbox remains in the rendered page chrome.
- Existing persistence, render caching expectations, and keyboard accessibility remain intact.

## Priority
High

## Status
released
