# B-004 — Shell Completion Suggests Subcommands After FILE Argument

## Bug ID
B-004

## Title
Shell completion suggests subcommands after the positional `FILE` argument is already present

## Severity
Medium — command execution itself still works, but completion UX is misleading and can append invalid-looking follow-up tokens such as `cleanup-home` after `mark some-file.md`.

## Symptoms
1. User types `mark some-file.md`
2. User presses Tab twice in a shell using the generated completion script
3. Completion offers top-level subcommands such as `cleanup-home`, `config`, `completions`, and `help`
4. The command line can end up looking like `mark some-file.md cleanup-home`

## Expected Behavior
Once the positional `FILE` argument has already been supplied, top-level subcommands should no longer be suggested for the same command invocation. Completion should either offer only flags still valid in that context or no further completions.

## Actual Behavior
The generated Bash completion function remains in the top-level `mark` completion state after consuming a non-subcommand token, so it keeps offering root-level subcommands even though `FILE` is already present.

## Reproduction Steps
1. Generate/load Bash completions for `mark`
2. Type `mark some-file.md`
3. Press Tab twice
4. Observe suggestions like `cleanup-home`, `config`, `completions`, and `help`

## Affected Area
- `src/cli.rs` — optional positional `file` plus optional subcommand structure
- `src/main.rs` — completion script generation entrypoint
- `tests/completions.rs` — no regression test currently covers this completion state

## Investigation Notes
- Reproduced directly from the repository-generated Bash completion script using:
  - `cargo run --quiet -- completions bash`
  - shell invocation of `_mark` with `COMP_WORDS=(mark some-file.md "")`
- The generated top-level `mark)` case still returns:
  - flags
  - `[FILE]`
  - subcommands (`completions`, `config`, `cleanup-home`, `help`)
- This indicates a repository-side completion-generation / CLI-structure bug rather than a purely local shell setup issue.

## Acceptance Criteria for Fix
1. After a positional `FILE` argument is already present, shell completion no longer suggests top-level subcommands for the same invocation.
2. Existing valid completions remain intact for root-level subcommands and flags.
3. Regression coverage is added so the bad completion state is tested.
4. If a full behavioral fix requires a documented limitation instead of a code-only fix, the implementation must explicitly justify that outcome and update the appropriate docs.

## Status
in_progress
