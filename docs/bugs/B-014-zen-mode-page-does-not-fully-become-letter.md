# B-014 — Zen mode page does not fully become the letter surface

## Severity

Medium

## Symptoms

Zen mode still leaves the page on the outer page background in some cases instead of visually turning the full page into the letter surface.

## Expected Behavior

When zen mode is enabled, the letter should visually disappear into the page so the whole page appears to take on the letter background color.

## Actual Behavior

Zen mode can still sample or retain the wrong background value, so the page keeps the outer page background instead of matching the letter surface reliably.

## Reproduction Steps

1. Render a Markdown file.
2. Toggle zen mode.
3. Compare the page background to the letter background.

## Affected Area

- `src/render.rs`
- embedded reader-shell CSS/JS

## Acceptance Criteria for Fix

- Zen mode reliably applies the effective letter background color to the page.
- Theme changes while zen mode is active keep the page synced to the effective letter background.
- Regression tests cover the intended visual contract.

## Status

released
