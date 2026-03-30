# B-012 — Root CLI Allows FILE and COMMAND Shape Overlap

## Bug ID
B-012

## Title
Top-level CLI currently mixes optional `[FILE]` and `[COMMAND]`

## Severity
High — the command surface is ambiguous, help text is misleading, and shell
completion can suggest invalid-looking combinations.

## Symptoms
1. The CLI currently presents as `mark [OPTIONS] [FILE] [COMMAND]`.
2. That shape implies a single invocation may validly combine a file positional
   and a top-level subcommand.
3. Shell completions can surface follow-up suggestions that reflect the mixed
   shape instead of the intended either/or behavior.

## Expected Behavior
The root command should have two distinct valid forms:
- `mark [OPTIONS] [FILE]`
- `mark [OPTIONS] [COMMAND]`

An invocation should not combine both a root file positional and a root
subcommand.

## Actual Behavior
The current CLI structure exposes both as optional at the same level, producing
combined help/completion behavior inconsistent with the desired UX.

## Reproduction Steps
1. Inspect the generated help output for `mark`.
2. Observe the root usage line includes both `[FILE]` and `[COMMAND]`.
3. Generate completions and inspect the root completion suggestions after typing
   a file token.

## Affected Area
- `src/cli.rs` — root command structure
- `tests/completions.rs` — completion expectations
- `src/main.rs` — any manual validation tied to the root shape

## Acceptance Criteria for Fix
1. Root help output presents file and command usage as separate alternatives.
2. The parser rejects mixed root invocations cleanly.
3. Root shell completions no longer suggest invalid file/command combinations.
4. Regression tests cover both help/parse behavior and completions.

## Status
released
