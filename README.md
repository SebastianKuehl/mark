# mark

A cross-platform CLI that renders Markdown files to HTML and opens them in your default browser.

`mark` saves each render invocation under `~/.mark/rendered/<entry>-<timestamp>-<hash>/`, opens the entry page immediately, and automatically cleans up old render runs after 30 days — no configuration needed.

When the Markdown file contains links to other local `.md` files, `mark` can either render just the requested file or recursively materialize the whole linked documentation set, depending on the selected render mode.

---

## Features

- Renders Markdown to a complete, self-contained HTML5 document with embedded CSS
- **Supports explicit render modes** — use `--single` to render only the requested file or `--recursive` to render linked local Markdown files too
- **Linked non-Markdown files** (`.txt`, `.png`, `.pdf`, etc.) are copied into the same render run with their relative paths preserved and their links rewritten — everything opens correctly
- **Breadcrumb navigation** on every page below the entry-point, showing the path from root to the current file
- **Hidden-by-default collapsible sidebar tree** on recursive renders, mirroring the source folder hierarchy with files listed before folders
- **Keyboard sidebar toggle** — press `e` in the rendered page or use the toggle button tooltip
- **Self-contained application shell** — rendered pages now use the embedded Rust-built shell and stylesheet shipped with `mark`, with no dependency on a checked-in `index.html` template
- **Reader config sidebar** — press `c` to toggle the right-side config panel open/closed, or click the ⚙ button; the panel includes theme controls, a hotkey reference, and reader-layout controls; the "Terminal command" accordion is integrated into the pane with an always-visible Copy button in its header; the "Save" button (enabled only when values differ from defaults) copies the command to the clipboard; changes preview live on the current page
- **Sidebar search** — recursive renders include a search field at the top of the hierarchy sidebar so you can quickly filter visible files and folders
- **In-page theme switcher** — press `t` to toggle between `light` and `dark`; the active choice is preserved across hierarchy navigation inside the rendered reader
- **Zen mode** — press `z` to hide reader chrome and switch to a distraction-free reading surface
- **PDF export button** — a download button beside ⚙ opens browser PDF export with a save-path picker where supported and a print fallback elsewhere; press `Primary`+`Shift`+`E` (Command on macOS, Control elsewhere) as a keyboard shortcut; print styling hides reader chrome and forces normal black document text for cleaner PDFs
- **CLI PDF export** — `mark pdf <FILE> <OUTPUT>` renders and exports directly to PDF when a supported headless browser CLI is available (`chromium`, `chromium-browser`, `google-chrome`, or `wkhtmltopdf`)
- **Render cache** — re-running `mark` on an unchanged file prompts before re-rendering; answer N to open the existing result instantly
- Opens the result in the system default browser
- Stores rendered files under `~/.mark/rendered/` in per-run directories — never in your project directory
- Auto-cleans rendered run directories older than 30 days on every run
- `--cleanup` mode for manual housekeeping
- `--no-open` mode for CI or scripting
- Persistent theme support (`system` / `light` / `dark`) via `mark config set-theme`
- Persistent render-mode and sidebar defaults via `mark config`
- Per-invocation theme override via `--theme`
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

### Render a single file only

```sh
mark --single README.md
```

This renders only the requested file, leaves local Markdown links untouched, prints a note when linked Markdown files were skipped, and omits the sidebar.

### Render recursively

```sh
mark --recursive docs/overview.md
```

This renders the entry file plus recursively linked local Markdown files into the same run directory. Only files within the entry file's parent directory (and its subdirectories) are rendered — links to files outside that subtree are silently skipped.

### Run cleanup only

```sh
mark --cleanup
```

Deletes rendered run directories older than 30 days from `~/.mark/rendered/` and prints a summary.

### Export directly to PDF

```sh
mark pdf README.md out/README.pdf
```

`mark pdf` uses a supported headless browser CLI when one is available on `PATH` (`chromium`, `chromium-browser`, `google-chrome`, or `wkhtmltopdf`). If none is available, `mark` leaves the rendered HTML in place, prints its path, and exits with a helpful error so you can finish PDF export manually.

### Help and version

```sh
mark --help
mark --version
```

---

## Theme

`mark` supports **system** (default), **light**, and **dark** render themes.

### Set the theme permanently

```sh
mark config set-theme system
mark config set-theme dark
mark config set-theme light
```

This writes the chosen theme to `~/.mark/config.toml` and is used for all future renders.

### Override the theme for a single run

```sh
mark --theme system README.md
mark --theme dark README.md
mark --theme light README.md
```

The `--theme` flag overrides the persisted config for that invocation only.

### Precedence

1. `--theme` CLI flag (highest priority)
2. Persisted value in `~/.mark/config.toml`
3. Default: `system`

### Config file location

`~/.mark/config.toml` (Linux/macOS) or `%USERPROFILE%\.mark\config.toml` (Windows).

Example contents:

```toml
theme = "system"
```

