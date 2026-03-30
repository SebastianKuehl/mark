# F-026 — Default Current-Directory Markdown Render

## Feature ID
F-026

## Title
Let `mark` run without an explicit file by discovering Markdown in the current directory

## User Value
Users should be able to run `mark` from a docs folder without first naming a
specific file, reducing friction for quick local browsing.

## Scope Details
- Allow `mark` with no positional file/folder argument.
- Discover Markdown files from the current working directory.
- Render all discovered Markdown files as part of that invocation.
- Use the first discovered Markdown file as the initial page that opens.
- Return a clear error when no Markdown files are found in the current path.
- Update CLI help text and tests so the fallback behavior is explicit.

## Dependencies
- M-023 — Default current-directory Markdown entry
- Existing render pipeline and recursive file discovery behavior

## Acceptance Criteria
1. `mark` with no positional argument succeeds when the current directory
   contains Markdown files.
2. The command renders all discovered Markdown files from the current path.
3. The first discovered Markdown file becomes the initial opened page.
4. `mark` with no Markdown files in the current path exits with a clear error.
5. Existing file-based and subcommand-based CLI forms continue to work.
6. Tests cover both the success path and the no-Markdown error path.

## Priority
High

## Milestone
M-023

## Status
ready
