# P-023 — M-021 Reader Shell Polish

## Prompt ID
P-023

## Linked Items
- M-021 — Sidebar search, theme persistence, and config panel polish
- F-023 — Sidebar search and reader-shell polish
- F-025 — Zen mode
- B-010 — User-selected theme resets after hierarchy navigation
- B-011 — Config sidebar overlaps header controls and lacks matching motion polish

## Task Objective
Implement all M-021 items in a single worktree. All changes are in
`src/render.rs` and `src/style.css` (inline CSS/JS inside `build_html_document`
and `render_theme_controls`).

## Worktree Instructions
```
git worktree add .worktrees/M-021-reader-polish -b feat/M-021-reader-polish main
```
Work **only** inside `.worktrees/M-021-reader-polish`. Never commit to `main`
directly.

## Files to Inspect
- `src/render.rs` — all shell HTML, CSS, and JS
- `src/style.css` — base stylesheet
- `tests/view_controls.rs` — integration tests for shell output
- `docs/features/F-023-sidebar-search-and-reader-shell-polish.md`
- `docs/features/F-025-zen-mode.md`
- `docs/bugs/B-010-theme-resets-on-navigation.md`
- `docs/bugs/B-011-config-sidebar-presentation-regressions.md`

## Scope — implement all of the following

### B-011 — Config sidebar presentation regressions
1. The config sidebar (`#mark-theme-menu`, `.mark-theme-menu`) must not overlap
   the export and config buttons when open. Currently `z-index:45` puts it
   behind the buttons (`z-index:50`). Fix so buttons remain accessible.
   Simplest fix: push the sidebar inner content padding-top high enough that it
   clears the button row, OR lower the right-control z-index so the open sidebar
   is behind it only when the pane is not the active focus target. The preferred
   solution is: keep the right-control buttons at z-index 50 and ensure the
   sidebar opens at z-index 40, but add enough `padding-top` in
   `.mark-theme-menu-inner` so content is never hidden under the button row.
2. The right config sidebar must have a CSS transition/slide-in animation that
   matches the left sidebar animation style (already uses `transform` +
   `transition`). Use the same pattern: `transform: translateX(100%)` when
   hidden → `translateX(0)` when shown. Remove the `hidden` attribute approach
   for visibility — switch to a CSS class toggle instead (e.g.
   `mark-theme-menu--open`) and rely on `transform`+`opacity` transition.
   Keep `aria-expanded` wiring on the toggle button up to date.
3. Empty reader-layout number inputs must normalize to sensible defaults
   (the same defaults used when the page renders) before updating live preview
   and before generating the terminal command. Add an `input` event handler that
   replaces blank/NaN values with the default before processing.

### F-023 — Sidebar search and reader-shell polish
4. Add a simple `<input type="search">` at the top of the left hierarchy sidebar
   (`#mark-sidebar`) that filters the visible nav entries. Filtering should hide
   any `<li>` items whose text content does not case-insensitively match the
   query. Directory nodes whose children all become hidden should also be hidden.
   Clearing the search restores the full tree.
5. The right config sidebar slide-in (from point 2) must feel symmetric with the
   left sidebar's existing animation.
