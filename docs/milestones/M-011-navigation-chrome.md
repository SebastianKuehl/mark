# M-011 — Navigation Chrome (Breadcrumbs + Sidebar)

## Milestone ID
M-011

## Title
Navigation Chrome

## Objective
Every rendered page in a multi-file set provides contextual navigation: a breadcrumb trail showing the path from the entry-point, and a sidebar listing the full rendered file hierarchy with the current page highlighted.

## Included Features
- F-011 — Breadcrumb navigation on linked pages
- F-012 — Sidebar with rendered file hierarchy

## Dependencies
- F-010 (released in v0.2.0) — provides BFS pipeline and link_map
- B-001 must be merged before M-011 work begins (overlapping `main.rs` changes)

## Acceptance Criteria
- All pages below the entry-point show a working breadcrumb trail.
- All pages (including entry-point) show a collapsible sidebar.
- Both features work in light and dark themes.
- `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check` all pass.
- README updated to document breadcrumbs and sidebar.

## Priority
Medium

## Status
blocked — waiting on B-001

## Target Release
v0.3.0 (minor)
