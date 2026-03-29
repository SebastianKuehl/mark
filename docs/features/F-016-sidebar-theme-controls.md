# F-016 — Sidebar and Theme Controls

## Feature ID
F-016

## Title
Hide the sidebar by default, add keyboard/sidebar hints, and support in-page light/dark/system switching

## User Value
Users want a less intrusive default reading experience, faster navigation controls, and the ability to change themes from the rendered page without re-running `mark`.

## Scope Details

### Sidebar default + hotkey

- The sidebar is hidden by default.
- The sidebar toggle control advertises the `e` keyboard shortcut via tooltip or equivalent visible hint.
- Pressing `e` toggles the sidebar open/closed from the rendered page.

### Sidebar ordering

At every directory level in the sidebar tree:

1. files appear first
2. folders appear second
3. each group is sorted deterministically (alphabetically)

This rule applies recursively inside nested folders too.

### Theme toggle

Add an in-page theme switcher with these options:

- `system`
- `light`
- `dark`

Each option must show both an icon and a text label.

### Theme defaults

- The rendered page defaults to `system`.
- Existing CLI/config theme support should be updated coherently so `system` is a valid initial theme.
- The in-page theme switch is ad hoc (browser-side) rather than requiring a re-render.

## Dependencies
- F-015 (shared CLI/config/render surfaces; implement sequentially in the same worktree)

## Acceptance Criteria
1. Sidebar is hidden by default.
2. Sidebar can be toggled with `e`.
3. Sidebar toggle advertises the hotkey.
4. Sidebar ordering is files first, folders second, recursively.
5. Theme switcher renders icon + label for `system`, `light`, and `dark`.
6. Default rendered theme is `system`.
7. Tests cover sidebar ordering, hidden-by-default state, and theme-toggle markup/behavior.

## Priority
High

## Milestone
M-014

## Status
released
