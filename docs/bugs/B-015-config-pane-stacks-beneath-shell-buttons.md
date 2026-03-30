# B-015 — Config pane stacks beneath shell buttons

## Severity

Medium

## Symptoms

When the config pane opens, the export and config buttons can still sit above it, making the pane look visually incorrect and partially obscured.

## Expected Behavior

The config pane should layer above the shell buttons while it is open.

## Actual Behavior

The fixed-position shell buttons have a higher stacking order than the pane, so they remain above the sliding panel.

## Reproduction Steps

1. Render a Markdown file.
2. Open the config pane.
3. Observe the export/config buttons relative to the pane edge.

## Affected Area

- `src/render.rs`
- embedded reader-shell CSS

## Acceptance Criteria for Fix

- The config pane layers above the export and config buttons.
- The shell buttons remain usable when the pane is closed.
- Regression coverage captures the intended layering contract in rendered HTML/CSS tests where practical.

## Status

in_progress
