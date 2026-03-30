# P-028 — M-025 CLI ergonomics and reader-shell regressions

## Linked Items

- M-025
- F-028
- F-029
- B-014
- B-015

## Task Objective

Implement the M-025 batch in a dedicated worktree. This batch combines CLI ergonomics, directory entry handling, PDF path/completion fixes, and two reader-shell regressions that all touch overlapping CLI/render/completion surfaces.

## Exact Scope

- Add short flags for `--theme` and `--no-open`.
- Change the manual version flag from `-V` to `-v`.
- Allow the positional argument to be either a Markdown file or a directory.
- When a directory is passed, discover top-level Markdown files in that directory, render them the same way as no-argument mode, and open the first discovered Markdown file.
- Support both `mark docs` and `mark docs/`.
- Implement `mark pdf <file> .` so `.` resolves to `<file-stem>.pdf` in the current working directory.
- Preserve the existing no-browser fallback behavior while using the resolved PDF path.
- Fix shell completions so the `pdf` subcommand completes file paths for both positional args.
- Fix zen mode so the page reliably takes on the effective letter background while active.
- Fix shell chrome layering so the config pane appears above the export/config buttons when open.

## Files or Areas to Inspect

- `src/cli.rs`
- `src/main.rs`
- `src/render.rs`
- `src/completions.rs`
- `tests/completions.rs`
- `tests/view_controls.rs`
- add/adjust CLI/PDF integration tests as needed

## Implementation Constraints

- Work only in the assigned task scope.
- Reuse existing Markdown discovery helpers where possible instead of duplicating traversal logic.
- Keep README untouched; only the Product Owner Agent may update it.
- Do not merge to `main` or tag releases.
- Preserve current behavior unless it directly conflicts with the requested changes.
- If you hit an API rate limit or similar usage cap, stop immediately and notify the master via output text instead of continuing.

## Acceptance Criteria

- All acceptance criteria from M-025/F-028/F-029/B-014/B-015 are satisfied.
- New behavior is covered by tests.
- `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` pass in the worktree.

## Testing Requirements

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`

## Worktree Instructions

- Create a git worktree from `main` at `.worktrees/M-025-cli-ergonomics`.
- Create and use branch `feat/M-025-cli-ergonomics`.
- Work only inside that worktree.

## Forbidden Actions

- Do not edit `README.md`.
- Do not merge to `main`.
- Do not tag releases.
- Do not expand scope beyond the items listed above.
- Do not continue after a blocking rate-limit event.

## Completion Report Format

Provide:

- item IDs completed
- prompt ID
- worktree path
- branch name
- summary of changes
- changed files
- tests/checks run and results
- known issues or follow-ups
- whether `README.md` needs a Product Owner update

## Rate-Limit Stop and Notify

If an API rate limit, token cap, or similar limitation prevents reliable continuation:

- stop work immediately
- report the affected item IDs and prompt ID
- describe what was completed before stopping
- describe what remains blocked
- state that work stopped because of API rate limiting
- provide the safe resume point
