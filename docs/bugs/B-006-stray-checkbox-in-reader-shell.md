# B-006 — Stray Unlabeled Checkbox Appears in the Reader Shell

## Bug ID
B-006

## Title
An unlabeled checkbox renders in the top-left corner of the page and appears to do nothing

## Severity
Medium — the control looks broken, confuses users, and suggests an incomplete or inaccessible interaction in the page chrome.

## Symptoms
1. Render a document with the current reader shell.
2. Observe a very small checkbox with no label in the top-left corner of the page.
3. Toggle it on and off.
4. No visible behavior changes and no explanatory text is present.

## Expected Behavior
No stray or unlabeled form control should appear in the page chrome. All rendered controls should be intentional, discoverable, and have observable behavior or accessible labeling.

## Actual Behavior
A tiny checkbox appears with no visible text and no obvious effect when toggled.

## Reproduction Steps
1. Run `mark <some-file>.md`.
2. Open the rendered page in the browser.
3. Inspect the top-left corner of the page.
4. Observe the unlabeled checkbox.

## Affected Area
- `src/render.rs` — rendered shell markup and control wiring
- `src/style.css` — control visibility and layout styling

## Acceptance Criteria for Fix
1. No unlabeled checkbox is visible in the top-left page chrome.
2. Any underlying control responsible for that artifact is either removed or correctly hidden without breaking intended behavior.
3. Reader controls remain keyboard-accessible and visually understandable after the fix.
4. Tests cover the rendered shell output so the artifact does not regress.

## Status
released
