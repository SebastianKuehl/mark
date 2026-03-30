# Releases

## v0.13.1 — patch

- **Date:** 2026-03-30
- **Type:** patch
- **Merged items:** rollback of sidebar chrome follow-up
- **Summary:** Released the current `main` state after reverting the unshipped `fix: polish sidebar control chrome` follow-up while keeping the earlier right-shell button alignment fix. This tag captures the rollback commit as the latest shipped state. The crate version is synced to `v0.13.1` in `Cargo.toml` and `Cargo.lock`.

---

## v0.13.0 — minor

- **Date:** 2026-03-30
- **Type:** minor
- **Merged items:** M-025, F-028, F-029, B-014, B-015
- **Summary:** Added shorthand CLI flags (`-n`, `-t`, and lowercase `-v`), taught the root positional argument to accept a directory so `mark docs` behaves like running `mark` inside that folder, and improved `mark pdf` ergonomics so `mark pdf <FILE> .` resolves to `<stem>.pdf` in the current directory. This release also finishes the reader-shell regression fixes by making zen mode visually collapse the page into the letter surface and ensuring the config pane layers above the shell buttons. The crate version is synced to `v0.13.0` in `Cargo.toml` and `Cargo.lock`.

---

## v0.12.0 — minor

- **Date:** 2026-03-30
- **Type:** minor
- **Merged items:** M-024, F-027
- **Summary:** Replaced the split cleanup surface with a unified `mark wipe` command. The legacy `cleanup-home` subcommand was renamed to `wipe`, the root `--cleanup` flag was removed, and cleanup is now expressed through explicit wipe modes for deleting the whole `.mark` folder, only config, only renders, or only renders older than 30 days. The crate version is synced to `v0.12.0` in `Cargo.toml` and `Cargo.lock`.

---

## v0.11.0 — minor

- **Date:** 2026-03-30
- **Type:** minor
- **Merged items:** M-023, F-026, B-013
- **Summary:** `mark` can now be invoked without an explicit file argument: it discovers Markdown files in the current directory, renders them together, and opens the first discovered file as the entry page, while producing a clear error if no Markdown files are present. This release also fixes zen mode so its background stays synchronized with the effective letter surface. The crate version is synced to `v0.11.0` in `Cargo.toml` and `Cargo.lock`.

---

## v0.10.0 — minor

- **Date:** 2026-03-30
- **Type:** minor
- **Merged items:** M-021, F-023, F-025, B-010, B-011, M-022, F-024, B-012
- **Summary:** Shipped two user-facing milestones together. The reader shell now supports left-sidebar search, a `z` zen mode, persistent in-browser theme choice across hierarchy navigation, and a smoother right config sidebar with safer default normalization for blank layout inputs. The CLI now exposes `mark pdf <FILE> <OUTPUT>`, uses clean either-or root usage (`mark [OPTIONS] [FILE]` or `mark [OPTIONS] <COMMAND>`), and keeps shell completion aligned with that command shape. The crate version is synced to `v0.10.0` in `Cargo.toml` and `Cargo.lock`.

---

## v0.9.3 — patch

- **Date:** 2026-03-30
- **Type:** patch
- **Merged items:** config pane sidebar polish
- **Summary:** The config UI now opens as a proper fixed right sidebar so its content remains usable instead of overflowing past the viewport. The `t` hotkey now changes theme without closing the config panel, preserving the open state while you preview theme changes. The crate version was synced to `v0.9.3` in both `Cargo.toml` and `Cargo.lock`.

---

## v0.9.2 — patch

- **Date:** 2026-03-30
- **Type:** patch
- **Merged items:** reader export + hotkey polish
- **Summary:** PDF export now prints with normal black document text for cleaner output even when the page is rendered in dark mode. Added an OS-agnostic `Primary`+`Shift`+`E` export shortcut, exposed the hotkeys inside the config menu, and removed the sidebar footer. Synced the crate version in `Cargo.toml` and `Cargo.lock` to the shipped tag.

---

## v0.9.1 — patch

- **Date:** 2026-03-30
- **Type:** patch
- **Merged items:** B-002
- **Summary:** Render cache now tracks the full rendered Markdown file tree, not just the entry file. Cache reuse is allowed only when every rendered Markdown file still matches its stored mtime, so linked-file edits invalidate the cache automatically. Recursive renders can now safely reuse cached output when the full source tree and render settings are unchanged.

---

## v0.9.0 — minor

- **Date:** 2026-03-30
- **Type:** minor
- **Merged items:** M-020, F-022, B-008, B-009
- **Summary:** Reader shell export and polish. Added a PDF export button beside the config control, with browser save-path picking where supported and clean print styling that hides reader chrome. Fixed the remaining letter top-alignment issue so the document starts level with the header controls. Flattened the terminal-command accordion into the config pane and moved the Copy button into the accordion header so it stays visible while collapsed.

---

## v0.8.0 — minor

- **Date:** 2026-03-30
- **Type:** minor
- **Merged items:** M-019, F-021
- **Summary:** Recursive render scope restriction. BFS link-following is now scoped to the entry file's parent directory and its subdirectories. Links to `.md` files or assets outside that subtree are silently skipped and their HTML hrefs are left unchanged. Prevents unintended rendering of sibling or parent directory content.

---

## v0.7.1 — patch

- **Date:** 2026-03-30
- **Type:** patch
- **Merged items:** M-018, B-007, F-020
- **Summary:** Reader shell UX refinements. `c` hotkey now toggles the config pane open/closed (was open-only). Document letter top edge aligned with the header chrome. "Terminal command" block in the config pane is now a collapsed `<details>` accordion by default. A full-width "Save" button appears below the accordion — disabled until any layout value differs from the rendered defaults; on click it copies the generated command to the clipboard.

---

## v0.7.0 — minor

- **Date:** 2026-03-30
- **Type:** minor
- **Merged items:** M-017, F-019, B-006
- **Summary:** Reader controls polish. The top-right reader button is now a settings/gear icon (⚙) that opens a combined config menu for both theme and reader-layout controls. `t` hotkey now toggles only between light and dark. New `c` hotkey opens the config menu. Reader-layout changes apply live to the current document via CSS custom properties. Letter-width label updated to rem. Page background radial glow reduced from 0.55 → 0.08 opacity. Stray unlabeled checkbox removed from page chrome.

---

## v0.6.1 — minor

- **Date:** 2026-03-30
- **Type:** minor
- **Merged items:** M-016, F-018, B-005
- **Summary:** Reader customization controls. Added `mark config set-layout` command for persisting font size, letter width, letter corner radius, and button radii. Rendered pages expose an in-page reader-layout form with a generated `mark config set-layout ...` command. Cache reuse is now settings-aware. Sidebar footer hotkeys are pinned and remain visible regardless of tree height. Removed residual `src/index.html` template dependency — shell is now fully self-contained in `src/render.rs`.

---

## v0.6.0 — minor

- **Date:** 2026-03-29
- **Type:** minor
- **Merged items:** M-015, F-017 — Template-driven render shell
- **Summary:** Rendered pages moved onto the bundled `src/index.html` shell at the time of this release, replacing the earlier hand-built wrapper while preserving sidebar behavior, theme switching, single-vs-recursive rendering, and icon + label theme options. That runtime dependency has since been removed in favor of today's self-contained shell in `src/render.rs`.

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
