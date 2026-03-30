# B-011 — Config Sidebar Presentation Regressions

## Bug ID
B-011

## Title
Config sidebar overlaps header controls and lacks matching presentation polish

## Severity
Medium — the panel remains usable, but it obscures adjacent controls and feels
unfinished compared with the left sidebar.

## Symptoms
1. The right config sidebar opens on top of the export/config buttons.
2. The left hierarchy sidebar has a slide-in animation, but the right sidebar
   does not.
3. If a reader-layout field is cleared, the UI can temporarily hold an empty
   value instead of snapping to a stable default.

## Expected Behavior
1. The config sidebar should open without covering the header controls.
2. The right sidebar should animate similarly to the left hierarchy sidebar.
3. Empty reader-layout fields should fall back to sensible defaults.

## Actual Behavior
The panel covers the controls that opened it, appears abruptly, and allows blank
layout input state.

## Reproduction Steps
1. Run `mark <file>.md`.
2. Open the config sidebar with `c` or the gear button.
3. Observe the panel positioning and opening behavior.
4. Clear a reader-layout input and inspect the preview/command behavior.

## Affected Area
- `src/render.rs` — shell markup, JS input normalization, and sidebar CSS

## Acceptance Criteria for Fix
1. The config sidebar no longer opens on top of the export/config buttons.
2. The right sidebar has an intentional slide-in/open-close animation.
3. Empty layout inputs revert to defined defaults before preview/command output.
4. Existing config interactions remain functional.
5. Verification passes with `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.

## Status
released
