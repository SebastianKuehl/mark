# F-029 — PDF path ergonomics and completions

## User Value

The PDF command should behave naturally with directory-like output targets and keep shell completions useful for both source and output paths.

## Scope Details

- Teach `mark pdf <file> .` to resolve the output path to `<file-stem>.pdf` in the current working directory.
- Preserve the existing fallback HTML behavior when no headless browser is installed, but ensure it uses the resolved output path cleanly.
- Fix shell completions so the `pdf` subcommand completes file paths for both arguments instead of dropping to plain token completion.

## Dependencies

- M-025

## Acceptance Criteria

- `mark pdf OVERVIEW.md .` resolves to `./OVERVIEW.pdf`.
- The fallback HTML path follows the resolved PDF path when no browser is available.
- Generated completions continue to offer file path suggestions for both `source` and `output` under `mark pdf`.

## Priority

High

## Milestone

M-025

## Status

released
