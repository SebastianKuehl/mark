# M-019 — Recursive Render Scope Restriction

## Milestone ID
M-019

## Title
Recursive Render Scope Restriction

## Objective
Make recursive link following safe and predictable by restricting it to the
entry file's directory subtree. Files and assets that resolve outside that
boundary are silently skipped rather than rendered or copied.

## Included Features
- F-021 — Restrict recursive rendering to entry file's directory subtree

## Dependencies
- F-010 — Recursive linked Markdown rendering (shipped)
- BFS discovery loop in `src/main.rs`

## Acceptance Criteria
- Recursive rendering follows only links that resolve within the entry file's
  parent directory and its subdirectories.
- Out-of-scope Markdown links are not rendered; their HTML link targets are
  left unchanged.
- Out-of-scope asset links are not copied; their HTML link targets are left
  unchanged.
- In-scope links and assets continue to work as before.
- New tests cover the in-scope / out-of-scope boundary.
- `cargo fmt`, `cargo clippy`, `cargo test` pass with zero warnings.

## Priority
High

## Status
ready

## Target Release
TBD
