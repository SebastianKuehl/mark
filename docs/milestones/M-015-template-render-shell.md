# M-015 — Template-Driven Render Shell

## Milestone ID
M-015

## Title
Template-Driven Render Shell

## Objective
Adopt `src/index.html` as the HTML shell used for rendered Markdown pages so `mark` reuses the shipped shadcn-based layout and component structure instead of building its own page chrome in Rust. Preserve current navigation behavior while swapping placeholder content and placeholder sidebar entries for generated Markdown, files, and folders.

## Included Features
- F-017 — Use `src/index.html` as the rendered page template

## Dependencies
- v0.5.1 (M-014 and B-004 released)
- Existing recursive render/sidebar data from `src/render.rs`

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
