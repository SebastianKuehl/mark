# F-024 — CLI PDF Export Subcommand

## Feature ID
F-024

## Title
Add `mark pdf` for direct PDF export

## User Value
Users should be able to generate a PDF directly from the CLI without relying on
the browser export UI, while still benefiting from normal path completion for
both the source Markdown file and destination PDF path.

## Scope Details
- Add a new top-level subcommand:
  - `mark pdf <target file> <target output path>`
- The source argument should complete like the existing file positional.
- The destination argument should support standard file/path completion out of
  the box.
- The command should render/export the requested target to the specified PDF
  path.
- The command must integrate with the existing CLI structure, help output, and
  completions.

## Dependencies
- M-022 — CLI PDF export and command shape cleanup
- Existing render pipeline and PDF export capabilities

## Acceptance Criteria
1. `mark pdf docs/file.md out/file.pdf` is accepted by the CLI.
2. Help output documents the new subcommand and its arguments clearly.
3. Shell completions treat both arguments as path/file positions.
4. Existing commands such as `config`, `cleanup-home`, and `completions` remain
   available and unaffected.
5. Tests cover argument parsing and completion behavior for the new subcommand.

## Priority
High

## Milestone
M-022

## Status
released
