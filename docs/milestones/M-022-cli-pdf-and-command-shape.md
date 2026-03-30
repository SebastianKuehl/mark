# M-022 — CLI PDF Export and Command Shape Cleanup

## Milestone ID
M-022

## Title
CLI PDF export and root-command argument cleanup

## Objective
Extend `mark` with a direct CLI PDF export command and tighten the top-level CLI
shape so file rendering and subcommand usage are cleanly separated in both
parsing and shell completion.

## Included Features and Bugs
- F-024 — Direct CLI PDF export subcommand
- B-012 — Root CLI currently mixes `[FILE]` and `[COMMAND]`

## Dependencies
- M-014 — View Controls and Render Modes
- M-019 — Recursive Render Scope Restriction
- M-020 — PDF Export and Letter Alignment Polish

## Acceptance Criteria
1. `mark pdf <target-file> <target-output-path>` is a documented CLI surface.
2. The new `pdf` subcommand has out-of-the-box shell completion for both the
   source file and destination path arguments.
3. The root CLI shape becomes `mark [OPTIONS] [FILE]` or `mark [OPTIONS]
   [COMMAND]`, but not both in the same invocation.
4. Root shell completion matches the new command shape and no longer suggests
   invalid combinations.
5. Verification passes with `cargo fmt --all`, `cargo clippy --all-targets
   --all-features -- -D warnings`, and `cargo test`.

## Priority
High

## Status
released

## Target Release
v0.10.0
