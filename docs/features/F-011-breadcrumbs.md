# F-011 — Breadcrumb Navigation on Linked Pages

## Feature ID
F-011

## Title
Breadcrumb trail on pages rendered below the entry-point

## User Value
When navigating deep into a multi-file rendered documentation set, the user can always see where they are relative to the entry-point and click any ancestor to jump back up.

## Scope Details

- Every rendered HTML page that was linked from another page (i.e. not the entry-point itself) shows a breadcrumb bar at the top of the content area.
- The breadcrumb reflects the **path taken during BFS discovery** from the entry-point to this file (shortest path / discovery path is acceptable).
- Format: `Home > chapter1.md > appendix.md` where each segment except the last is a clickable link to that rendered file.
- The entry-point page itself does NOT show a breadcrumb (it is the root).
- Breadcrumb is injected into the HTML template, above the main content, styled consistently with the existing light/dark theme.

## Dependencies
- F-010 (merged — provides BFS render pipeline and link_map)
- B-001 (must be merged first — same `main.rs` area)

## Acceptance Criteria
1. Pages rendered as linked descendants show a breadcrumb trail.
2. Each ancestor segment is a working link to the rendered HTML file.
3. The current page's name is shown as plain text (not a link).
4. The entry-point page shows no breadcrumb.
5. Breadcrumb renders correctly in both light and dark themes.
6. All existing tests pass; new tests cover breadcrumb HTML injection.

## Priority
Medium

## Milestone
M-011

## Status
blocked — waiting on B-001 merge
