#!/usr/bin/env bash
# install.sh — install the mark CLI into ~/.mark/bin
#
# Usage: ./scripts/install.sh
#
# Requirements:
#   - Rust/Cargo must be installed (https://rustup.rs/)
#   - No sudo required — everything is user-scoped
#
# What this script does:
#   1. Builds mark in release mode
#   2. Creates ~/.mark/bin and ~/.mark/rendered if they don't exist
#   3. Copies the binary to ~/.mark/bin/mark
#   4. Adds ~/.mark/bin to PATH in your shell config (idempotent)

set -euo pipefail

MARK_DIR="$HOME/.mark"
BIN_DIR="$MARK_DIR/bin"
RENDERED_DIR="$MARK_DIR/rendered"
BINARY="$BIN_DIR/mark"
PATH_SNIPPET='case ":$PATH:" in *":$HOME/.mark/bin:"*) ;; *) export PATH="$HOME/.mark/bin:$PATH" ;; esac'
PATH_MARKER='# >>> mark CLI path >>>'
PATH_MARKER_END='# <<< mark CLI path <<<'

# ── helpers ──────────────────────────────────────────────────────────────────

info()    { echo "[mark] $*"; }
success() { echo "[mark] ✓ $*"; }
warn()    { echo "[mark] ⚠ $*" >&2; }
die()     { echo "[mark] ✗ $*" >&2; exit 1; }

# Add PATH block to a shell config file, idempotently.
add_path_to_file() {
    local file="$1"
    # Create the file if it doesn't exist yet.
    touch "$file"
    # Skip if the marker is already present.
    if grep -qF "$PATH_MARKER" "$file" 2>/dev/null; then
        info "PATH already configured in $file — skipping."
        return 0
    fi
    {
        echo ""
        echo "$PATH_MARKER"
        echo "$PATH_SNIPPET"
        echo "$PATH_MARKER_END"
    } >> "$file"
    success "Added ~/.mark/bin to PATH in $file"
    RESTART_NEEDED=1
}

# ── preflight ─────────────────────────────────────────────────────────────────

# Verify Cargo is available.
if ! command -v cargo &>/dev/null; then
    die "Cargo not found. Install Rust from https://rustup.rs/ and try again.
       After installing, restart your terminal and re-run this script."
fi

# Must be run from the project root (where Cargo.toml lives).
if [ ! -f "Cargo.toml" ]; then
    die "Cargo.toml not found. Run this script from the mark project root."
fi

# ── build ─────────────────────────────────────────────────────────────────────

info "Building mark (release)…"
cargo build --release
success "Build complete."

# ── install dirs ──────────────────────────────────────────────────────────────

mkdir -p "$BIN_DIR" "$RENDERED_DIR"
success "Directories ready: $MARK_DIR"

# ── copy binary ───────────────────────────────────────────────────────────────

cp target/release/mark "$BINARY"
chmod +x "$BINARY"
success "Binary installed: $BINARY"

# ── PATH setup ────────────────────────────────────────────────────────────────

RESTART_NEEDED=0

# Detect which shell config files to update.
# We attempt all applicable files; each update is idempotent.

# bash
if [ -f "$HOME/.bashrc" ]; then
    add_path_to_file "$HOME/.bashrc"
fi
# macOS bash login shell
if [ -f "$HOME/.bash_profile" ]; then
    add_path_to_file "$HOME/.bash_profile"
fi
# zsh
if [ -f "$HOME/.zshrc" ]; then
    add_path_to_file "$HOME/.zshrc"
fi

# fish
FISH_CONFIG="$HOME/.config/fish/config.fish"
if command -v fish &>/dev/null || [ -f "$FISH_CONFIG" ]; then
    mkdir -p "$(dirname "$FISH_CONFIG")"
    touch "$FISH_CONFIG"
    FISH_MARKER='# >>> mark CLI path >>>'
    FISH_LINE='fish_add_path "$HOME/.mark/bin"'
    if ! grep -qF "$FISH_MARKER" "$FISH_CONFIG" 2>/dev/null; then
        {
            echo ""
            echo "$FISH_MARKER"
            echo "$FISH_LINE"
            echo "# <<< mark CLI path <<<"
        } >> "$FISH_CONFIG"
        success "Added ~/.mark/bin to PATH in $FISH_CONFIG"
        RESTART_NEEDED=1
    else
        info "PATH already configured in $FISH_CONFIG — skipping."
    fi
fi

# .profile fallback: use it if none of the above files existed.
if [ ! -f "$HOME/.bashrc" ] && [ ! -f "$HOME/.bash_profile" ] && \
   [ ! -f "$HOME/.zshrc" ] && ! command -v fish &>/dev/null; then
    add_path_to_file "$HOME/.profile"
fi

# ── done ──────────────────────────────────────────────────────────────────────

echo ""
success "mark installed successfully!"
echo ""
echo "  Binary : $BINARY"
echo "  Renders: $RENDERED_DIR"
echo ""

if [ "$RESTART_NEEDED" -eq 1 ]; then
    echo "  ⚠  Restart your terminal (or source your shell config) to use mark:"
    echo ""
    if [ -f "$HOME/.zshrc" ]; then
        echo "    source ~/.zshrc"
    fi
    if [ -f "$HOME/.bashrc" ]; then
        echo "    source ~/.bashrc"
    fi
    if [ -f "$HOME/.bash_profile" ]; then
        echo "    source ~/.bash_profile"
    fi
else
    echo "  mark is ready to use — open a new terminal and run: mark --help"
fi
echo ""
