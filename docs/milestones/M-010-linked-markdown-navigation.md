# M-010 — Linked Markdown Navigation

## Milestone ID
M-010

## Title
Linked Markdown Navigation

## Objective
Enable `mark` to render a complete multi-file Markdown documentation set in a single invocation. When the entry-point file links to other local `.md` files, those files are rendered automatically and all inter-document links are rewritten to their rendered HTML counterparts, making the full set navigable in the browser.

## Included Features
- F-010 — Recursive linked Markdown rendering with HTML link rewriting

## Dependencies
- M1–M9 (all shipped; no blockers)

## Acceptance Criteria
- Running `mark overview.md` renders all transitively linked local Markdown files.
- Browser navigation between rendered pages works via standard HTML links.
- Circular references are handled safely.
- `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` all pass.
- README updated to document the new navigation behaviour.

## Priority
High

## Status
in_progress

## Target Release
v0.2.0 (minor — new user-facing feature)
