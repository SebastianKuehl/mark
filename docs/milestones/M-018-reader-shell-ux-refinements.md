# M-018 — Reader Shell UX Refinements

## Milestone ID
M-018

## Title
Reader Shell UX Refinements

## Objective
Address three closely related polish items that improve the visual coherence
and usability of the rendered page and config panel: fix the `c` toggle
behavior, align the content letter with the header chrome, and upgrade the
terminal command section to a collapsible accordion with a Save button.

## Included Features
- B-007 — `c` hotkey only opens config pane, does not toggle it
- F-020 — Reader shell UX refinements (letter alignment, accordion, save button)

## Dependencies
- M-017 — Reader Controls Polish (released v0.7.0)
- F-019 — Config menu hotkeys and live reader-layout preview (released)

## Acceptance Criteria
- `c` toggles the config pane open and closed.
- The document letter top edge aligns with the bottom of the header / tops of
  the sidebar and config buttons.
- The "Terminal command" block is a collapsed accordion by default in the config
  panel.
- A "Save" button appears below the accordion, full-width to the layout section.
- The Save button is disabled when no layout values differ from the render-time
  defaults; enabled once any value changes.
- Clicking Save copies the generated command to the clipboard and shows a
  brief confirmation state.
- All existing tests pass. New tests cover the toggle behavior and save button
  disabled/enabled logic.
- `cargo fmt`, `cargo clippy`, `cargo test` pass with zero warnings.

## Priority
High

## Status
released

## Target Release
v0.7.1
