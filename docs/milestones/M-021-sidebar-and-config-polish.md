# M-021 — Sidebar Search and Config Panel Polish

## Milestone ID
M-021

## Title
Sidebar search, theme persistence, and config panel polish

## Objective
Polish the rendered reader chrome so both sidebars behave consistently, the
user's live theme choice survives in-app navigation, and the growing config UI
remains usable and intentional.

## Included Features and Bugs
- F-023 — Sidebar search and reader-shell polish
- F-025 — Zen mode
- B-010 — User-selected theme resets after hierarchy navigation
- B-011 — Config sidebar overlaps header controls and lacks matching motion polish

## Dependencies
- M-017 — Reader Controls Polish
- M-018 — Reader Shell UX Refinements
- M-020 — PDF Export and Letter Alignment Polish

## Acceptance Criteria
1. The config sidebar opens below or clear of the export/config buttons instead
   of covering them.
2. The right config sidebar uses a slide-in animation consistent with the left
   hierarchy sidebar.
3. A simple search field exists in the left hierarchy sidebar.
4. Clicking another file in the hierarchy preserves the user-selected theme if
   it differs from the original render theme.
5. Empty reader-layout inputs normalize to sensible defaults instead of
   persisting blank state.
6. The reader-layout panel reflects the requested button ordering, spacing, and
   wording changes.
7. Hotkey copy uses the requested labels and key order.
8. Pressing `z` toggles a zen mode that hides UI chrome, removes the visible
   letter shell while keeping document content visible, and promotes the page
   background to the letter background color.
9. Verification passes with `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.

## Priority
High

## Status
released

## Target Release
v0.10.0
