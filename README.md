# mark

A cross-platform CLI that renders Markdown files to HTML and opens them in your default browser.

`mark` saves rendered HTML to `~/.mark/rendered/`, opens it immediately, and automatically cleans up files older than 30 days — no configuration needed.

---

## Features

- Renders Markdown to a complete, self-contained HTML5 document with embedded CSS
- Opens the result in the system default browser
- Stores rendered files under `~/.mark/rendered/` — never in your project directory
- Auto-cleans rendered files older than 30 days on every run
- `--cleanup` mode for manual housekeeping
- `--no-open` mode for CI or scripting
- Works on Linux, macOS, and Windows
- No network dependencies — all styling is embedded

---

## Supported platforms

| Platform | Status |
|----------|--------|
| macOS    | ✅     |
| Linux    | ✅     |
| Windows  | ✅     |

---

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain, 1.70+)
- A default browser configured on your system

---

## Installation

### Linux / macOS

From the repository root:

```sh
bash scripts/install.sh
```

The installer:
1. Builds `mark` in release mode with `cargo build --release`
2. Copies the binary to `~/.mark/bin/mark`
3. Adds `~/.mark/bin` to your PATH in your shell config (`.bashrc`, `.zshrc`, fish config, or `.profile` as fallback)
4. PATH setup is idempotent — running the installer again is safe

After installation, restart your shell or run the printed `source` command:

```sh
source ~/.bashrc   # or ~/.zshrc, or restart your terminal
mark --version
```

### Windows

Open PowerShell (no admin required) from the repository root:

```powershell
.\scripts\install.ps1
```

The installer:
1. Builds `mark.exe` in release mode
2. Copies it to `%USERPROFILE%\.mark\bin\mark.exe`
3. Adds `%USERPROFILE%\.mark\bin` to your user PATH (idempotent)
4. Marks `.mark` as a hidden folder

Restart PowerShell after installation for the PATH change to take effect.

---

## Usage

### Render a Markdown file and open in browser

```sh
mark README.md
```

### Render without opening the browser

```sh
mark --no-open README.md
```

Useful in CI, scripts, or headless environments.

### Run cleanup only

```sh
mark --cleanup
```

Deletes rendered HTML files older than 30 days from `~/.mark/rendered/` and prints a summary.

### Help and version

```sh
mark --help
mark --version
```

---

## Where files are stored

| Path | Purpose |
|------|---------|
| `~/.mark/` | Root app directory |
| `~/.mark/bin/` | Installed `mark` binary |
| `~/.mark/rendered/` | Generated HTML files |

Rendered filenames follow the pattern `<source-stem>-<timestamp>-<hash>.html`, e.g.:

```
README-1711648523-a3f2b1.html
```

This ensures no collisions even when rendering the same file multiple times.

---

## Cleanup behaviour

On every normal render run, `mark` automatically deletes HTML files in `~/.mark/rendered/` whose **modified time** is more than 30 days in the past.

- Only `.html` files inside `~/.mark/rendered/` are deleted — never anything else
- Cleanup is best-effort: if one file cannot be deleted, a warning is printed and the run continues
- Run `mark --cleanup` explicitly for a cleanup-only run with a printed summary

---

## PATH setup

The installer adds a guarded PATH entry to your shell config using a `case` statement that prevents duplication even when multiple config files source each other:

```sh
# >>> mark CLI path >>>
case ":$PATH:" in *":$HOME/.mark/bin:"*) ;; *) export PATH="$HOME/.mark/bin:$PATH" ;; esac
# <<< mark CLI path <<<
```

The uninstaller removes this exact block cleanly.

---

## Security note

`mark` passes raw HTML blocks in Markdown through unchanged (via `pulldown-cmark`). This is intentional for local user files. **Do not use `mark` to render untrusted Markdown from external sources** — embedded `<script>` or other HTML could execute in your browser.

---

## Uninstall

### Linux / macOS

```sh
bash scripts/uninstall.sh
```

This removes `~/.mark/bin/mark` and the PATH block from your shell configs. It will prompt before deleting `~/.mark/rendered/`.

### Windows

```powershell
.\scripts\uninstall.ps1
```

---

## Troubleshooting

**`mark: command not found` after install**
Restart your terminal or run `source ~/.bashrc` (or the equivalent for your shell). The installer prints the exact command.

**Browser doesn't open**
`mark` uses the [`open`](https://crates.io/crates/open) crate, which calls `xdg-open` on Linux, `open` on macOS, and `start` on Windows. Ensure a default browser is configured. If opening fails, `mark` prints a warning and exits successfully — the HTML file is still written.

**`cargo` not found during install**
Install Rust via [rustup.rs](https://rustup.rs/): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

**Rendered files accumulating**
Run `mark --cleanup` to remove files older than 30 days.

---

## Development

```sh
# Build (debug)
cargo build

# Run directly
cargo run -- path/to/file.md

# Run tests
cargo test

# Format
cargo fmt

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Build release
cargo build --release
```

---

## CI

The repository includes a GitHub Actions workflow (`.github/workflows/ci.yml`) that runs on every push to `main` and on all pull requests:

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`

Runs on Linux, macOS, and Windows (stable Rust).

---

## License

MIT — see [LICENSE](LICENSE).
