# P-020 — M-018 Reader Shell UX Refinements

## Prompt ID
P-020

## Linked Items
- M-018 — Reader Shell UX Refinements
- B-007 — `c` hotkey only opens config pane, does not toggle it
- F-020 — Reader shell UX refinements: letter top-alignment, accordion terminal command, and save button

## Task Objective
Implement three UX polish items in the rendered page's reader shell, all in
`src/render.rs`:
1. Fix the `c` hotkey to toggle the config pane open/closed (B-007)
2. Align the document content letter top edge with the header chrome (F-020 §1)
3. Upgrade the "Terminal command" block to a subtle collapsed accordion with a
   full-width Save button (F-020 §2–3)

## Worktree Instructions
```
git worktree add .worktrees/M-018-reader-ux -b chore/M-018-reader-ux main
```
Work exclusively in `.worktrees/M-018-reader-ux`. Do not touch the main checkout.

## Files to Inspect
- `src/render.rs` — all HTML, inline JS, and embedded CSS for the rendered page
- `src/style.css` — external stylesheet embedded at build time
- `tests/` — existing test suite for patterns and fixtures

## Exact Scope

### 1. B-007 — `c` hotkey toggle fix
In the inline JavaScript in `src/render.rs`, locate `openConfigMenu()` and the
`keydown` handler. Change the `c` key handler so it:
- opens the config pane if it is currently closed
- closes the config pane if it is currently open
This must mirror the `e` key sidebar toggle pattern. Name the function
`toggleConfigMenu()` (or update the existing name consistently everywhere it
is referenced).

### 2. F-020 §1 — Letter top-alignment
Inspect the CSS for the main content area (`.mark-letter` or equivalent). Add
or adjust `padding-top` / `margin-top` so the visible top edge of the content
letter is flush with the bottom of the header bar, visually level with the tops
of the sidebar toggle and config buttons. This is a CSS adjustment only.

### 3. F-020 §2 — Terminal command accordion
In the config pane HTML (in `src/render.rs`), replace the static "Terminal
command" section with an HTML `<details>`/`<summary>` element that is
**collapsed by default** (`open` attribute absent). The `<summary>` text should
be minimal and understated, e.g. "Terminal command". When expanded, it shows
the existing `<code>` block and Copy button unchanged. Accordion styling must
be subtle — no heavy borders or prominent treatment. It should feel secondary.

### 4. F-020 §3 — Save button
Below the terminal command `<details>` accordion (still inside the config
pane layout section), add a "Save" button:
- Full-width relative to the reader layout section of the config pane
- **Disabled by default** (when no layout value differs from the rendered
  defaults — i.e. the values the page was rendered with)
- **Enabled** when any layout form control differs from its initial (rendered)
  value
- **On click**: copies the generated `mark config set-layout ...` command to
  the clipboard (identical to the existing Copy button mechanism) and briefly
  shows a "Saved" or "Copied ✓" confirmation label before resetting

Implementation notes for the save button:
- At page load, capture the initial values of all layout form controls
  (font size, letter width, font family, line height) into JS variables.
- On every `input`/`change` event from any layout control, compare current
  values to initial values. If any differ, enable the save button; otherwise
  disable it.
- The save button's disabled visual style should be clearly muted (low
  contrast, `cursor: not-allowed` or similar).
- The save button's enabled style should match the visual language of the rest
  of the config pane controls (not a primary CTA — keep it secondary/subtle).

## Implementation Constraints
- All changes are in `src/render.rs` (and/or `src/style.css` for CSS adjustments).
- Do NOT touch `README.md`.
- Do NOT merge to `main`.
- Do NOT tag releases.
- Do NOT change scope beyond what is listed above.
- The rendered page must remain a single self-contained HTML file — no new
  external resources.
- All existing reader shell functionality (sidebar, theme toggle, layout
  controls, copy button, hotkeys `e`, `t`) must continue to work exactly as
  before.

## Acceptance Criteria
1. Pressing `c` when config pane is closed → opens it.
2. Pressing `c` when config pane is open → closes it.
3. The top of the rendered letter is visually flush with the bottom of the
   header bar / tops of the sidebar and config buttons.
4. The "Terminal command" block is a collapsed `<details>` accordion by default.
5. Clicking the accordion summary expands/collapses it; Copy button still works.
6. A full-width "Save" button appears below the accordion.
7. Save button is disabled when layout values match rendered defaults.
8. Save button is enabled when at least one layout value differs.
9. Clicking the enabled Save button copies the command and shows a brief
   confirmation.
10. `cargo fmt --all` passes.
11. `cargo clippy --all-targets --all-features -- -D warnings` passes with zero
    warnings.
12. `cargo test` passes — all existing tests green, new tests added covering:
    - toggle behavior of `c` hotkey (HTML contains `toggleConfigMenu`)
    - save button disabled attribute present in default render
    - accordion `<details>` element present in config pane

## Forbidden Actions
- Edit `README.md`
- Merge to `main`
- Tag releases
- Change scope
- Close the task independently
- Continue after a blocking API rate-limit event

## Completion Report Format
Return a report containing:
- Item IDs: B-007, F-020, M-018
- Prompt ID: P-020
- Worktree: `.worktrees/M-018-reader-ux`
- Branch: `chore/M-018-reader-ux`
- Summary of changes made
- Files changed
- Tests run and results
- Known issues or follow-ups
- Whether README.md needs updating (recommendation only)

## Rate-Limit Stop and Notify
If you hit an API rate limit, token cap, or service throttling that prevents
reliable continuation:
1. Stop work immediately
2. Do not retry
3. Preserve current state
4. Output a rate-limit notice to the master containing: your role (anvil agent),
   item IDs (B-007/F-020/M-018), prompt ID (P-020), worktree path, branch,
   what was completed, what remains blocked, and the safe resume point
5. Do not mark the task as complete