6. Remove the `<p class="mark-layout-help">` description paragraph from the
   reader-layout form (the one that says "Adjust the values below, then run the
   generated command…"). Keep the form fields and the terminal command accordion.
7. Move the Save button (`#mark-save-layout`) to appear **above** the terminal
   command `<details>` accordion. Add a bit more spacing between the layout
   inputs and the Save button (at least `1rem` gap).
8. Restyle the `Copy` button (`.mark-layout-copy`) in the accordion summary as a
   tertiary button — visually muted border with transparent background, smaller
   padding, reduced prominence relative to the Save button.
9. Update the hotkey list so the export shortcut reads
   `<kbd>Shift</kbd><span>+</span><kbd>Primary</kbd><span>+</span><kbd>E</kbd>`
   (Shift before Primary).
10. Update the `E` hotkey description from "Toggle sidebar" to "Toggle
    hierarchy".

### F-025 — Zen mode
11. Add a `z` hotkey that toggles zen mode.
12. Zen mode must:
    - Hide all reader UI chrome (sidebar button, export button, config button,
      config sidebar, left hierarchy sidebar).
    - Remove the visible letter container styling (border, background,
      box-shadow, border-radius on `.mark-letter` or equivalent) while keeping
      document text content visible and readable.
    - Set the full page background (`document.documentElement` or `body`) to the
      same background color as the letter container — i.e. adopt the letter's
      `--card` or `background` color so the page feels like a plain reading
      surface.
13. Pressing `z` again restores the full reader shell.
14. Add `Z` to the hotkey section: `<kbd>Z</kbd><span>Zen mode</span>`.

### B-010 — Theme persists across hierarchy navigation
15. When a user changes the theme with `t` or the config controls, store that
    choice in `sessionStorage` (key: `mark-theme`).
16. On each page load, check `sessionStorage['mark-theme']`. If present, apply it
    immediately (before paint if possible) overriding the data-theme set by the
    render.
17. When navigating to another file in the hierarchy sidebar, the receiving page
    will pick up the stored theme from sessionStorage automatically.
18. Existing `setTheme` function must write to sessionStorage as well as the DOM.

## Implementation Constraints
- All HTML, CSS, and JS lives inside `src/render.rs` — the inline string in
  `build_html_document` and the `render_theme_controls` function.
- `src/style.css` contains the base document/print styles; shell CSS is in the
  inline `<style>` block in `build_html_document`.
- Do not extract templates to separate files.
- Do not change the Rust API surface of public functions (`render_markdown`,
  `render_markdown_rewriting_links`, `chrome`).
- Do not edit `README.md`.
- Do not merge to `main` or tag releases.

## Acceptance Criteria (all must pass)
1. Config sidebar no longer overlaps the export/config buttons.
2. Right sidebar slides in/out with a CSS transition.
3. Empty layout inputs normalize to defaults instead of producing blank output.
4. Left sidebar has a working search input that filters the tree.
5. Reader-layout help paragraph is removed.
6. Save button is above the terminal-command accordion.
7. Copy button has tertiary visual treatment.
8. Export hotkey shows Shift before Primary; `E` hotkey reads "Toggle hierarchy".
9. `z` toggles zen mode: hides chrome, removes letter styling, page BG = letter BG.
10. `Z` appears in the hotkey list.
11. Theme change is stored to sessionStorage and applied on each page load.
12. `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`,
    and `cargo test` all pass with zero warnings or failures.

## Testing Requirements
- Update or add tests in `src/render.rs` mod tests and/or `tests/view_controls.rs`
  to cover:
  - presence of the sidebar search input
  - zen mode hotkey wiring (`z` key handler present)
  - `Z` hotkey entry in the hotkey list
  - Shift before Primary in export hotkey display
  - `sessionStorage` write in `setTheme`
  - Save button positioned before accordion in HTML string order
  - Absence of the reader-layout help paragraph

## Forbidden Actions
- Do not edit `README.md`
- Do not merge to `main`
- Do not tag releases
- Do not change scope beyond this prompt

## Rate-Limit Notice
If you hit an API rate limit that prevents reliable continuation, stop
immediately and notify the master via output text. Do not continue.

## Completion Report Format
Report back with:
- Item IDs completed: M-021, F-023, F-025, B-010, B-011
- Prompt ID: P-023
- Worktree: `.worktrees/M-021-reader-polish`
- Branch: `feat/M-021-reader-polish`
- Summary of all changes made
- Files changed
- Test results (`cargo test` output summary)
- Any known issues or follow-ups
- Whether `README.md` needs updating (recommendation only — do not edit it)
