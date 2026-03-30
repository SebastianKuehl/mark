# F-027 — Unified `wipe` Cleanup Command

## Feature ID
F-027

## Title
Replace `cleanup-home` and `--cleanup` with a unified `wipe` subcommand

## User Value
Users should have one clear cleanup entry point instead of split destructive and
maintenance surfaces spread between a subcommand and a root flag.

## Scope Details
- Rename `cleanup-home` to `wipe`.
- Remove the root `--cleanup` option.
- Add explicit `wipe` modes/options for:
  - deleting the entire `.mark` folder
  - deleting only config
  - deleting only renders
  - deleting renders older than 30 days
- Update CLI help text, shell completions, docs, and tests to match the new
  cleanup surface.
- Keep destructive behavior explicit and understandable.

## Dependencies
- M-024 — Wipe command and cleanup surface redesign
- Existing cleanup and app-directory deletion logic

## Acceptance Criteria
1. `mark wipe` becomes the canonical cleanup command surface.
2. `cleanup-home` no longer appears in the active CLI surface.
3. `--cleanup` is removed from the root CLI options.
4. Users can target all data, only config, only renders, or only old renders
   via `wipe`.
5. Help output and tests clearly document the new options.
6. Existing unrelated subcommands and render flows continue to work.

## Priority
High

## Milestone
M-024

## Status
ready
