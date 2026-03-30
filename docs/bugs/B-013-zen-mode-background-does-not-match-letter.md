# B-013 — Zen Mode Background Does Not Match Letter Color

## Bug ID
B-013

## Title
Zen mode background does not adapt to the current letter color

## Severity
Medium — zen mode still works, but the reading surface does not match the
intended distraction-free presentation and can feel visually wrong under
different theme/layout combinations.

## Symptoms
1. Open a rendered page and toggle zen mode with `z`.
2. The outer page background does not consistently match the current letter
   background color.
3. The result can leave a visible mismatch between the reading surface and the
   document body area that zen mode is supposed to simplify.

## Expected Behavior
When zen mode is enabled, the full page background should adapt to the same
effective color as the current letter background so the page reads as one
continuous surface.

## Actual Behavior
Zen mode can leave the page background on a different color than the letter's
current background instead of fully adopting the letter color.

## Reproduction Steps
1. Run `mark <file>.md`.
2. Open the rendered output in the browser.
3. Toggle zen mode with `z`.
4. Compare the page background against the visible letter color before zen mode
   removed the letter shell.

## Affected Area
- `src/render.rs` — zen-mode client-side styling and background synchronization
- F-025 — Zen mode

## Acceptance Criteria for Fix
1. Enabling zen mode updates the page background to the effective current letter
   background color.
2. The background remains correct across supported themes.
3. Disabling zen mode restores the normal reader shell styling.
4. Regression tests cover the zen-mode background behavior where practical.

## Status
released