Rendered pages also include an in-page config menu (top-right ⚙ button, or press `c`) that contains the theme controls. Press `t` to toggle directly between light and dark without opening the menu.

## Render mode and sidebar defaults

Persist default behavior in `~/.mark/config.toml`:

```sh
mark config set-render-mode recursive
mark config set-render-mode single
mark config set-sidebar hidden
mark config set-sidebar visible
mark config set-layout --font-size 17 --letter-width 8.5 --letter-radius 12 --sidebar-button-radius 999 --theme-button-radius 999
```

Precedence:

1. explicit CLI flags such as `--single`, `--recursive`, or `--theme`
2. persisted config values
3. hardcoded defaults (`recursive`, sidebar `hidden`, theme `system`)

Example config:

```toml
theme = "system"
render_mode = "recursive"
sidebar = "hidden"

[appearance]
font_size_px = 17
letter_width_in = 8.5
letter_radius_px = 12
sidebar_button_radius_px = 999
theme_button_radius_px = 999
```

Rendered pages include a reader-layout form inside the config menu (⚙ / `c`). Adjust the values there — changes preview live on the current page — and copy the generated `mark config set-layout ...` command into your terminal to persist the new defaults for future renders. Use the adjacent download button to export the current document as a PDF.



## Where files are stored

| Path | Purpose |
|------|---------|
| `~/.mark/` | Root app directory |
| `~/.mark/bin/` | Installed `mark` binary |
| `~/.mark/rendered/` | Per-invocation render directories |
| `~/.mark/config.toml` | Persistent configuration |

Each invocation gets a run directory named `<entry-stem>-<timestamp>-<hash>/`, for example:

```
README-1711648523-a3f2b1/
```

Within that run directory, rendered Markdown and copied assets preserve their source-relative paths. For example:

```
~/.mark/rendered/README-1711648523-a3f2b1/README.html
~/.mark/rendered/README-1711648523-a3f2b1/chapters/intro.html
~/.mark/rendered/README-1711648523-a3f2b1/assets/logo.png
```

This keeps each render isolated and avoids filename collisions across nested folders or repeated runs.

---

## Cleanup behaviour

On every normal render run, `mark` automatically deletes per-invocation render directories in `~/.mark/rendered/` whose contents are more than 30 days old.

- Only direct children of `~/.mark/rendered/` that belong to old render runs are deleted
- Legacy top-level `.html` files from older versions are also cleaned up when they age out
- Cleanup is best-effort: if one file cannot be deleted, a warning is printed and the run continues
- Run `mark --cleanup` explicitly for a cleanup-only run with a printed summary

---

## Home folder cleanup (`cleanup-home`)

`mark cleanup-home` removes the entire `~/.mark` directory (or `%USERPROFILE%\.mark` on Windows) from your home folder.  This is a **destructive, irreversible operation** that deletes:

- All rendered HTML files
- Your `config.toml` (theme preference)
- The installed `mark` binary inside `.mark/bin`

### How it differs from `--cleanup`

| Command | What it removes |
|---------|-----------------|
| `mark --cleanup` | Rendered run directories older than 30 days only |
| `mark cleanup-home` | The entire `.mark` app directory |

### Usage

```sh
# Interactive — prompts for confirmation
mark cleanup-home

# Non-interactive — skips the prompt (useful in scripts)
mark cleanup-home --yes
```

By default you must type `yes` at the confirmation prompt.  If the directory does not exist, the command exits successfully with a no-op message.

### Windows note

On Windows the running `mark.exe` binary lives inside `.mark/bin/`.  Because Windows locks executables that are in use, the binary itself may not be removable while `mark` is running.  In that case `mark cleanup-home` performs a best-effort deletion, skips the locked file, prints a warning, and asks you to re-run after the process exits.

---


`mark` can generate completion scripts for bash, zsh, fish, and PowerShell.

### Automatic installation

The install scripts set up completions automatically:

| Shell | Location | Config hook |
|-------|----------|-------------|
| bash | `~/.bash_completion.d/mark` | sourced from `~/.bashrc` / `~/.bash_profile` |
| zsh (oh-my-zsh) | `$ZSH_CUSTOM/completions/_mark` | auto-loaded by oh-my-zsh before `compinit` |
| zsh (plain) | `~/.zsh/completions/_mark` | `fpath` + `compinit` added to `~/.zshrc` |
| fish | `~/.config/fish/completions/mark.fish` | auto-loaded by fish |
| PowerShell | `%USERPROFILE%\.mark\completions\mark.ps1` | dot-sourced from `$PROFILE` |

All hooks are idempotent — running the installer a second time is safe.

> **oh-my-zsh users**: the installer detects oh-my-zsh and drops `_mark` into
> `$ZSH_CUSTOM/completions/` (default: `~/.oh-my-zsh/custom/completions/`).
> oh-my-zsh adds that directory to `fpath` before calling `compinit`, so no
> `~/.zshrc` edits are needed. Run `omz reload` or open a new terminal to
> activate completions.

### Manual installation

Generate a script and install it yourself:

