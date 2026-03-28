#!/usr/bin/env bash
# uninstall.sh — remove the mark CLI from ~/.mark/bin
#
# Usage: ./scripts/uninstall.sh
#
# What this script does:
#   1. Removes the mark binary from ~/.mark/bin
#   2. Removes the PATH entry this installer added from shell config files
#   3. Optionally removes ~/.mark/rendered (asks for confirmation)
#   4. Does NOT remove ~/.mark itself unless rendered is also removed and bin
#      is empty, to avoid destroying user data.
#
# No sudo required.

set -euo pipefail

MARK_DIR="$HOME/.mark"
BIN_DIR="$MARK_DIR/bin"
BINARY="$BIN_DIR/mark"
RENDERED_DIR="$MARK_DIR/rendered"
PATH_MARKER='# >>> mark CLI path >>>'
PATH_MARKER_END='# <<< mark CLI path <<<'

# ── helpers ───────────────────────────────────────────────────────────────────

info()    { echo "[mark] $*"; }
success() { echo "[mark] ✓ $*"; }
warn()    { echo "[mark] ⚠ $*" >&2; }

# Remove the PATH block added by install.sh from a file, idempotently.
remove_path_from_file() {
    local file="$1"
    if [ ! -f "$file" ]; then
        return 0
    fi
    if ! grep -qF "$PATH_MARKER" "$file" 2>/dev/null; then
        return 0
    fi

    # Safety: verify both markers are present before rewriting.
    # If the end marker is absent (user-edited the file), we refuse to
    # rewrite rather than silently truncating the rest of the config.
    if ! grep -qF "$PATH_MARKER_END" "$file" 2>/dev/null; then
        warn "End marker missing in $file — skipping removal to avoid corruption."
        warn "Remove the block manually between '$PATH_MARKER' and '$PATH_MARKER_END'."
        return 1
    fi

    # Use a temp file so we don't corrupt the config on failure.
    local tmp
    tmp="$(mktemp)"
    # Delete every line from PATH_MARKER through PATH_MARKER_END (inclusive).
    # We use a simple state-machine approach that avoids awk regex escaping issues.
    local in_block=0
    while IFS= read -r line || [ -n "$line" ]; do
        if [ "$line" = "$PATH_MARKER" ]; then
            in_block=1
            continue
        fi
        if [ "$in_block" -eq 1 ]; then
            if [ "$line" = "$PATH_MARKER_END" ]; then
                in_block=0
            fi
            continue
        fi
        printf '%s\n' "$line"
    done < "$file" > "$tmp"
    mv "$tmp" "$file"
    success "Removed PATH entry from $file"
}

# ── remove binary ─────────────────────────────────────────────────────────────

if [ -f "$BINARY" ]; then
    rm -f "$BINARY"
    success "Removed binary: $BINARY"
else
    info "Binary not found at $BINARY — nothing to remove."
fi

# ── remove PATH entries ───────────────────────────────────────────────────────

remove_path_from_file "$HOME/.bashrc"
remove_path_from_file "$HOME/.bash_profile"
remove_path_from_file "$HOME/.zshrc"
remove_path_from_file "$HOME/.profile"

FISH_CONFIG="$HOME/.config/fish/config.fish"
if [ -f "$FISH_CONFIG" ]; then
    # Fish uses a different marker block but same approach.
    FISH_MARKER='# >>> mark CLI path >>>'
    FISH_MARKER_END='# <<< mark CLI path <<<'
    if grep -qF "$FISH_MARKER" "$FISH_CONFIG" 2>/dev/null; then
        if ! grep -qF "$FISH_MARKER_END" "$FISH_CONFIG" 2>/dev/null; then
            warn "End marker missing in $FISH_CONFIG — skipping removal to avoid corruption."
            warn "Remove the block manually between '$FISH_MARKER' and '$FISH_MARKER_END'."
        else
            fish_tmp="$(mktemp)"
            fish_in_block=0
            while IFS= read -r line || [ -n "$line" ]; do
                if [ "$line" = "$FISH_MARKER" ]; then
                    fish_in_block=1
                    continue
                fi
                if [ "$fish_in_block" -eq 1 ]; then
                    if [ "$line" = "$FISH_MARKER_END" ]; then
                        fish_in_block=0
                    fi
                    continue
                fi
                printf '%s\n' "$line"
            done < "$FISH_CONFIG" > "$fish_tmp"
            mv "$fish_tmp" "$FISH_CONFIG"
            success "Removed PATH entry from $FISH_CONFIG"
        fi
    fi
fi

# ── optionally remove rendered files ─────────────────────────────────────────

echo ""
if [ -d "$RENDERED_DIR" ]; then
    printf "[mark] Remove rendered HTML files in %s? [y/N] " "$RENDERED_DIR"
    read -r answer
    case "$answer" in
        [yY]|[yY][eE][sS])
            rm -rf "$RENDERED_DIR"
            success "Removed $RENDERED_DIR"
            ;;
        *)
            info "Leaving $RENDERED_DIR intact."
            ;;
    esac
fi

# Remove the bin dir if it's now empty.
if [ -d "$BIN_DIR" ] && [ -z "$(ls -A "$BIN_DIR" 2>/dev/null)" ]; then
    rmdir "$BIN_DIR"
    success "Removed empty directory: $BIN_DIR"
fi

# Remove ~/.mark if it's now empty.
if [ -d "$MARK_DIR" ] && [ -z "$(ls -A "$MARK_DIR" 2>/dev/null)" ]; then
    rmdir "$MARK_DIR"
    success "Removed empty directory: $MARK_DIR"
fi

# ── done ──────────────────────────────────────────────────────────────────────

echo ""
success "mark uninstalled."
echo ""
echo "  Restart your terminal for PATH changes to take effect."
echo ""
