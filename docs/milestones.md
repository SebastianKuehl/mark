# milestones

## Milestone 1: Project scaffold and CLI skeleton

### Goal
- Create the Rust project structure for `mark`
- Add dependencies
- Implement CLI parsing with:
  - `mark <FILE>`
  - `mark --cleanup`
  - `mark --no-open <FILE>`
  - `--help`
  - `--version`
- Implement app path resolution for:
  - home dir
  - `.mark`
  - `.mark/rendered`
  - `.mark/bin`
- Create module layout
- Add initial tests for path resolution and output filename generation
- Ensure `cargo fmt`, `cargo check`, and `cargo test` pass

### Do not implement yet
- browser opening
- cleanup deletion logic
- install scripts
- README polish
- CI

### Deliverables
- working CLI skeleton
- project file tree
- compilable code
- basic unit tests

---

## Milestone 2: Markdown rendering and output writing

### Goal
- Read Markdown input file
- Validate file existence
- Render Markdown to full standalone HTML
- Add embedded CSS
- Generate unique output filenames
- Write HTML to `.mark/rendered`
- Support `--no-open`
- Print useful output path info
- Add tests for rendering non-empty HTML and render flow using temp dirs

### Do not implement yet
- browser opening in normal mode
- cleanup deletion logic
- install scripts
- README polish
- CI

### Deliverables
- end-to-end render and write path
- tests for render output

---

## Milestone 3: Browser opening and cleanup

### Goal
- Open rendered HTML in default browser in normal mode
- Implement `--cleanup`
- On normal render runs, delete rendered HTML files older than 30 days
- Use file modified time
- Cleanup must be best-effort
- Only delete HTML files in `.mark/rendered`
- Add tests for cleanup age filtering
- Ensure browser opening can be skipped or mocked in tests

### Deliverables
- complete runtime functionality
- tested cleanup logic

---

## Milestone 4: Install and uninstall scripts

### Goal
- Create `scripts/install.sh`
- Create `scripts/install.ps1`
- Create `scripts/uninstall.sh`
- Create `scripts/uninstall.ps1`
- Install binary into user-scoped `.mark/bin`
- Ensure PATH setup is idempotent
- Linux/macOS:
  - support bash and zsh
  - use `.profile` fallback
  - support fish if practical
- Windows:
  - update user PATH
  - try to mark `.mark` hidden
- Do not require sudo or admin rights
- Print helpful messages and next steps

### Deliverables
- working install and uninstall scripts
- clear behavior and safety

---

## Milestone 5: Documentation, polish, CI

### Goal
- Write full `README.md`
- Add `LICENSE`
- Add `.gitignore`
- Add CI workflow for build and test on Linux, macOS, and Windows if practical
- Verify clippy is clean:
  - `cargo clippy --all-targets --all-features -- -D warnings`
- Final polish of errors and messages

### Deliverables
- production-ready repo
- complete documentation
- quality checks in place

---

## Milestone 6: Shell autocomplete and completion installation

### Goal
- Add first-class shell completion support for `mark`
- Ensure the positional file argument completes as a file path
- Support completion generation for at least:
  - bash
  - zsh
  - fish
  - PowerShell
- Add a CLI command to generate completion scripts, for example:
  - `mark completions bash`
  - `mark completions zsh`
  - `mark completions fish`
  - `mark completions powershell`
- Ensure completions work for:
  - flags such as `--help`, `--version`, `--cleanup`, `--no-open`
  - subcommands related to completion generation
  - file path completion for the Markdown input argument
- Update install scripts so shell completion is installed or hooked up in a
  user-scoped, idempotent way where practical
- Preserve normal shell file completion behavior and do not break it

### Requirements
- Use idiomatic Rust support for completion generation, such as `clap_complete`
  if using `clap`
- Mark the input file argument with the appropriate file path hint so generated
  completion scripts know it is a path-like argument
- Completion setup must be user-scoped only
- Do not require sudo or administrator rights
- Installer changes must be idempotent
- If automatic completion installation is not practical for a shell, document
  the manual setup clearly in the README
- PowerShell support should be included for Windows
- Do not break existing CLI behavior

### zsh install strategy — oh-my-zsh vs plain zsh

Appending `fpath` + `compinit` to the end of `~/.zshrc` does **not** work when
oh-my-zsh (or similar frameworks) are present, because `compinit` is called by
the framework earlier in shell init. Adding `fpath` after that point has no
effect.

The correct approach:
- **oh-my-zsh detected** (`$ZSH` dir exists): drop `_mark` into
  `$ZSH_CUSTOM/completions/` (default `~/.oh-my-zsh/custom/completions/`).
  oh-my-zsh adds that directory to `fpath` automatically before calling
  `compinit`, so no `~/.zshrc` edits are needed.
- **Plain zsh** (no framework): add `fpath=(~/.zsh/completions $fpath)` and
  `autoload -Uz compinit && compinit` to `~/.zshrc`. This only works reliably
  if no prior `compinit` call exists in the file.

Detection logic in `install.sh`: check for the `$ZSH` environment variable or
the `~/.oh-my-zsh` directory.

### Testing
Add at least:
- tests or smoke checks that completion scripts can be generated successfully
- tests or assertions that the file input argument is configured as a file path
  hint if feasible
- verification that normal build and test commands still pass

### Documentation
Update the README with:
- supported shells
- how completions are installed automatically
- how to manually install completions if desired
- examples for generating completion scripts manually

### Deliverables
- working completion generation in the CLI
- installer integration for shell completion where practical
- documentation for setup and troubleshooting
- no regression in normal CLI behavior
