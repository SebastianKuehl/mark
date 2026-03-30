# M-015 — Historical Template-Driven Render Shell

## Milestone ID
M-015

## Title
Historical Template-Driven Render Shell

## Objective
Capture the historical v0.6.0 milestone that moved rendered pages onto `src/index.html`. That implementation has since been superseded by the current self-contained shell assembled in `src/render.rs` with embedded `src/style.css`.

## Included Features
- F-017 — Use `src/index.html` as the rendered page template

## Dependencies
- v0.5.1 (M-014 and B-004 released)
- Existing recursive render/sidebar data from `src/render.rs`

## Historical Note

This milestone remains part of the release record, but the current renderer no longer depends on `src/index.html`.

## Acceptance Criteria
- Rendered pages are built from `src/index.html` rather than a separately assembled HTML document string.
- The template's `markdown-prose` content area receives the rendered Markdown in place of the placeholder paragraph.
- The template's sidebar hierarchy area is populated from real rendered files/folders instead of placeholder links.
- The implementation preserves the template's existing shadcn-flavored markup/classes for sidebar and content wrappers so the provided styling remains intact.
- Existing shipped behavior for sidebar visibility, files-first recursive ordering, current-page highlighting, and in-page theme switching remains available after the template migration.
- All checks pass; `README.md` is updated after merge if user-visible behavior needs explanation.

## Priority
High

## Status
released

## Target Release
v0.6.0 (minor)
