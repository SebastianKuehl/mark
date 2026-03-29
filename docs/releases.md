# Releases

## v0.2.1 — patch

- **Date:** 2026-03-29
- **Type:** patch
- **Merged items:** B-001 — Non-Markdown linked files not copied
- **Summary:** Local non-Markdown files linked from Markdown (e.g. `.txt`, `.png`, `.pdf`) are now copied to `~/.mark/rendered/` and their links are rewritten to absolute paths, making them accessible in the browser. Includes path-traversal guard and idempotent copy. 8 new tests.

---

## v0.2.0 — minor

- **Date:** 2026-03-29
- **Type:** minor
- **Merged items:** F-010, M-010 — Recursive linked Markdown rendering
- **Summary:** `mark` now discovers all local `.md` links in the entry-point file recursively, renders every transitively reachable file, and rewrites inter-document links to their rendered HTML paths. Browser navigation across a multi-file Markdown project works out of the box. Circular references are handled safely via a canonical-path visited set. 34 new tests added.

---

## v0.1.2 — patch

- **Date:** 2026-03-29 (retroactive record)
- **Type:** patch
- **Merged items:** M9 — Home folder cleanup command
- **Summary:** Added `mark cleanup-home` subcommand for destructive removal of the entire `~/.mark` directory. Includes `--yes` flag for non-interactive use. Windows best-effort handling for locked binary.

---

## v0.1.1 — patch

- **Date:** retroactive record
- **Type:** patch
- **Merged items:** version flag fix
- **Summary:** Fixed `--version` output to display `v<version>` format.

---

## v0.1.0 — minor

- **Date:** retroactive record
- **Type:** minor
- **Merged items:** M1–M8
- **Summary:** Initial feature-complete release. Markdown rendering, browser open, cleanup, install/uninstall scripts, CI, shell completions, code block copy actions, persistent theme configuration.
