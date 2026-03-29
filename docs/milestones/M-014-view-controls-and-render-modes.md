# M-014 — View Controls and Render Modes

## Milestone ID
M-014

## Title
View Controls and Render Modes

## Objective
Give users explicit control over how `mark` renders and browses documentation sets: single-file versus recursive rendering, configurable default sidebar behavior, faster keyboard access to the sidebar, deterministic sidebar ordering, and a browser-side theme switcher with a system default.

## Included Features
- F-015 — Render mode flags and persistent defaults
- F-016 — Sidebar behavior polish and in-page theme switching

## Dependencies
- v0.4.0 (M-013 / F-014 released)

## Acceptance Criteria
- `mark` supports explicit `--single/-s` and `--recursive/-r` render modes.
- Single-file mode renders only the passed file, does not show the sidebar, and clearly notes skipped local Markdown links.
- Recursive mode preserves the current linked-file rendering behavior.
- Config supports persistent defaults for render mode and sidebar visibility.
- Sidebar is hidden by default, advertises the `e` hotkey, and can be toggled with that key.
- Sidebar ordering is files first, then folders, recursively within every directory level.
- Rendered pages expose a theme toggle with `system`, `light`, and `dark`, each with icon + label.
- All checks pass and README is updated after merge.

## Priority
High

## Status
released

## Target Release
v0.5.0 (minor)
