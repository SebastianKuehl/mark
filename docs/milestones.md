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

---

## Milestone 7: Code block copy actions

### Goal
- Add copy actions to rendered code blocks in the generated HTML
- Provide two separate buttons for each eligible code block:
  - `Copy`
  - `Copy clean`
- `Copy` must copy the code exactly as displayed
- `Copy clean` must copy a cleaned version of the code intended for easier
  direct terminal or editor use

### UX requirements
- Use two buttons, not one toggle-based control
- Use text plus icons, not icon-only buttons
- Recommended labels and semantics:
  - `Copy` with a clipboard/copy icon
  - `Copy clean` with a broom icon
- Buttons should be visually unobtrusive but clearly discoverable
- Show a brief success state after copying, such as:
  - `Copied`
  - `Copied clean`
- Show a clear but lightweight failure message if clipboard access fails

### Clean-copy behavior
- `Copy clean` must remove full-line comments only
- Do not remove inline comments
- Do not attempt aggressive parsing that could corrupt code
- Only support clean-copy for explicitly supported languages
- Recommended initial supported languages:
  - `bash`
  - `sh`
  - `zsh`
  - `fish`
  - `powershell`
  - `python`
  - `rust`
  - `javascript`
  - `typescript`
- For unsupported or unknown languages:
  - keep the normal `Copy` button
  - hide or disable `Copy clean`

### Comment stripping rules
Implement conservative full-line comment stripping only.

Examples:
- shell-like languages:
  - strip lines whose trimmed form starts with `#`
- Python:
  - strip lines whose trimmed form starts with `#`
- Rust, JavaScript, TypeScript:
  - strip lines whose trimmed form starts with `//`
- PowerShell:
  - strip lines whose trimmed form starts with `#`

Do not attempt for v1:
- block comment removal such as `/* ... */`
- HTML comment stripping
- SQL comment stripping
- inline comment stripping
- language parsing beyond simple full-line detection

### Technical requirements
- The rendered HTML must remain self-contained
- Do not depend on external JavaScript or CSS resources
- Add the button UI during HTML rendering or post-processing of rendered code
  blocks
- Clipboard support must work from the locally opened rendered HTML page where
  browser permissions allow it
- Keep the implementation robust for multiple code blocks per page

### Testing
Add at least:
- tests for clean-copy transformation logic
- tests that supported language detection behaves as expected
- tests or render verification that code blocks receive the expected copy UI
- ensure normal rendering still works

### Documentation
Update the README with:
- what the two buttons do
- which languages support `Copy clean`
- limitations of clean-copy behavior
- that `Copy clean` removes only full-line comments

### Deliverables
- rendered code blocks with `Copy` and `Copy clean` actions
- conservative and tested clean-copy logic
- no regressions to normal Markdown rendering
- updated documentation
---

## Milestone 8: Persistent theme configuration

### Goal
- Add support for a persistent render theme:
  - `dark`
  - `light`
- The user must be able to set the theme permanently via the CLI
- The configured theme must affect rendered HTML output
- The configured theme must be overridable for a single invocation by passing a
  CLI argument

### CLI requirements
Add persistent theme configuration commands such as:
- `mark config set-theme dark`
- `mark config set-theme light`

Add render-time override support such as:
- `mark --theme dark README.md`
- `mark --theme light README.md`

### Behavior
- Store the persistent configuration in `.mark/config.toml`
- Theme precedence must be:
  1. CLI override via `--theme`
  2. persisted config value
  3. default to `light` if no config exists
- Invalid theme values must fail with a clear error
- Rendering must apply the resolved theme consistently to the generated HTML
- Existing render features must continue to work

### Rendering requirements
- The generated HTML must render appropriately for both dark and light themes
- Existing UI elements in the rendered page must remain readable in both themes
- Keep the HTML self-contained

### Testing
Add at least:
- tests for config read and write behavior
- tests for theme precedence
- tests that rendered HTML changes according to the selected theme

### Documentation
Update the README with:
- how to set the theme permanently
- how to override it for one run
- where the config file is stored
- the precedence rules

### Deliverables
- persistent theme config support
- single-run theme override support
- updated rendering for dark and light themes
- tests and documentation