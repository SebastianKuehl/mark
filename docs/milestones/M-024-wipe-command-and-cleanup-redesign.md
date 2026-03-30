# M-024 — Wipe Command and Cleanup Surface Redesign

## Milestone ID
M-024

## Title
Replace legacy cleanup surfaces with a unified `wipe` command

## Objective
Simplify destructive and housekeeping operations by removing the legacy
`--cleanup` flag, renaming `cleanup-home` to `wipe`, and consolidating cleanup
operations under explicit `wipe` subcommand modes.

## Included Features and Bugs
- F-027 — Unified `wipe` cleanup command with scoped deletion modes

## Dependencies
- M-022 — CLI PDF export and root-command argument cleanup
- M-023 — Default current-directory Markdown entry
- Existing cleanup, cleanup-home, and config storage behavior in `src/main.rs`

## Acceptance Criteria
1. `cleanup-home` is renamed to `wipe` everywhere in the CLI surface.
2. The root `--cleanup` option is removed.
3. `mark wipe` supports explicit modes for:
   - deleting the entire `.mark` folder
   - deleting only config
   - deleting only renders
   - deleting renders older than 30 days
4. Help output, completions, and tests reflect the new command shape.
5. Legacy cleanup behavior is either removed or redirected in a clear,
   intentional way consistent with the approved CLI redesign.
6. Verification passes with `cargo fmt --all`, `cargo clippy --all-targets
   --all-features -- -D warnings`, and `cargo test`.

## Priority
High

## Status
ready

## Target Release
TBD