**bash**
```sh
mark completions bash > ~/.bash_completion.d/mark
# Add to ~/.bashrc (once):
echo 'source ~/.bash_completion.d/mark' >> ~/.bashrc
source ~/.bashrc
```

**zsh (oh-my-zsh)**
```sh
mkdir -p "${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/completions"
mark completions zsh > "${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/completions/_mark"
# Reload (oh-my-zsh adds $ZSH_CUSTOM/completions to fpath automatically):
omz reload
```

**zsh (plain, no framework)**
```sh
mkdir -p ~/.zsh/completions
mark completions zsh > ~/.zsh/completions/_mark
# Add to ~/.zshrc (once, before any compinit call):
echo 'fpath=(~/.zsh/completions $fpath)' >> ~/.zshrc
echo 'autoload -Uz compinit && compinit' >> ~/.zshrc
source ~/.zshrc
```

**fish**
```sh
mark completions fish > ~/.config/fish/completions/mark.fish
# Fish auto-loads completions from this directory — no further steps needed.
```

**PowerShell**
```powershell
mark completions powershell > "$env:USERPROFILE\.mark\completions\mark.ps1"
# Add to your $PROFILE (once):
Add-Content $PROFILE ". '$env:USERPROFILE\.mark\completions\mark.ps1'"
```

### Preview a completion script

```sh
mark completions bash
mark completions zsh
mark completions fish
mark completions powershell
```

---


The installer adds a guarded PATH entry to your shell config using a `case` statement that prevents duplication even when multiple config files source each other:

```sh
# >>> mark CLI path >>>
case ":$PATH:" in *":$HOME/.mark/bin:"*) ;; *) export PATH="$HOME/.mark/bin:$PATH" ;; esac
# <<< mark CLI path <<<
```

The uninstaller removes this exact block cleanly.

---

## Code block copy actions

Every fenced code block in the rendered HTML includes action buttons in a toolbar above the block:

- **📋 Copy** — copies the code exactly as shown
- **🧹 Copy clean** — copies the code with full-line comments removed (supported languages only)

### Supported languages for Copy clean

`bash`, `sh`, `zsh`, `fish`, `powershell`, `python`, `rust`, `javascript`, `typescript`

### Limitations

- `Copy clean` removes only **full-line comments** (lines where the trimmed content starts with the comment marker)
- Inline comments (code followed by a comment on the same line) are **not** removed
- Block comments (`/* ... */`) are **not** removed
- Languages not in the supported list show only the `Copy` button

---

## Linked Markdown navigation

When a Markdown file contains links to other local `.md` files, `mark` can either render only the requested file or recursively render all of them and rewrite every inter-document link to point to the corresponding rendered HTML file.

```sh
mark docs/overview.md
```

If `overview.md` links to `chapter1.md`, `chapter2.md`, and `chapter1.md` links to `appendix.md`, all four files are rendered in one invocation. Each rendered HTML file's links point to the other rendered files, so you can navigate the entire documentation set in the browser without any extra commands.

- **Recursive** — follows links transitively to any depth
- **Scoped** — only follows links within the entry file's directory subtree; links outside that boundary are skipped and their hrefs left unchanged
- **Circular-safe** — already-visited files are never rendered twice
- **Non-intrusive** — external URLs, image links, and non-Markdown file links are left unchanged
- **Fragment-aware** — `./api.md#section` rewrites to the rendered HTML path with `#section` intact
- **Explicit control** — `--recursive` forces linked rendering; `--single` keeps local Markdown links unrendered and notes the skipped links

For each linked file rendered beyond the entry point, `mark` prints:

```
  → rendered: chapter1.md → ~/.mark/rendered/overview-<ts>-<hash>/chapter1.html
```

---

## Navigation chrome

Every page in a multi-file render set gets two navigation elements injected automatically — no flags needed.

### Breadcrumbs

Pages below the entry-point show a breadcrumb trail at the top:

```
overview › chapter1 › appendix
```

Each ancestor is a clickable link. The current page appears as plain text. The entry-point itself shows no breadcrumb.

### Sidebar

Recursive renders show a collapsible sidebar tree listing all files rendered in the current invocation. The sidebar is hidden by default, can be toggled with the ☰ button or the `e` hotkey, orders files before folders at every level, mirrors the source folder structure, highlights the current page, and links to all other rendered pages.

---

## Render cache

`mark` remembers the last render for each source file in `~/.mark/render-cache.toml`. For single-file renders, if the file and selected render settings haven't changed since the last render:

```
Already rendered: ~/.mark/rendered/overview-1711648523-a3f2b1/overview.html
Re-render? [y/N]:
```

- **N / Enter** — opens the existing rendered file immediately, skips rendering.
- **Y** — re-renders and updates the cache.

If the source file has changed (different mtime), `mark` re-renders silently without prompting.

`mark --cleanup` prunes cache entries whose render run directory no longer exists on disk.
`mark --no-open` always re-renders without prompting (non-interactive mode).

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
