# mark project specification

You are a senior Rust engineer. Build an entire production-quality Rust CLI
project from scratch.

## Project name

`mark`

## Goal

Create a cross-platform CLI named `mark` that takes a Markdown file, renders it
to HTML, stores the rendered HTML in a hidden folder named `.mark` in the
user’s home directory, opens the rendered file in the system’s default browser,
and automatically deletes rendered files older than 30 days.

The project must work on:
- Linux
- macOS
- Windows

## Important constraints

- The CLI command must be `mark`
- The hidden folder must be named `.mark`
- The project should be fully created by the agent, including code, scripts,
  documentation, and tests
- Prefer a clean, maintainable implementation over unnecessary complexity
- Use stable Rust only; do not use nightly features
- Make sensible implementation choices and document them
- Avoid unnecessary dependencies

## Functional requirements

### 1. CLI behavior

Primary invocation:

```text
mark path/to/file.md
```

The command should:
1. Validate that the input file exists
2. Read the Markdown file
3. Render the Markdown to HTML
4. Save the HTML into the app folder under the user’s home directory
5. Open the rendered HTML in the system default browser
6. Delete previously rendered HTML files older than 30 days

The program should:
- print clear user-facing messages, including the output path
- return non-zero exit codes on failure

### 2. Storage layout

Use a hidden directory named `.mark` in the user’s home directory.

Expected runtime layout:

- Linux/macOS:
  - `~/.mark/`
