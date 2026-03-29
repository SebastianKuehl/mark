# M-013 — Folder Hierarchy Preservation

## Milestone ID
M-013

## Title
Folder Hierarchy Preservation

## Objective
Preserve the source folder structure in both the rendered output directory and the sidebar navigation tree. Eliminate flat-dump layout and filename collisions. Give users a rendered output that mirrors their actual documentation structure.

## Included Features
- F-014 — Preserve folder hierarchy in rendered output and sidebar tree

## Dependencies
- v0.3.0 (all previous work merged)

## Acceptance Criteria
- Per-invocation render directory `~/.mark/rendered/<entry>-<ts>-<hash>/` created on each run.
- Source relative paths preserved inside the run directory.
- Sidebar shows a collapsible folder tree matching source hierarchy.
- 30-day cleanup operates on per-invocation directories.
- All checks pass; README updated.

## Priority
High

## Status
released

## Target Release
v0.4.0 (minor)
