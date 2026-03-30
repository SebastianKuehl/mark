# M-023 — Default Current-Directory Markdown Entry

## Milestone ID
M-023

## Title
Allow `mark` to render from the current directory when no file is provided

## Objective
Make the top-level CLI more forgiving by allowing `mark` with no positional
file/folder argument to discover Markdown files in the current working
directory, render them, and open the first discovered file as the entry page.

## Included Features and Bugs
- F-026 — Default to current-directory Markdown discovery when no file is given

## Dependencies
- M-022 — CLI PDF export and root-command argument cleanup
- Existing recursive render and output orchestration in `src/main.rs`

## Acceptance Criteria
1. Running `mark` with no positional file/folder argument searches the current
   directory for Markdown files.
2. If Markdown files are found, `mark` renders all discovered Markdown files in
   that current directory context.
3. The initially opened page is the first discovered Markdown file.
4. If no Markdown files are found, `mark` exits with a clear error message.
5. Help output and tests describe and cover the no-argument behavior.
6. Verification passes with `cargo fmt --all`, `cargo clippy --all-targets
   --all-features -- -D warnings`, and `cargo test`.

## Priority
High

## Status
released

## Target Release
v0.11.0
