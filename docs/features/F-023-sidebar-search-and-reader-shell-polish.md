# F-023 — Sidebar Search and Reader-Shell Polish

## Feature ID
F-023

## Title
Add left-sidebar search and polish reader-shell controls

## User Value
Users should be able to quickly filter the hierarchy, use a config panel that
feels visually consistent with the rest of the shell, and interact with
reader-layout controls that remain clear even when fields are blank or the pane
contains more content.

## Scope Details
- Add a simple search input to the left hierarchy sidebar to filter visible
  files/folders.
- Give the right config sidebar the same slide-in motion language as the left
  sidebar.
- Normalize empty reader-layout inputs back to sensible defaults before live
  preview and before generating the terminal command.
- Remove the reader-layout descriptive paragraph from the config panel.
- Move the Save button above the "Terminal command" accordion and preserve a bit
  more vertical space between the layout inputs and the Save button.
- Restyle the "Copy" button in the accordion header as a tertiary action.
- Update hotkey copy so the export shortcut shows `Shift` before `Primary`.
- Update the `E` hotkey description to say "Toggle hierarchy".
- Keep hotkey copy and shell wiring compatible with a new `z` zen-mode toggle.

## Dependencies
- F-019 — Config menu hotkeys and live reader-layout preview polish
- F-020 — Reader shell UX refinements

## Acceptance Criteria
1. The left hierarchy sidebar contains a simple search input that filters the
   visible navigation entries.
2. The right config sidebar animates in/out instead of appearing abruptly.
3. Blank layout inputs fall back to stable default values for preview and
   command generation.
4. The reader-layout description text is removed.
5. The Save button appears above the terminal-command accordion with visibly
   improved spacing from the inputs.
6. The Copy button remains always visible in the accordion header and uses a
   tertiary visual treatment.
7. Hotkey text shows `Shift` before `Primary` for export and `E` is described as
   "Toggle hierarchy".
8. Tests cover the updated shell output where practical.

## Priority
High

## Milestone
M-021

## Status
released