- Windows:
  - `%USERPROFILE%\.mark\`

Inside `.mark`, create at least:
- `.mark/rendered/` for generated HTML files
- `.mark/bin/` for installed CLI binary

Notes:
- On Unix-like systems, the dot-prefixed folder is naturally hidden
- On Windows, the installer should additionally try to mark `.mark` as hidden

### 3. Rendered file behavior

- Render the Markdown into a complete standalone HTML document
- Include basic embedded CSS for readable viewing
- Do not depend on external network resources for styling or scripts
- Use UTF-8
- Create unique output filenames to avoid collisions
- Recommended format:
  - `<source-stem>-<timestamp>-<short-hash>.html`
- Store all generated files only in `.mark/rendered/`

### 4. Cleanup behavior

- On every normal run, automatically delete rendered HTML files in
  `.mark/rendered/` that are older than 30 days
- Use file modified time for age checks
- Only delete files created in the rendered output directory
- Only delete rendered HTML files, not arbitrary files
- Cleanup should be best-effort:
  - if one file cannot be deleted, continue and report a warning

Also provide a cleanup-only mode:

```text
mark --cleanup
```

This should perform cleanup and print a summary without rendering anything.

### 5. Browser opening

- After rendering, open the resulting HTML file in the system default browser
- This must work on Linux, macOS, and Windows
- Handle failure gracefully with a clear error message

Also provide:

```text
mark --no-open path/to/file.md
```

This should render and store the HTML but not open the browser.

### 6. CLI UX

Use a polished CLI parser and support at least:
- `mark <FILE>`
- `mark --cleanup`
- `mark --no-open <FILE>`
- `mark --help`
- `mark --version`

Behavior rules:
- `--cleanup` should not require an input file
- normal render mode requires an input file
- invalid combinations should show a useful help/error message

## Implementation guidance

### 7. Rust crate choices

Use stable, common crates. Good choices include:
- `clap` for CLI parsing
- `anyhow` or `thiserror` for error handling
- `pulldown-cmark` or `comrak` for Markdown rendering
- `dirs`, `directories`, or `home` for locating the home directory
- `open` or `webbrowser` for opening the browser
- `chrono` or `time` for timestamp and age logic
- standard library where possible

Keep dependencies reasonable. Do not add unnecessary crates.

### 8. Project structure

Create a clean project layout similar to:

```text
Cargo.toml
Cargo.lock
src/main.rs
src/cli.rs
src/render.rs
src/storage.rs
src/cleanup.rs
src/browser.rs
src/error.rs
scripts/install.sh
scripts/install.ps1
scripts/uninstall.sh
scripts/uninstall.ps1
README.md
.gitignore
LICENSE
```

You may adjust module names if needed, but keep the structure clean and easy to
follow.

### 9. Install scripts

Create installation scripts that automatically hook the CLI into the user’s
system configuration.

#### Linux/macOS install script

File: `scripts/install.sh`

Requirements:
- build the project in release mode
- copy the built binary into `~/.mark/bin/mark`
- create `~/.mark/` and `~/.mark/rendered/` if they do not exist
- ensure `~/.mark/bin` is on the user PATH
- make PATH setup idempotent; do not duplicate entries
- update the appropriate shell config where reasonable:
  - detect and support at least bash and zsh
  - support fish if practical
  - use `.profile` as a fallback
- do not require sudo
- print clear next steps if the shell needs to be restarted
- if Rust/Cargo is missing, fail with a clear message and exact instructions

#### Windows install script

File: `scripts/install.ps1`

Requirements:
- build the project in release mode
- copy the built binary into `%USERPROFILE%\.mark\bin\mark.exe`
- create `%USERPROFILE%\.mark\` and `%USERPROFILE%\.mark\rendered\`
- add `%USERPROFILE%\.mark\bin` to the user PATH if it is not already present
- make PATH setup idempotent
- try to mark `%USERPROFILE%\.mark` as hidden
- do not require admin rights
- print a message if the terminal needs to be restarted for PATH changes to
  apply
- if Rust/Cargo is missing, fail with a clear actionable message

#### Uninstall scripts

Recommended:
- `scripts/uninstall.sh`
- `scripts/uninstall.ps1`

Behavior:
- remove installed binary from `.mark/bin`
- remove PATH hook if the installer added one
- optionally leave `.mark/rendered` intact unless the user explicitly confirms
  deletion
- document behavior clearly

### 9a. Shell completions

The CLI should support shell completion generation and installation.

Requirements:
- Provide a command to generate shell completion scripts, such as:
  - `mark completions bash`
  - `mark completions zsh`
  - `mark completions fish`
  - `mark completions powershell`
- Ensure the main file input argument is configured for file path completion
- Support at least:
  - bash
  - zsh
  - fish
  - PowerShell
- Install scripts should install or hook up completions in a user-scoped,
  idempotent way where practical
- If a shell cannot be auto-configured robustly, document manual setup in the
  README
- Do not break normal default shell filename completion behavior

### 10. Cross-platform requirements

- Must compile and run on Linux, macOS, and Windows
- Avoid OS-specific logic unless necessary
- Encapsulate platform-specific behavior cleanly
- Handle paths with spaces
- Handle relative and absolute input paths
- Use user-scoped install locations only
- Do not require global system directories

### 11. HTML output details

- Generate a complete HTML5 document
- Include:
  - proper doctype
  - `meta charset="utf-8"`
  - viewport meta tag
  - a sensible title based on the source filename
  - embedded CSS for readable typography
- Keep the HTML self-contained
- Make the page look decent for normal Markdown documents

### 11b. Persistent theme configuration

The CLI should support a persistent render theme configuration.

Requirements:
- Supported theme values:
  - `dark`
  - `light`
- The user must be able to set the theme permanently via the CLI
- A single invocation must be able to override the configured theme

Recommended CLI:
- `mark config set-theme dark`
- `mark config set-theme light`
- `mark --theme dark <FILE>`
- `mark --theme light <FILE>`

Configuration storage:
- Store configuration in `.mark/config.toml`

Theme resolution precedence:
1. per-invocation override via `--theme`
2. persisted config value from `.mark/config.toml`
3. default to `light` if no config exists

Behavior:
- invalid theme values must produce a clear error
- if the config file does not exist, rendering should still work using the
  default theme
- the selected theme must affect generated HTML styling
- rendered pages must remain self-contained
- all rendered page UI should remain readable in both dark and light themes

Documentation:
- document the config file location
- document how to set the theme permanently
- document how to override it for a single run
- document precedence rules

### 11c. Home folder cleanup command

The CLI should support a destructive cleanup command that removes the app folder
from the user's home directory.

Requirements:
- Add a command separate from `--cleanup`, for example:
  - `mark cleanup-home`
- Support non-interactive confirmation bypass:
  - `mark cleanup-home --yes`

Behavior:
- By default, require explicit confirmation before deleting the app directory
- Remove the resolved `.mark` directory recursively
- If the directory does not exist, return success with a clear no-op message
- Only delete the resolved `.mark` directory
- Never delete parent directories or arbitrary user-supplied paths

Safety:
- Validate the resolved target path before deletion
- Keep deletion logic conservative
- Handle platform-specific issues safely, especially Windows executable locking
  when the running binary lives inside `.mark/bin`

Documentation:
- document how this command differs from `mark --cleanup`
- document confirmation behavior
- document any platform-specific caveats

### 12. Safety and behavior notes

- Treat the input Markdown as local user content
- If raw HTML from Markdown is preserved, document in the README that rendering
  untrusted Markdown may be unsafe in a browser
- Never delete files outside `.mark/rendered`
- Never recurse outside the intended app directory
- Avoid panics for normal runtime errors; return useful errors instead

### 13. Testing

Add tests.

At minimum include:
- unit tests for path resolution and output filename generation
- tests for cleanup age filtering logic
- tests that rendering produces non-empty HTML from Markdown input
- integration-style test for a render flow that writes output to a temporary
  test directory without opening a browser

Design the code so browser opening can be skipped or mocked in tests.

### 14. Code quality

- Use idiomatic Rust
- Keep functions focused and modules small
- Add comments where they help, but do not over-comment obvious code
- Ensure:
  - `cargo fmt --all` passes
  - `cargo clippy --all-targets --all-features -- -D warnings` passes
  - `cargo test` passes
- Commit `Cargo.lock` since this is an application

### 15. README requirements

Write a clear `README.md` with broad setup and usage instructions.

Include:
- project overview
- feature summary
- supported platforms
- prerequisites
- installation instructions for Linux/macOS
- installation instructions for Windows
- how to run the CLI
- examples:
  - `mark README.md`
  - `mark --no-open README.md`
  - `mark --cleanup`
- where files are stored
- cleanup behavior
- how PATH setup works
- troubleshooting
- uninstall instructions
- development commands:
  - `cargo build`
  - `cargo run -- <file>`
  - `cargo test`
  - `cargo fmt`
  - `cargo clippy`

### 16. Nice-to-have improvements

If reasonable and not disruptive, include:
- a basic CI workflow for build/test on Linux, macOS, and Windows
- a small default stylesheet that makes rendered Markdown pleasant to read
- source filename and render timestamp shown in the HTML page footer or header
- helpful error messages for common mistakes
- deterministic, readable logs and messages

### 17. Acceptance criteria

The project is complete only if all of the following are true:
- Running the installer on Linux/macOS installs `mark` into `~/.mark/bin`
  and updates PATH in a user-scoped, idempotent way
- Running the installer on Windows installs `mark.exe` into
  `%USERPROFILE%\.mark\bin` and updates the user PATH idempotently
- Running `mark some.md`:
  - creates `.mark/rendered/` if needed
  - cleans up old rendered files older than 30 days
  - renders `some.md` into HTML
  - writes the HTML into `.mark/rendered/`
  - opens it in the default browser
- Running `mark --no-open some.md` renders without opening a browser
- Running `mark --cleanup` removes old rendered files and prints a summary
- The project builds on stable Rust
- Tests pass
- README is complete and usable
- The implementation is clean and maintainable

### 18. Final output expectations

Produce the full project, not just a plan.

If you can create files directly, create them.
If you cannot create files directly, output:
1. the final file tree
2. every file with full contents
3. any commands needed to build, test, and install

Do not stop after pseudocode or partial scaffolding. Deliver a complete,
working project.
