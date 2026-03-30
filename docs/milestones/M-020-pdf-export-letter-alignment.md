# M-020 — PDF Export and Letter Alignment Polish

## Milestone ID
M-020

## Title
PDF Export and Letter Alignment Polish

## Objective
Add a PDF export button to the reader chrome and fix the residual letter
top-alignment bug so the content area and header buttons share the same
vertical start position.

## Included Features and Bugs
- B-008 — Letter top edge misaligned with header buttons
- B-009 — Terminal command accordion card styling + Copy button placement
- F-022 — PDF export button with file picker

## Dependencies
- M-018 — Header button layout (released v0.7.1)

## Acceptance Criteria
- Letter top edge is visually level with the top of the sidebar and config
  buttons.
- A PDF export button (download icon) appears left of the ⚙ button.
- Clicking it opens a file picker (where supported) then triggers
  `window.print()` with a print stylesheet that hides chrome.
- Terminal command accordion is flat (no border/background/shadow), integrated into the config pane surface.
- Copy button is always visible in the accordion `<summary>` row, right-aligned.
- `cargo fmt`, `cargo clippy`, `cargo test` pass with zero warnings.

## Priority
Medium

## Status
released

## Target Release
v0.9.0
