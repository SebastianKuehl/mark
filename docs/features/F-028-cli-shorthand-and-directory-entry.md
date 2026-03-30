# F-028 — CLI shorthand flags and directory entry handling

## User Value

Users should be able to invoke `mark` with shorter, more discoverable flags and point it at a directory directly instead of needing to `cd` first.

## Scope Details

- Add short forms for `--theme` and `--no-open`.
- Change the manual version flag from uppercase `-V` to lowercase `-v`.
- Accept a directory path passed as the positional argument.
- When the positional argument is a directory, discover top-level Markdown files inside that directory, render them the same way as no-argument mode, and open the first discovered Markdown file.
- Support directory arguments with and without a trailing slash.

## Dependencies

- M-025

## Acceptance Criteria

- `mark -t dark file.md` overrides the theme for the invocation.
- `mark -n file.md` renders without opening the browser.
- `mark -v` prints the version string.
- `mark docs` and `mark docs/` behave like running `mark` inside `docs/`.
- Directory mode errors clearly when the target directory contains no Markdown files.

## Priority

High

## Milestone

M-025

## Status

in_progress
