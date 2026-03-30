# F-020 — Reader Shell UX Refinements

## Feature ID
F-020

## Title
Reader shell UX refinements: letter top-alignment, accordion terminal command, and save button

## User Value
Three complementary polish items that make the rendered reading surface and
config panel feel intentional and complete:

1. The document letter (content area) should visually start at the same
   height as the sidebar toggle button and config button in the header bar,
   so the page feels spatially coherent rather than top-heavy or misaligned.
2. The "Terminal command" section in the config pane is noisy for users who
   just want to tweak the layout visually. Collapsing it into a subtle
   accordion keeps the panel clean while preserving the save workflow.
3. A "Save" button below the accordion runs the generated `mark config
   set-layout ...` command automatically, removing the copy-paste step. The
   button must only be active when the user has changed at least one
   layout value from its current persisted default — if nothing has changed,
   the button remains disabled to avoid no-op saves.

## Scope Details

### 1. Letter top-alignment
- Inspect the CSS and HTML structure that positions the main content letter
  relative to the header chrome.
- Adjust top padding/margin or `padding-top` on the content area so the
  visible top of the letter (article/content box) is visually flush with the
  bottom edge of the header bar, at the same vertical position as the tops of
  the sidebar toggle button and config button.
- This is a CSS-only change; no markup changes expected unless layout requires it.

### 2. Terminal command as a subtle accordion
- Replace the current static "Terminal command" block in the config pane with
  an `<details>`/`<summary>` (or equivalent JS toggle) that is collapsed by
  default.
- The summary label should be minimal and understated (e.g. "Terminal command").
- When expanded, it shows the existing `<code>` block with the generated
  command and the Copy button exactly as before.
- The accordion styling should be subtle — no heavy borders or bold treatment;
  it should feel like a secondary reference item, not a primary control.

### 3. Save button
- Add a "Save" button below the terminal command accordion.
- Clicking it executes the generated `mark config set-layout ...` command via
  a `fetch` to a local helper, a clipboard write + notification, or — most
  practically — writes the config values by calling a small local endpoint if
  one exists, or emits the command via the page's existing mechanism. Since
  `mark` renders static HTML, the practical implementation is:
  - If `mark` provides a local dev server or IPC channel during serving: use that.
  - Otherwise: the save button should write the `mark config set-layout` values
    directly to `~/.mark/config.toml` via a `fetch` POST to a local endpoint
    that `mark` must expose during rendering/serving, OR trigger an OS-level
    custom URL scheme if one is registered, OR — simplest and most realistic —
    use the Clipboard API to copy the command and show a brief "Copied — paste
    in your terminal" tooltip, matching the existing Copy button behavior but
    with a more prominent, labeled call-to-action.
  - **The simplest correct implementation**: the Save button copies the command
    to the clipboard (same as the existing Copy button) and shows a brief
    success state. Its distinguishing trait is that it is **disabled until the
    user changes at least one layout value from the currently-applied defaults**
    (i.e. from the values the page was rendered with).
- The button stretches to the full width of the reader layout section of the
  config panel (not just the terminal command subsection).
- When the button is disabled (no changes), it has a visually muted appearance.
- When the button becomes active (at least one change), it becomes clearly
  clickable.
- After clicking, the button briefly shows a "Saved" or "Copied" confirmation
  state before returning to active.

## Dependencies
- F-019 — Config menu hotkeys and live reader-layout preview polish (shipped)
- B-007 — `c` hotkey not toggling (should be fixed in same milestone)

## Acceptance Criteria
1. The top of the rendered letter (content box) aligns vertically with the
   bottom of the header bar / top of the sidebar toggle and config buttons.
2. The "Terminal command" block is wrapped in a collapsed accordion by default.
3. The accordion can be expanded/collapsed by clicking its summary.
4. The Copy button within the accordion continues to work as before.
5. A "Save" button appears below the accordion, full-width to the layout section.
6. The Save button is disabled when no layout values differ from the rendered
   defaults.
7. When one or more values differ, the Save button becomes enabled.
8. Clicking Save copies the generated command to the clipboard and shows a
   brief success state.
9. Tests cover the save button disabled/enabled state logic and the accordion
   presence.
10. `cargo fmt`, `cargo clippy`, `cargo test` all pass.

## Priority
High

## Milestone
M-018

## Status
released
