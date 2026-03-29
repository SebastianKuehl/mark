# F-015 — Render Mode Flags and Persistent Defaults

## Feature ID
F-015

## Title
Add explicit single/recursive render modes and persist their defaults

## User Value
Users need a predictable way to choose whether `mark` should render only the file they asked for or recursively materialize an entire linked documentation set. They also want that preference to persist without repeating flags on every invocation.

## Scope Details

### CLI render modes

Add mutually exclusive flags:

- `--single`, `-s`
- `--recursive`, `-r`

If neither is passed, `mark` uses the persisted config default. The hardcoded fallback remains recursive mode.

### Single-file mode

- Only the input file is rendered.
- Linked local Markdown files are not rendered or rewritten.
- The rendered page does not include the sidebar.
- The CLI prints a concise note listing skipped local Markdown links when any are present.

### Recursive mode

- Preserves the current recursive linked-Markdown behavior.
- Continues to render linked files into the same run directory and rewrite links between them.

### Config defaults

Extend `~/.mark/config.toml` with a persistent render-mode setting and sidebar-default setting.

Reasonable command surface:

- `mark config set-render-mode <single|recursive>`
- `mark config set-sidebar <hidden|visible>`

CLI flags override config, and config overrides the hardcoded default.

## Dependencies
- M-013 / F-014

## Acceptance Criteria
1. `mark -s file.md` renders only `file.md`.
2. `mark -r file.md` performs recursive linked rendering as before.
3. Passing both flags is rejected with a clear argument-conflict error.
4. Single-file mode emits a clear note when local Markdown links were skipped.
5. Sidebar is omitted in single-file mode.
6. Config can persist render-mode and sidebar defaults.
7. Tests cover CLI/config precedence and single-vs-recursive behavior.

## Priority
High

## Milestone
M-014

## Status
released
