# F-025 — Zen Mode

## Feature ID
F-025

## Title
Add `z` hotkey for zen mode

## User Value
Readers should be able to temporarily remove interface chrome and focus on the
document content with a single keypress, without losing the actual letter
content being read.

## Scope Details
- Add a new `z` hotkey that toggles zen mode on and off.
- Zen mode hides reader UI chrome such as sidebar/config/export controls.
- Zen mode hides the visual letter treatment itself while keeping the document
  content visible and readable.
- In zen mode, the whole page background adopts the normal letter background
  color instead of the old outer page background color.
- Toggling `z` again restores the normal reader shell.

## Dependencies
- F-019 — Config menu hotkeys and live reader-layout preview polish
- F-023 — Sidebar search and reader-shell polish

## Acceptance Criteria
1. Pressing `z` enables zen mode.
2. Pressing `z` again disables zen mode.
3. Zen mode hides reader UI elements.
4. Zen mode removes the visible letter container styling without hiding the
   document content inside it.
5. In zen mode, the full page background switches to the letter background
   color.
6. Existing navigation and content rendering continue to work when zen mode is
   toggled.
7. Tests cover the presence of the new hotkey wiring and zen-mode shell output
   where practical.

## Priority
High

## Milestone
M-021

## Status
released
