# B-007 — `c` Hotkey Only Opens Config Pane, Does Not Toggle It

## Bug ID
B-007

## Title
Pressing `c` opens the config pane but pressing it again does not close it

## Severity
Medium — the `c` hotkey is not self-consistent with the expected toggle pattern
used by every other keyboard shortcut on the page (`e` toggles sidebar, `t`
toggles theme). Users expect a second press of `c` to dismiss the panel.

## Symptoms
1. Render any document.
2. Press `c` — config pane opens.
3. Press `c` again — config pane does not close; nothing happens.

## Expected Behavior
`c` acts as a toggle: first press opens the config pane, second press closes
it, alternating on every subsequent press — matching the established hotkey
pattern for `e` (sidebar) and `t` (theme).

## Actual Behavior
`c` only opens the config pane. Pressing it again while the pane is open has
no effect.

## Reproduction Steps
1. Run `mark <some-file>.md`.
2. Open the rendered page.
3. Press `c` — pane opens.
4. Press `c` again — pane remains open.

## Affected Area
- `src/render.rs` — `openConfigMenu()` function and `keydown` handler in
  the inline JavaScript

## Acceptance Criteria for Fix
1. Pressing `c` when the config pane is closed opens it.
2. Pressing `c` when the config pane is open closes it.
3. The toggle behavior mirrors `e` (sidebar) — second press dismisses.
4. Existing `t` hotkey behavior is unaffected.
5. Tests cover the toggle round-trip.

## Status
released
