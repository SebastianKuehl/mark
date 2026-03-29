# P-015 — M-014: View Controls and Render Modes

## Prompt ID
P-015

## Linked Item
M-014 — View Controls and Render Modes

Feature scope covered:
- F-015 — Render mode flags and persistent defaults
- F-016 — Sidebar and theme controls

## Task Objective
Implement the post-v0.4.0 controls milestone in a single worktree: explicit single-vs-recursive render modes, persisted defaults for render mode and sidebar visibility, hidden-by-default sidebar with `e` hotkey + tooltip, files-first recursive sidebar ordering, and an in-page theme switcher offering `system`, `light`, and `dark`.

---

## Worktree Instructions

```sh
cd /Users/sebastian/Documents/repos/mark
git worktree add .worktrees/M-014-view-controls -b feat/M-014-view-controls main
cd .worktrees/M-014-view-controls
```

All work must happen inside `.worktrees/M-014-view-controls`.

---

## Exact Scope

### 1. CLI render modes and config defaults

Update `src/cli.rs`, `src/config.rs`, and `src/main.rs` so that:

- `mark` accepts `--single/-s` and `--recursive/-r`
- these flags are mutually exclusive
- if neither is passed, render mode comes from persisted config
- hardcoded default remains recursive mode
- config supports persistent defaults for:
  - render mode
  - sidebar visibility

Recommended CLI/config surface:

- `mark config set-render-mode <single|recursive>`
- `mark config set-sidebar <hidden|visible>`

Also extend the existing theme option so `system` is a valid theme value.

### 2. Single-file mode semantics

When render mode is `single`:

- render only the entry file
- do not BFS-discover or render linked Markdown files
- do not rewrite local Markdown links to generated HTML
- do not render the sidebar
- print a concise CLI note listing skipped local Markdown links if any were found

### 3. Recursive mode semantics

When render mode is `recursive`, preserve the current linked-file behavior.

### 4. Sidebar behavior

Update `src/render.rs` and `src/style.css` so that:

- the sidebar is hidden by default
- the sidebar toggle indicates the `e` hotkey via tooltip and/or equivalent visible affordance
- pressing `e` toggles the sidebar open/closed
- the hotkey should avoid firing in obvious editable contexts

### 5. Sidebar ordering

Update sidebar tree construction so that, at every directory level:

1. files come before folders
2. files are sorted alphabetically
3. folders are sorted alphabetically

This ordering must apply recursively inside every folder.

### 6. In-page theme toggle

Add a rendered-page theme control offering:

- `system`
- `light`
- `dark`

Requirements:

- each option includes icon + text
- default rendered theme is `system`
- switching themes is ad hoc in the browser (no re-render required)
- existing CLI/config theme plumbing should remain coherent with the new `system` option

### 7. Reasonable implementation assumptions

Use these assumptions unless the code clearly suggests a better aligned pattern:

- CLI precedence: explicit render-mode flag > config render-mode default > hardcoded recursive
- Sidebar precedence: config sidebar default controls initial open/hidden state
- Theme precedence for initial render: explicit CLI theme > config theme > hardcoded system
- Single-mode “links are noted” means printing a concise CLI note naming skipped local Markdown links

---

## Files to Inspect

- `src/cli.rs`
- `src/config.rs`
- `src/main.rs`
- `src/render.rs`
- `src/style.css`
- `README.md` (for recommendation only — do not edit)
- `docs/milestones/M-014-view-controls-and-render-modes.md`
- `docs/features/F-015-render-mode-defaults.md`
- `docs/features/F-016-sidebar-theme-controls.md`

---

## Implementation Constraints

- Do not add new external crate dependencies unless strictly necessary.
- Reuse existing patterns for config loading, CLI validation, and render orchestration.
- Keep the work in one worktree; do not split into additional branches/tasks.
- Preserve existing recursive behavior when `recursive` is selected.
- Keep single-file rendering behavior clean and explicit rather than silently pretending recursive features still apply.

---

## Acceptance Criteria

1. `mark -s file.md` renders only the entry file, shows no sidebar, and notes skipped local Markdown links.
2. `mark -r file.md` performs the current recursive rendering flow.
3. `mark` rejects simultaneous `--single` and `--recursive`.
4. Config persists default render mode and sidebar visibility.
5. Sidebar is hidden by default and toggles with `e`.
6. Sidebar toggle advertises the hotkey.
7. Sidebar ordering is files first, then folders, recursively.
8. Rendered pages include an icon+label theme switcher for `system`, `light`, and `dark`.
9. Default rendered theme is `system`.
10. All pre-existing tests pass and new tests cover the new behavior.

---

## Testing Requirements

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

All must pass.

---

## Forbidden Actions

- Do **not** edit `README.md`
- Do **not** merge or push to `main`
- Do **not** create or move git tags
- Do **not** change scope documents
- Do **not** declare work complete without the full handoff
- Do **not** continue through a blocking API rate-limit condition

---

## Rate-Limit Stop and Notify Instructions

If an API rate limit, token cap, service throttling response, or similar limitation prevents reliable continuation:

1. Stop work immediately
2. Do not keep retrying blindly
3. Preserve the current worktree state
4. Notify the master / Product Owner Agent via output text using this format:

```text
Item ID:        M-014
Prompt ID:      P-015
Worktree path:  .worktrees/M-014-view-controls
Branch name:    feat/M-014-view-controls
Completed:      <what was finished before stopping>
Blocked:        <what remains blocked>
Rate limit:     <exact limitation, if known>
Resume point:   <safe next step once limits clear>
```

Do not report the task as complete if rate limiting blocked reliable completion.

---

## Completion Report Format

```text
Item ID:        M-014
Prompt ID:      P-015
Worktree path:  .worktrees/M-014-view-controls
Branch name:    feat/M-014-view-controls
Summary:        <what was implemented>
Changed files:  <list>
Checks run:     cargo fmt --all, cargo clippy --all-targets --all-features -- -D warnings, cargo test
Check results:  <pass or describe failures>
Known issues:   <caveats>
README update needed: <yes / no, and why>
```
