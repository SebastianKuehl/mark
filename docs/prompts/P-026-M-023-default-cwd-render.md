# P-026 — M-023 Default Current-Directory Markdown Entry

## Prompt ID
P-026

## Linked Items
- M-023 — Default current-directory Markdown entry
- F-026 — Default current-directory Markdown render

## Task Objective
Allow `mark` to be invoked with no explicit file/folder argument. In that case,
discover Markdown files in the current working directory, render all discovered
Markdown files, open the first discovered file as the initial page, and return
a clear error if none are found.

## Worktree Instructions
```sh
git worktree add .worktrees/M-023-default-cwd -b feat/M-023-default-cwd main
```
Work **only** inside `.worktrees/M-023-default-cwd`. Never commit to `main`
directly.

## Files to Inspect
- `src/cli.rs`
- `src/main.rs`
- relevant storage/render orchestration tests
- `tests/render_integration.rs`
- `tests/completions.rs`
- `docs/milestones/M-023-default-cwd-render-entrypoint.md`
- `docs/features/F-026-default-cwd-render.md`

## Exact Scope
- Make `mark` without a positional file/folder argument fall back to discovering
  Markdown files in the current working directory.
- Render all discovered Markdown files in that current-directory context.
- Use the first discovered Markdown file as the initial page that opens.
- Return a clear error message when no Markdown files are found.
- Update help text and tests to describe/cover the no-argument behavior.
- Preserve existing file-based and subcommand-based CLI behavior.

## Implementation Constraints
- Keep scope limited to M-023 / F-026.
- Do not edit `README.md`.
- Do not merge to `main`.
- Do not tag releases.
- Keep command-shape behavior from M-022 intact.

## Acceptance Criteria
1. Running `mark` with no positional file/folder argument searches the current
   directory for Markdown files.
2. If Markdown files are found, `mark` renders all discovered Markdown files in
   that current directory context.
3. The initially opened page is the first discovered Markdown file.
4. If no Markdown files are found, `mark` exits with a clear error message.
5. Help output and tests describe and cover the no-argument behavior.
6. Existing file-based and subcommand-based CLI forms continue to work.
7. `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`,
   and `cargo test` all pass.

## Testing Requirements
- Add or update tests for:
  - no-argument current-directory Markdown discovery success path
  - no-argument no-Markdown error path
  - existing CLI parsing behavior remaining intact
- Run:
  - `cargo fmt --all`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test`

## Forbidden Actions
- Do not edit `README.md`
- Do not merge to `main`
- Do not tag releases
- Do not change scope beyond M-023 / F-026

## Rate-Limit Notice
If you hit an API rate limit that prevents reliable continuation, stop
immediately and notify the master via output text. Do not continue.

## Completion Report Format
Report back with:
- Item IDs completed: M-023, F-026
- Prompt ID: P-026
- Worktree: `.worktrees/M-023-default-cwd`
- Branch: `feat/M-023-default-cwd`
- Summary of changes made
- Files changed
- Tests/checks run and results
- Any known issues or follow-ups
- Whether `README.md` needs updating (recommendation only)
