# P-024 — M-022 CLI Shape and PDF Subcommand

## Prompt ID
P-024

## Linked Items
- M-022 — CLI PDF export and command shape cleanup
- B-012 — Root CLI mixes optional file and command forms
- F-024 — CLI PDF export subcommand

## Task Objective
Fix the root CLI shape so `[FILE]` and `[COMMAND]` are mutually exclusive, then
add the `mark pdf <source> <output>` subcommand with path completion. Complete
B-012 before F-024 since F-024 depends on the clean CLI shape.

## Worktree Instructions
```
git worktree add .worktrees/M-022-cli-pdf -b feat/M-022-cli-pdf main
```
Work **only** inside `.worktrees/M-022-cli-pdf`. Never commit to `main` directly.

## Files to Inspect
- `src/cli.rs` — root command structure, `Cli` struct, `Commands` enum
- `src/main.rs` — CLI dispatch, validation logic
- `tests/completions.rs` — completion regression tests
- `docs/bugs/B-012-root-cli-file-command-overlap.md`
- `docs/features/F-024-cli-pdf-export-subcommand.md`

## Scope

### B-012 — Root CLI file/command overlap
The current `Cli` struct has both `pub file: Option<PathBuf>` and
`#[command(subcommand)] pub command: Option<Commands>` at the same level,
producing usage: `mark [OPTIONS] [FILE] [COMMAND]`.

The desired behavior is two exclusive forms:
- `mark [OPTIONS] [FILE]` — render a file
- `mark [OPTIONS] [COMMAND]` — run a subcommand

**Implementation approach:**

Use clap's `conflicts_with` or `ArgGroup` to make `file` and `command` mutually
exclusive, OR restructure the CLI so the top level is an enum of:
1. A file-render variant (positional FILE + render options)
2. A subcommand variant

The simplest correct approach is:
- Keep `file: Option<PathBuf>` and `command: Option<Commands>` as they are in
  the struct (clap requires both optional for the parser to work).
- Add manual validation in `main.rs`: if both `file.is_some()` and
  `command.is_some()`, print a clear error and exit.
- Update the `#[command(...)]` attribute or add custom usage text so help output
  shows the two forms separately rather than combined.
- Update `tests/completions.rs` to assert that the root completion no longer
  offers subcommands after a file token is already present (following the
  existing test pattern in that file).

Alternatively, if a cleaner structural solution is feasible within clap 4's
derive API without breaking existing behavior, use it — but do not over-engineer.

### F-024 — `mark pdf <source> <output>` subcommand
Add a new `Pdf` variant to `Commands`:

```rust
/// Export a Markdown file directly to PDF via headless browser print.
///
/// Example: mark pdf docs/file.md out/file.pdf
Pdf {
    /// Markdown source file to render and export.
    #[arg(value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
    source: PathBuf,
    /// Destination PDF path.
    #[arg(value_name = "OUTPUT", value_hint = clap::ValueHint::FilePath)]
    output: PathBuf,
},
```

The runtime implementation of the `pdf` subcommand:
- Render the source Markdown to a temporary HTML file (using the existing render
  pipeline with `--no-open` semantics).
- Print a clear message explaining that PDF output requires a browser: output
  the path of the rendered HTML file and instruct the user how to print to PDF
  via their browser, OR if a headless browser CLI (`chromium`, `chromium-browser`,
  `google-chrome`, `wkhtmltopdf`) is found on `PATH`, invoke it to produce the
  PDF at the output path.
- If no headless browser is available, exit with a non-zero status and a helpful
  message rather than silently failing.
- In either case, the subcommand must be reachable and parseable.

Shell completion must expose both arguments as file/path positions using
`value_hint = clap::ValueHint::FilePath`.

## Implementation Constraints
- Changes confined to `src/cli.rs`, `src/main.rs`, and test files.
- Do not touch `src/render.rs` or `src/style.css`.
- Do not edit `README.md`.
- Do not merge to `main` or tag releases.
- Keep all existing subcommands (`config`, `completions`, `cleanup-home`) intact
  and working.

## Acceptance Criteria
1. `mark file.md config set-theme light` (mixed form) is rejected with a clear
   error message.
2. `mark --help` output clearly distinguishes the two usage forms.
3. `mark pdf docs/file.md out/file.pdf` is parsed correctly by the CLI.
4. Both `pdf` arguments have `value_hint = FilePath` for shell completion.
5. Completion tests pass; root completion no longer offers subcommands after a
   file positional.
6. Existing subcommands and flags continue to work.
7. `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`,
   and `cargo test` all pass with zero warnings or failures.

## Testing Requirements
- Add a test to `src/cli.rs` (or `tests/completions.rs`) that asserts `mark
  file.md` parses as file-render and `mark pdf src dst` parses as `Commands::Pdf`.
- Update or add completion tests to assert subcommands are not offered after a
  file positional.

## Forbidden Actions
- Do not edit `README.md`
- Do not merge to `main`
- Do not tag releases
- Do not change scope beyond this prompt

## Rate-Limit Notice
If you hit an API rate limit that prevents reliable continuation, stop
immediately and notify the master via output text. Do not continue.

## Completion Report Format
Report back with:
- Item IDs completed: M-022, B-012, F-024
- Prompt ID: P-024
- Worktree: `.worktrees/M-022-cli-pdf`
- Branch: `feat/M-022-cli-pdf`
- Summary of all changes made
- Files changed
- Test results (`cargo test` output summary)
- Any known issues or follow-ups
- Whether `README.md` needs updating (recommendation only — do not edit it)
