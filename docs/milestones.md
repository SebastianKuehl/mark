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
