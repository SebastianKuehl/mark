# P-027 — M-024 Wipe Command and Cleanup Surface Redesign

## Prompt ID
P-027

## Linked Items
- M-024 — Wipe command and cleanup surface redesign
- F-027 — Unified `wipe` cleanup command

## Task Objective
Replace the legacy cleanup CLI surface with a unified `wipe` subcommand:
rename `cleanup-home` to `wipe`, remove the root `--cleanup` option, and add
explicit `wipe` modes for deleting all app data, only config, only renders, or
only renders older than 30 days.

## Worktree Instructions
```sh
git worktree add .worktrees/M-024-wipe-command -b feat/M-024-wipe-command main
```
Work **only** inside `.worktrees/M-024-wipe-command`. Never commit to `main`
directly.

## Files to Inspect
- `src/cli.rs`
- `src/main.rs`
- `src/cleanup.rs`
- `src/cleanup_home.rs`
- `tests/completions.rs`
- relevant integration tests under `tests/`
- `README.md` (Product Owner will update after merge; do not edit)
- `docs/milestones/M-024-wipe-command-and-cleanup-redesign.md`
- `docs/features/F-027-unified-wipe-command.md`

## Exact Scope
- Rename the active subcommand surface from `cleanup-home` to `wipe`.
- Remove the root `--cleanup` option.
- Add explicit `wipe` modes/options for:
  - deleting the entire `.mark` folder
  - deleting only config
  - deleting only renders
  - deleting renders older than 30 days
- Update CLI help text and shell completions to reflect the new command shape.
- Update/add tests to cover the new modes and removal of legacy surface.
- Keep unrelated subcommands and render flows working.

## Implementation Constraints
- Do not edit `README.md`.
- Do not merge to `main`.
- Do not tag releases.
- Keep scope limited to M-024 / F-027.
- If you choose specific flag names for the `wipe` modes, make them explicit,
  consistent, and test-covered.

## Acceptance Criteria
1. `mark wipe` becomes the canonical cleanup command surface.
2. `cleanup-home` no longer appears in the active CLI surface.
3. `--cleanup` is removed from the root CLI options.
4. Users can target all data, only config, only renders, or only old renders
   via `wipe`.
5. Help output, completions, and tests clearly document the new options.
6. Existing unrelated subcommands and render flows continue to work.
7. `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`,
   and `cargo test` all pass.

## Testing Requirements
- Add/update tests for:
  - root CLI no longer accepting `--cleanup`
  - `wipe` help/options
  - deletion-mode parsing/behavior
  - shell completion updates
- Run:
  - `cargo fmt --all`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test`

## Forbidden Actions
- Do not edit `README.md`
- Do not merge to `main`
- Do not tag releases
- Do not change scope beyond M-024 / F-027

## Rate-Limit Notice
If you hit an API rate limit that prevents reliable continuation, stop
immediately and notify the master via output text. Do not continue.

## Completion Report Format
Report back with:
- Item IDs completed: M-024, F-027
- Prompt ID: P-027
- Worktree: `.worktrees/M-024-wipe-command`
- Branch: `feat/M-024-wipe-command`
- Summary of changes made
- Files changed
- Tests/checks run and results
- Any known issues or follow-ups
- Whether `README.md` needs updating (recommendation only)
