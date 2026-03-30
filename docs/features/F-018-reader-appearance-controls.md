# F-018 — Persistent Reader Appearance Controls

## Feature ID
F-018

## Title
Configure reader appearance from the rendered page and persist it through `mark config`

## User Value
Users can fine-tune the reading experience without editing config files by hand. The rendered page itself shows the available layout controls and produces the exact terminal command needed to save them for future renders.

## Scope Details
- Add a new `mark config set-layout` command for persisting reader appearance settings.
- Persist these settings in `~/.mark/config.toml`:
  - base font size
  - letter width
  - letter corner radius
  - sidebar button radius
  - theme button radius
- Expose those settings in an in-page form reachable from the top-right theme/settings control.
- The form must render the exact `mark config set-layout ...` command to run in a terminal.
- The rendered shell must apply the configured values on the next render.
- Cache reuse must reject stale renders whose stored appearance settings no longer match the current config.

## Dependencies
- F-017 — Template-driven render shell
- F-016 — Sidebar and theme controls

## Acceptance Criteria
1. `mark config set-layout` accepts all five appearance values and persists them.
2. Rendered pages show a reader-layout form with those same five controls.
3. The form outputs a copyable terminal command using the `mark` CLI.
4. Rendered pages apply the persisted font size, letter width, and radii.
5. Cache option matching includes appearance settings so new values trigger a fresh render.
6. Tests cover config persistence and rendered-shell output for the new controls.

## Priority
High

## Milestone
M-016

## Status
merged
