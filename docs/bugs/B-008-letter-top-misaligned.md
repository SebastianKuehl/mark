# B-008 — Letter Top Edge Misaligned with Header Buttons

## Bug ID
B-008

## Title
Letter content area starts at the bottom of the header buttons, not the same height

## Severity
Medium — visual misalignment makes the page feel unbalanced; the content letter
sits too low relative to the header chrome.

## Symptoms
The top visible edge of the rendered document letter (content area) is flush
with the **bottom** of the sidebar toggle and config buttons in the header bar,
not with their **top** edge. There is a visible gap between the top of the
page chrome and the start of the letter.

## Expected Behavior
The letter's top edge should be visually level with the **top** of the sidebar
toggle button and config button — i.e. the letter and the buttons share the
same vertical starting point within the layout.

## Actual Behavior
The letter starts at or near the bottom edge of the buttons (the full height of
the button row is used as top padding), pushing the letter down unnecessarily.

## Reproduction Steps
1. Run `mark <file>.md` and open the rendered page.
2. Observe the sidebar toggle (☰) and config (⚙) buttons in the top-right.
3. Note that the top edge of the letter content area is vertically below the
   bottom of those buttons.

## Affected Area
- `src/style.css` — `padding-top` or `margin-top` on `.mark-content-wrapper`
  or equivalent selector
- Possibly `src/render.rs` if padding is set via inline CSS

## Acceptance Criteria for Fix
1. The visible top of the letter content box is at the same vertical position
   as the top of the sidebar toggle and config buttons.
2. No existing layout, sidebar, or config pane functionality is broken.
3. `cargo fmt`, `cargo clippy`, `cargo test` pass with zero warnings.

## Status
ready
