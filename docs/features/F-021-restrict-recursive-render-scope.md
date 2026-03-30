# F-021 — Restrict Recursive Rendering to Entry File's Directory Subtree

## Feature ID
F-021

## Title
Recursive rendering only follows links within the entry file's directory and below

## User Value
Currently, if a Markdown file links to a file outside its own directory tree,
`mark` follows that link and renders it. This produces renders that include
files the user did not intend to publish, potentially leaking content from
sibling or parent directories. Scoping recursive rendering to the entry file's
subtree makes the behavior predictable and safe.

## Scope Details
- When `mark` is run on `docs/my-file.md`, recursive link resolution must only
  follow links that resolve to files inside `docs/` (and any subdirectory
  thereof).
- Links that resolve to files outside `docs/` — i.e. parent directories, sibling
  directories, or absolute paths outside the subtree — must be treated as
  external or out-of-scope and skipped (not rendered, not copied).
- The resolved canonical path of each linked file must be compared to the
  canonical path of the entry file's parent directory. If the linked file's
  path does not start with the entry directory's path, the link is out of scope.
- Skipped out-of-scope links should be left as-is in the HTML output (or
  optionally noted in a log message), not rewritten to broken or dead paths.
- This restriction applies only to recursive Markdown link following. Non-Markdown
  asset links (images, PDFs, etc.) follow the same rule: only assets within the
  entry file's directory subtree are copied.
- Single-file rendering mode (`--single`) is unaffected (it never follows links).

## Dependencies
- F-010 — Recursive linked Markdown rendering (shipped)
- The BFS discovery loop in `src/main.rs` and link extraction in `src/render.rs`

## Acceptance Criteria
1. Running `mark docs/my-file.md` in recursive mode renders only files under
   `docs/` and its subdirectories.
2. A linked Markdown file outside `docs/` (e.g. `../other/page.md`) is not
   rendered; its link is left as-is in the output HTML.
3. A linked asset outside `docs/` (e.g. `../images/logo.png`) is not copied;
   its link is left as-is.
4. A linked file inside `docs/subdir/page.md` is rendered normally.
5. Existing tests pass. New tests cover the boundary: in-scope links are
   followed, out-of-scope links are skipped.
6. `cargo fmt`, `cargo clippy`, `cargo test` all pass with zero warnings.

## Priority
High

## Milestone
M-019 (TBD — planned as a standalone milestone)

## Status
released
