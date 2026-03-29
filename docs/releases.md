# Releases

## v0.6.0 — minor

- **Date:** 2026-03-29
- **Type:** minor
- **Merged items:** M-015, F-017 — Template-driven render shell
- **Summary:** Rendered pages now use the bundled `src/index.html` shell as the source of truth for page layout, replacing the old hand-built wrapper while preserving sidebar behavior, theme switching, single-vs-recursive rendering, and icon + label theme options.

---

## v0.5.1 — patch

- **Date:** 2026-03-29
- **Type:** patch
- **Merged items:** B-004 — Completion suggests subcommands after FILE
- **Summary:** Bash completion generation now stops suggesting root subcommands like `config` and `cleanup-home` after the positional `FILE` argument is already present, while preserving normal root-level subcommand completion before a file is supplied. Includes regression coverage for the generated Bash completion behavior.

---

## v0.5.0 — minor

- **Date:** 2026-03-29
- **Type:** minor
- **Merged items:** M-014 (F-015, F-016) — View controls and render modes
- **Summary:** Added explicit `--single/-s` and `--recursive/-r` render modes, persistent defaults for render mode and sidebar visibility, hidden-by-default recursive sidebar with `e` hotkey support, files-first recursive sidebar ordering, and an in-page `system` / `light` / `dark` theme switcher with icon + label controls.

---

## v0.4.0 — minor

- **Date:** 2026-03-29
- **Type:** minor
- **Merged items:** F-014, M-013 — Folder hierarchy preservation
- **Summary:** Each render now writes into its own `~/.mark/rendered/<entry>-<ts>-<hash>/` run directory. Rendered Markdown and copied assets preserve their source-relative folder hierarchy, the sidebar renders a collapsible folder tree, cleanup removes old run directories, and the render cache tracks run directories instead of individual HTML files.

---

## v0.3.0 — minor

- **Date:** 2026-03-29
- **Type:** minor
- **Merged items:** M-011 (F-011, F-012), F-013/M-012, B-001
- **Summary:** Full navigation chrome: breadcrumbs on linked pages, collapsible CSS sidebar on all pages. Render cache (`~/.mark/render-cache.toml`) prompts before re-rendering unchanged files. Non-Markdown linked files (assets) are copied to `~/.mark/rendered/` and their links are rewritten. 94 tests total.

---

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
