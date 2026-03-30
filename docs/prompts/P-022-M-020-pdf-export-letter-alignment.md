# P-022 — M-020 PDF export + reader shell polish

## Prompt ID
P-022

## Linked Items
- M-020 — PDF Export and Letter Alignment Polish
- F-022 — PDF export button with file picker
- B-008 — Letter top edge misaligned with header buttons
- B-009 — Terminal command accordion visual issues

## Assigned Agent
Copilot CLI `anvil`

## Task Objective
Implement the remaining M-020 backlog in one dedicated worktree:
1. fix the residual top alignment issue so the paper/letter starts at the same
   visual height as the sidebar and config buttons
2. add a PDF export button immediately left of the config button
3. make the terminal command accordion visually flat/integrated with the config
   pane, with the Copy button always visible in the accordion header and aligned
   to the right

## Files and Areas to Inspect
- `src/render.rs`
- `src/style.css`
- `src/main.rs` only if needed for platform-safe command generation or export wiring
- `tests/render_integration.rs`
- any existing render/layout tests related to the config pane or shell chrome

## Required Worktree Setup
Create a dedicated worktree before making changes:

```sh
git worktree add .worktrees/M-020-pdf-export -b chore/M-020-pdf-export main
```

Do all work inside `.worktrees/M-020-pdf-export` on branch
`chore/M-020-pdf-export`.

## Exact Scope

### 1. Letter top alignment
- The paper/letter must visually start at the same top position as the sidebar
  and config buttons, not below their bottom edge.
- Adjust layout spacing in the embedded stylesheet and any related shell CSS so
  this holds on desktop and mobile.

### 2. PDF export button
- Add a new button immediately left of the config button.
- Use a clear download/export-style icon.
- Clicking it should let the user choose a destination path and filename where
  supported, then export the currently viewed rendered page as a PDF.
- The intended browser-first approach is:
  - use `window.showSaveFilePicker(...)` when available to let the user choose
    a file name/path
  - then trigger printing/export flow
  - include a graceful fallback when that API is unavailable or the picker is
    cancelled
- Ensure the rendered/printed output hides reader chrome (buttons, sidebar,
  config pane, etc.) and focuses on the letter/document content.

### 3. Terminal command accordion polish
- The accordion must have no card/island styling:
  - no separate border
  - no separate background block
  - no separate shadow
- It should visually sit on the config pane surface.
- The Copy button must be inside the accordion header (`<summary>` row),
  always visible, and aligned to the right even while collapsed.
- Expanded content must still display the generated command correctly.

### 4. Tests and coverage
- Add or update tests to cover the new markup/CSS/JS expectations where
  practical.
- Validate that existing behavior still works.

## Implementation Constraints
- Do not edit `README.md`.
- Do not merge, tag, or update milestone/feature/bug statuses.
- Keep changes focused on M-020 scope only.
- Reuse existing render-shell patterns and helpers where possible.
- Preserve current theme/config functionality.
- Avoid introducing fragile browser-specific behavior without fallback.

## Acceptance Criteria
1. Letter top edge visually aligns with the top of the sidebar/config buttons.
2. A PDF export button exists immediately left of the config button.
3. PDF export flow uses a file picker where supported and still provides a safe
   fallback path otherwise.
4. Print/export output hides UI chrome and renders the document cleanly.
5. Terminal command accordion is flat/integrated with the config pane.
6. Copy button is always visible in the accordion header and right-aligned.
7. `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`,
   and `cargo test` all pass.

## Forbidden Actions
- Do not edit `README.md`
- Do not merge to `main`
- Do not create or move tags
- Do not change docs status records on your own
- Do not continue through a blocking API rate limit

## Completion Handoff Format
Report back with:

```text
Item ID:        M-020
Prompt ID:      P-022
Worktree path:  .worktrees/M-020-pdf-export
Branch name:    chore/M-020-pdf-export
Summary:        <short description of changes>
Changed files:  <list>
Checks run:     cargo fmt, cargo clippy, cargo test
Check results:  <pass/fail details>
Known issues:   <follow-ups or caveats>
README update needed: <yes/no and why>
```

## Rate-Limit Stop and Notify
If you hit an API rate limit, token cap, service throttling response, or similar
limit that prevents reliable continuation:
1. stop immediately
2. preserve the current state
3. do not pretend the task is complete
4. notify the master via output text with item ID, prompt ID, worktree, branch,
   completed work, blocked remainder, exact limit if known, and safe resume point
