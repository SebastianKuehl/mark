# agent rules

Follow these rules for all work in this repository.

- Read `docs/project-spec.md` before making changes
- Implement only the requested milestone
- Do not work on later milestones unless explicitly asked
- Keep the project cross-platform:
  - Linux
  - macOS
  - Windows
- Use stable Rust only
- Prefer simple, maintainable code
- Do not add unnecessary dependencies
- Do not remove existing tests unless they are replaced with better ones
- Preserve working behavior unless the milestone explicitly changes it
- Keep modules focused and avoid putting everything into `main.rs`
- Use `PathBuf` and standard path APIs; do not build paths with naive string
  concatenation
- Handle paths with spaces correctly
- Keep browser opening test-friendly and skippable in tests
- Cleanup logic must never delete outside `.mark/rendered`
- Installer behavior must be user-scoped only
- Do not require sudo or administrator rights
- PATH modifications must be idempotent
- Prefer clear user-facing errors over panics
- If a milestone cannot be fully completed, stop and explain exactly what
  remains

After making changes, run these commands when possible:
- `cargo fmt --all`
- `cargo check`
- `cargo test`

For the final polish milestone, also run:
- `cargo clippy --all-targets --all-features -- -D warnings`

At the end of each milestone, provide:
- a summary of files changed
- commands run
- any known issues or follow-up items
