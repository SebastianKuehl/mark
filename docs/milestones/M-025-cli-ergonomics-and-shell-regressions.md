# M-025 — CLI ergonomics and reader-shell regressions

## Objective

Close the next batch of CLI ergonomics gaps and reader-shell regressions so `mark` is easier to invoke, shell completions stay helpful, and zen/config presentation matches the intended polished experience.

## Included Features and Bugs

- F-028 — CLI shorthand flags and directory entry handling
- F-029 — PDF command path ergonomics and completions
- B-014 — Zen mode still does not visually collapse the page into the letter surface
- B-015 — Config pane overlays above export/config buttons incorrectly

## Dependencies

- None

## Acceptance Criteria

- `mark` accepts shorthand flags for theme and no-open, and the manual version flag uses lowercase `-v`.
- Passing a directory path such as `mark docs` or `mark docs/` behaves like invoking `mark` from inside that directory.
- `mark pdf <file> .` resolves the output path to `<file-stem>.pdf` in the current working directory.
- PDF subcommand completions still complete input/output file paths.
- Zen mode makes the full page visually adopt the letter surface instead of leaving the page on the outer background.
- The config pane renders above the export/config buttons while open.
- Validation passes with fmt, clippy, and test.

## Priority

High

## Status

in_progress

## Target Release

TBD
