# M-016 — Reader Customization Controls

## Milestone ID
M-016

## Title
Reader Customization Controls

## Objective
Keep the rendered shell self-contained while letting users tune key reading-surface dimensions and control radii from the rendered page itself. Keep sidebar footer hotkeys visible even when the hierarchy is taller than the viewport.

## Included Features
- F-018 — Persistent reader appearance controls
- B-005 — Sidebar footer hotkeys can disappear behind long hierarchy trees

## Dependencies
- v0.6.0-era render shell work, now carried forward in the self-contained renderer
- Existing theme + sidebar controls from M-014 / F-016

## Acceptance Criteria
- Recursive sidebar footer hotkeys remain visible at the bottom of the sidebar, even when the hierarchy overflows.
- The footer uses an opaque-enough background treatment so overlaid tree content remains legible.
- Rendered pages expose a form that lets users tune font size, letter width, letter corner radius, sidebar button radius, and theme button radius.
- The form emits a `mark config` command that persists those values into `~/.mark/config.toml`.
- The next render automatically uses the persisted values.
- Cache reuse stays settings-aware so outdated appearance settings are not silently reused.
- All checks pass; `README.md` and owner docs reflect the new controls.

## Priority
High

## Status
released

## Target Release
v0.6.1
