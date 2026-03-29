# F-012 — Sidebar with Rendered File Hierarchy

## Feature ID
F-012

## Title
Sidebar showing the full rendered file hierarchy with current-page highlight

## User Value
Every rendered page shows a collapsible sidebar listing all files that were rendered in the same `mark` invocation. The user can see the full documentation structure at a glance and jump to any file in one click, with the current page visually highlighted.

## Scope Details

- A sidebar is injected into every rendered HTML page (including the entry-point).
- The sidebar lists all rendered files as a tree, mirroring the BFS discovery hierarchy.
- The current page is highlighted (bold or accent colour).
- All other entries are clickable links to their rendered HTML files.
- The sidebar is collapsible (a toggle button or CSS checkbox trick — no external JS libraries).
- Sidebar is styled consistently with the existing light/dark theme.
- On narrow viewports the sidebar should not break the layout (acceptable: hide or overlay).

## Dependencies
- F-010 (merged)
- F-011 (same worktree — implemented together)
- B-001 (must be merged first)

## Acceptance Criteria
1. Every rendered page includes a sidebar listing all files rendered in the invocation.
2. The current page is visually distinguished.
3. All sidebar links work correctly in the browser.
4. The sidebar is collapsible.
5. Light and dark themes both render the sidebar legibly.
6. All existing tests pass; new tests cover sidebar HTML injection.

## Priority
Medium

## Milestone
M-011

## Status
blocked — waiting on B-001 merge
