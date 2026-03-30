# F-017 — Historical `src/index.html` Template-Shell Migration

## Feature ID
F-017

## Title
Historical migration from the Rust-built shell to the checked-in `src/index.html` template

## User Value
This feature captured the shipped v0.6.0-era migration to a checked-in template shell. It is retained as historical documentation only; the current renderer has since moved back to a self-contained HTML shell assembled in `src/render.rs` with embedded `src/style.css`.

## Historical Note

This document describes the now-superseded template-shell implementation that originally shipped in `v0.6.0`. The current runtime renderer no longer depends on `src/index.html`.

## Scope Details

### Template source of truth

- Treat `src/index.html` as the base HTML shell for rendered pages.
- Stop treating the current Rust-built shell as the primary source of layout markup.
- The implementation may sanitize or remove placeholder demo content from the template, but it must preserve the template's structural wrappers and classes that provide the intended styling.

### Markdown injection

- Replace the placeholder content currently rendered as `&lt;put rendered html here&gt;` inside the template's `.markdown-prose` region with actual rendered Markdown HTML.
- Preserve the existing paper/content container styling already present in the template.

### Sidebar injection

- Replace the placeholder hierarchy entries (`overview.html`, `docs/`, `getting-started.html`) with the actual generated file/folder tree for the current render.
- Reuse the template's sidebar wrapper, nesting structure, and styling-oriented classes/elements rather than discarding them for a different sidebar implementation.
- Preserve released sidebar behavior:
  - hidden by default unless configured otherwise
  - toggleable from the rendered page
  - files first, folders second, recursively
  - current page visibly highlighted

### Existing chrome behavior that must survive

- In-page theme switching (`system`, `light`, `dark`) must continue to work within the template shell.
- Existing Markdown rendering features such as breadcrumbs, rewritten links, and code-block actions must remain compatible with the template-based shell.
- Single-file mode should continue to omit sidebar content, even when the template is used as the page shell.

## Dependencies
- M-015
- F-016 (released sidebar/theme behaviors to preserve)
- Current render pipeline in `src/render.rs`

## Acceptance Criteria
1. `mark` uses `src/index.html` as the source template for rendered pages.
2. The content placeholder in the template is replaced with rendered Markdown output.
3. The sidebar placeholder links/folders are replaced with generated navigation using the template's existing sidebar markup/styling structure.
4. Existing sidebar and theme features from `v0.5.0` remain intact after the migration.
5. Recursive renders still show the generated navigation tree; single-file renders still avoid sidebar content.
6. Tests cover template placeholder replacement and preservation of the current navigation/theme behaviors.

## Priority
High

## Milestone
M-015

## Status
released (historical)
