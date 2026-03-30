# mark — Project Scope

## Project Summary

`mark` is a cross-platform Rust CLI that renders Markdown files to self-contained HTML5 documents and opens them in the system default browser. Rendered files are stored under `~/.mark/rendered/` and automatically cleaned up after 30 days.

## Business / User Objective

Developers and writers need a zero-friction way to preview Markdown documentation locally without a server, build pipeline, or network access. `mark` provides a single command that produces a polished, navigable HTML view.

## In-Scope

- Markdown → HTML rendering (via `pulldown-cmark`)
- Embedded CSS with light / dark theme support
- Persistent theme config (`~/.mark/config.toml`)
- Persistent reader appearance config (`mark config set-layout`) for font size, letter width, and shell radii
- Per-invocation `--theme` override
- Recursive resolution and rendering of locally linked Markdown files, with HTML link rewriting so browser navigation works
- Self-contained rendered HTML shell assembled in `src/render.rs` with embedded `src/style.css`, preserving sidebar/page chrome without a separate `src/index.html` template
- In-page reader controls surfaced from a dedicated config menu, with theme/layout hotkeys and copyable `mark config` commands for saving appearance preferences
- Live application of in-page reader-layout adjustments to the currently viewed document preview before the settings are persisted
- Reader width controls expressed in `rem` units instead of inches
- Code block copy and "copy clean" (strip comments) toolbar buttons
- Shell completions (bash, zsh, fish, PowerShell)
- Install / uninstall scripts (Linux, macOS, Windows)
- `--cleanup` flag (delete rendered files older than 30 days)
- `--no-open` flag (render without opening browser)
- `cleanup-home` subcommand (destructive removal of `~/.mark`)
- GitHub Actions CI (fmt + clippy + test on Linux, macOS, Windows)

## Out of Scope

- Web server or live-reload mode
- Remote / URL Markdown fetching
- PDF or other non-HTML output formats
- Authentication or multi-user support
- Plugin or extension system
- GUI application

## Key Constraints

- Language: stable Rust only (no nightly features)
- Command name must be `mark`
- App directory must be `~/.mark`
- No network dependencies at render time
- Cross-platform: Linux, macOS, Windows

## Technical Assumptions

- Users have a default browser configured
- Local Markdown links use relative paths (e.g. `./chapter1.md` or `chapter1.md`)
- Circular link graphs are handled safely (visited-set deduplication)
- `pulldown-cmark` is sufficient for all Markdown parsing needs

## Release Expectations

- Patch releases for bug fixes
- Minor releases for new user-facing features
- Follows semver (`vX.Y.Z`), tagged on `main` only

## Success Criteria

- All shipped features pass `cargo test` and `cargo clippy -- -D warnings`
- CI is green on all three platforms
- README accurately reflects all shipped capabilities
- Users can render a multi-file Markdown project and navigate it entirely in the browser
