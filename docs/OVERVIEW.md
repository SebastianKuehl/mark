# mark — Owner Overview

> Quick-navigation hub for all project documentation.
> Open this file with `mark docs/OVERVIEW.md` to browse everything in the browser.

---

## 🗂 Project

| Document                                       | Purpose                                                         |
| ---------------------------------------------- | --------------------------------------------------------------- |
| [scope.md](scope.md)                           | Project summary, in/out of scope, constraints, success criteria |
| [project-spec.md](project-spec.md)             | Original build specification                                    |
| [worker-agent-rules.md](worker-agent-rules.md) | Rules governing all anvil implementation agents                 |
| [../README.md](../README.md)                   | Public-facing user documentation                                |

### ID Conventions

- `M-###` = milestone
- `F-###` = feature
- `B-###` = bug
- `P-###` = delegated prompt

Only the instructor and the Product Owner Agent may update `README.md`.

Current active execution item: `—`.
Current active milestone: `—` (all documented work released through `v0.13.0`).

---

## 📦 Releases

| Version | Type | Highlights |
|---------|------|------------|
| [v0.13.0](releases.md#v0130--minor) | minor | Short CLI flags, directory entry rendering, improved PDF path ergonomics, and final zen/config shell regression fixes |
| [v0.12.0](releases.md#v0120--minor) | minor | Unified `mark wipe` cleanup modes replace `cleanup-home` and the old `--cleanup` flag |
| [v0.11.0](releases.md#v0110--minor) | minor | `mark` now works without an explicit file, and zen mode backgrounds stay synced to the letter surface |
| [v0.10.0](releases.md#v0100--minor) | minor | Sidebar search, zen mode, theme persistence across navigation, `mark pdf`, and clean root CLI usage |
| [v0.9.3](releases.md#v093--patch) | patch | Config pane is now a right sidebar and `t` no longer closes it while toggling theme |
| [v0.9.2](releases.md#v092--patch) | patch | Black-text PDF export, `Primary`+`Shift`+`E`, config hotkey help, sidebar footer removed, crate version synced |
| [v0.9.1](releases.md#v091--patch) | patch | Render cache now invalidates on linked Markdown file changes and safely reuses unchanged recursive renders |
| [v0.9.0](releases.md#v090--minor) | minor | PDF export button, final letter top alignment fix, flat terminal-command accordion with visible Copy button |
| [v0.8.0](releases.md#v080--minor) | minor | Recursive render scope restriction — BFS follows only links within entry file's directory subtree |
| [v0.7.1](releases.md#v071--patch) | patch | Reader shell UX — c hotkey toggles, letter alignment, accordion terminal command, save button |
| [v0.7.0](releases.md#v070--minor) | minor | Reader controls polish — config menu (⚙/c), t light/dark toggle, live layout preview, rem units, subtle bg glow, stray checkbox removed |
| [v0.6.1](releases.md#v061--minor) | minor | Reader customization controls — `mark config set-layout`, in-page layout form, settings-aware cache, sidebar footer pinned, template dep removed |
| [v0.6.0](releases.md#v060--minor) | minor | Render shell milestone landed (initially via `src/index.html`, now superseded by the self-contained shell in `src/render.rs`) |
| [v0.5.1](releases.md#v051--patch) | patch | Bash completions stop suggesting root subcommands after the positional `FILE` argument |
| [v0.5.0](releases.md#v050--minor) | minor | Single vs recursive render modes, hidden-by-default sidebar controls, in-page theme switcher |
| [v0.2.0](releases.md#v020--minor) | minor | Recursive linked Markdown rendering + HTML link rewriting |
| [v0.1.2](releases.md#v012--patch) | patch | `cleanup-home` subcommand |
| [v0.1.1](releases.md#v011--patch) | patch | `--version` format fix |
| [v0.1.0](releases.md#v010--minor) | minor | Initial release (M1–M8) |

→ Full release log: [releases.md](releases.md)

---

## 🏁 Milestones

| ID | Title | Status | Doc |
|----|-------|--------|-----|
| M-010 | Linked Markdown Navigation | ✅ released | [M-010](milestones/M-010-linked-markdown-navigation.md) |
| M-011 | Navigation Chrome (breadcrumbs + sidebar) | ✅ released | [M-011](milestones/M-011-navigation-chrome.md) |
| M-012 | Render Memory | ✅ released | [M-012](milestones/M-012-render-memory.md) |
| M-013 | Folder Hierarchy Preservation | ✅ released | [M-013](milestones/M-013-folder-hierarchy.md) |
| M-014 | View Controls and Render Modes | ✅ released | [M-014](milestones/M-014-view-controls-and-render-modes.md) |
| M-015 | Historical Template-Shell Migration | ✅ released | [M-015](milestones/M-015-template-render-shell.md) |
| M-016 | Reader Customization Controls | ✅ released | [M-016](milestones/M-016-reader-customization-controls.md) |
| M-017 | Reader Controls Polish | ✅ released | [M-017](milestones/M-017-reader-controls-polish.md) |
| M-018 | Reader Shell UX Refinements | ✅ released | [M-018](milestones/M-018-reader-shell-ux-refinements.md) |
| M-020 | PDF Export and Letter Alignment Polish | ✅ released | [M-020](milestones/M-020-pdf-export-letter-alignment.md) |
| M-021 | Sidebar Search and Config Panel Polish | ✅ released | [M-021](milestones/M-021-sidebar-and-config-polish.md) |
| M-022 | CLI PDF Export and Command Shape Cleanup | ✅ released | [M-022](milestones/M-022-cli-pdf-and-command-shape.md) |
| M-023 | Default Current-Directory Markdown Entry | ✅ released | [M-023](milestones/M-023-default-cwd-render-entrypoint.md) |
| M-024 | Wipe Command and Cleanup Surface Redesign | ✅ released | [M-024](milestones/M-024-wipe-command-and-cleanup-redesign.md) |
| M-025 | CLI Ergonomics and Reader-Shell Regressions | ✅ released | [M-025](milestones/M-025-cli-ergonomics-and-shell-regressions.md) |
| M-019 | Recursive Render Scope Restriction | ✅ released | [M-019](milestones/M-019-recursive-render-scope-restriction.md) |
| M1–M9 | Original build milestones (scaffold → cleanup-home) | ✅ released | [milestones.md](milestones.md) |

---

## ✨ Features

| ID | Title | Status | Doc |
|----|-------|--------|-----|
| [F-010](features/F-010-linked-markdown-rendering.md) | Recursive linked Markdown rendering | ✅ released | [F-010](features/F-010-linked-markdown-rendering.md) |
| [F-011](features/F-011-breadcrumbs.md) | Breadcrumb navigation | ✅ released | [F-011](features/F-011-breadcrumbs.md) |
| [F-012](features/F-012-sidebar.md) | Sidebar hierarchy | ✅ released | [F-012](features/F-012-sidebar.md) |
| [F-013](features/F-013-render-memory.md) | Render memory + re-render confirmation | ✅ released | [F-013](features/F-013-render-memory.md) |
| [F-014](features/F-014-folder-hierarchy.md) | Preserve folder hierarchy in rendered output and sidebar tree | ✅ released | [F-014](features/F-014-folder-hierarchy.md) |
| [F-015](features/F-015-render-mode-defaults.md) | Render mode flags and persistent defaults | ✅ released | [F-015](features/F-015-render-mode-defaults.md) |
| [F-016](features/F-016-sidebar-theme-controls.md) | Sidebar and theme controls | ✅ released | [F-016](features/F-016-sidebar-theme-controls.md) |
| [F-017](features/F-017-template-shell.md) | Historical `src/index.html` template-shell migration | ✅ released | [F-017](features/F-017-template-shell.md) |
| [F-018](features/F-018-reader-appearance-controls.md) | Persistent reader appearance controls | ✅ released | [F-018](features/F-018-reader-appearance-controls.md) |
| [F-019](features/F-019-config-menu-live-preview.md) | Config menu hotkeys and live reader-layout preview polish | ✅ released | [F-019](features/F-019-config-menu-live-preview.md) |
| [F-020](features/F-020-reader-shell-ux-refinements.md) | Reader shell UX refinements (letter alignment, accordion, save button) | ✅ released | [F-020](features/F-020-reader-shell-ux-refinements.md) |
| [F-021](features/F-021-restrict-recursive-render-scope.md) | Restrict recursive rendering to entry file's directory subtree | ✅ released | [F-021](features/F-021-restrict-recursive-render-scope.md) |
| [F-022](features/F-022-pdf-export-button.md) | PDF export button with file picker | ✅ released | [F-022](features/F-022-pdf-export-button.md) |
| [F-023](features/F-023-sidebar-search-and-reader-shell-polish.md) | Sidebar search and reader-shell polish | ✅ released | [F-023](features/F-023-sidebar-search-and-reader-shell-polish.md) |
| [F-025](features/F-025-zen-mode.md) | Zen mode | ✅ released | [F-025](features/F-025-zen-mode.md) |
| [F-024](features/F-024-cli-pdf-export-subcommand.md) | CLI PDF export subcommand | ✅ released | [F-024](features/F-024-cli-pdf-export-subcommand.md) |
| [F-026](features/F-026-default-cwd-render.md) | Default current-directory Markdown render | ✅ released | [F-026](features/F-026-default-cwd-render.md) |
| [F-027](features/F-027-unified-wipe-command.md) | Unified `wipe` cleanup command | ✅ released | [F-027](features/F-027-unified-wipe-command.md) |
| [F-028](features/F-028-cli-shorthand-and-directory-entry.md) | CLI shorthand flags and directory entry handling | ✅ released | [F-028](features/F-028-cli-shorthand-and-directory-entry.md) |
| [F-029](features/F-029-pdf-path-ergonomics-and-completions.md) | PDF path ergonomics and completions | ✅ released | [F-029](features/F-029-pdf-path-ergonomics-and-completions.md) |

---

## 🐛 Bugs

| ID | Title | Severity | Status | Doc |
|----|-------|----------|--------|-----|
| [B-001](bugs/B-001-non-md-files-not-copied.md) | Non-Markdown linked files not copied | Medium | ✅ released | [B-001](bugs/B-001-non-md-files-not-copied.md) |
| [B-002](bugs/B-002-cache-linked-file-staleness.md) | Render cache misses linked-file changes | Medium | ✅ released | [B-002](bugs/B-002-cache-linked-file-staleness.md) |
| [B-003](bugs/B-003-cache-theme-mismatch.md) | Theme change not reflected on cache hit | Low | ✅ released | [B-003](bugs/B-003-cache-theme-mismatch.md) |
| [B-004](bugs/B-004-completion-subcommands-after-file.md) | Completion suggests subcommands after FILE | Medium | ✅ released | [B-004](bugs/B-004-completion-subcommands-after-file.md) |
| [B-005](bugs/B-005-sidebar-footer-hotkeys-hidden.md) | Sidebar footer hotkeys can disappear behind tall hierarchies | Medium | ✅ released | [B-005](bugs/B-005-sidebar-footer-hotkeys-hidden.md) |
| [B-006](bugs/B-006-stray-checkbox-in-reader-shell.md) | Stray unlabeled checkbox appears in the reader shell | Medium | ✅ released | [B-006](bugs/B-006-stray-checkbox-in-reader-shell.md) |
| [B-007](bugs/B-007-c-hotkey-not-toggling.md) | `c` hotkey only opens config pane, does not toggle it | Medium | ✅ released | [B-007](bugs/B-007-c-hotkey-not-toggling.md) |
| [B-008](bugs/B-008-letter-top-misaligned.md) | Letter top edge misaligned with header buttons | Medium | ✅ released | [B-008](bugs/B-008-letter-top-misaligned.md) |
| [B-009](bugs/B-009-terminal-accordion-visual-issues.md) | Terminal command accordion card styling + Copy button placement | Medium | ✅ released | [B-009](bugs/B-009-terminal-accordion-visual-issues.md) |
| [B-010](bugs/B-010-theme-resets-on-navigation.md) | User-selected theme resets after hierarchy navigation | High | ✅ released | [B-010](bugs/B-010-theme-resets-on-navigation.md) |
| [B-011](bugs/B-011-config-sidebar-presentation-regressions.md) | Config sidebar overlaps controls, lacks matching animation, and allows blank layout values | Medium | ✅ released | [B-011](bugs/B-011-config-sidebar-presentation-regressions.md) |
| [B-012](bugs/B-012-root-cli-file-command-overlap.md) | Root CLI mixes optional file and command forms | High | ✅ released | [B-012](bugs/B-012-root-cli-file-command-overlap.md) |
| [B-013](bugs/B-013-zen-mode-background-does-not-match-letter.md) | Zen mode background does not match the effective letter color | Medium | ✅ released | [B-013](bugs/B-013-zen-mode-background-does-not-match-letter.md) |
| [B-014](bugs/B-014-zen-mode-page-does-not-fully-become-letter.md) | Zen mode page does not fully become the letter surface | Medium | ✅ released | [B-014](bugs/B-014-zen-mode-page-does-not-fully-become-letter.md) |
| [B-015](bugs/B-015-config-pane-stacks-beneath-shell-buttons.md) | Config pane stacks beneath shell buttons | Medium | ✅ released | [B-015](bugs/B-015-config-pane-stacks-beneath-shell-buttons.md) |

---

## 📋 Progress

Current operational ledger — statuses, active worktrees, agents, blockers:

→ [progress.md](progress.md)

---

## 📝 Prompts

| ID | Linked Item | Doc |
|----|-------------|-----|
| [P-010](prompts/P-010-F-010-linked-markdown-rendering.md) | F-010 | Recursive linked Markdown rendering |
| [P-011](prompts/P-011-B-001-copy-assets.md) | B-001 | Non-md linked files not copied |
| [P-012](prompts/P-012-M-011-nav-chrome.md) | M-011 (F-011+F-012) | Breadcrumbs + sidebar |
| [P-013](prompts/P-013-F-013-render-memory.md) | F-013 | Render memory |
| [P-014](prompts/P-014-F-014-folder-hierarchy.md) | F-014 | Preserve folder hierarchy in rendered output and sidebar tree |
| [P-015](prompts/P-015-M-014-view-controls-and-render-modes.md) | M-014 (F-015+F-016) | View controls and render modes |
| [P-016](prompts/P-016-B-004-completion-after-file.md) | B-004 | Fix completion suggestions after positional FILE |
| [P-017](prompts/P-017-F-017-template-shell.md) | F-017 | Implement template-driven render shell from `src/index.html` |
| [P-018](prompts/P-018-F-017-theme-icon-followup.md) | F-017 | Restore theme-switcher icon + label parity in the F-017 worktree |
| [P-019](prompts/P-019-M-017-reader-controls-polish.md) | M-017 (F-019 + B-006) | Reader controls polish — config icon, c/t hotkeys, live preview, rem units, bg glow, stray checkbox |
| [P-022](prompts/P-022-M-020-pdf-export-letter-alignment.md) | M-020 (F-022 + B-008 + B-009) | PDF export + final reader shell polish |
| [P-023](prompts/P-023-M-021-reader-shell-polish.md) | M-021 (F-023 + F-025 + B-010 + B-011) | Reader shell polish — search, zen mode, theme persistence, config sidebar fixes |
| [P-024](prompts/P-024-M-022-cli-shape-and-pdf.md) | M-022 (F-024 + B-012) | CLI shape cleanup + `mark pdf` subcommand |
| [P-025](prompts/P-025-B-013-zen-mode-background.md) | B-013 | Zen-mode background synchronization |
| [P-026](prompts/P-026-M-023-default-cwd-render.md) | M-023 (F-026) | Default current-directory Markdown entry |
| [P-027](prompts/P-027-M-024-wipe-command.md) | M-024 (F-027) | Replace `cleanup-home` and `--cleanup` with unified `wipe` |
| [P-028](prompts/P-028-M-025-cli-ergonomics-and-shell-regressions.md) | M-025 (F-028 + F-029 + B-014 + B-015) | CLI shorthand, directory input, PDF path/completion, zen/config regressions |
| m1–m9 | Original milestones | [m1](prompts/m1.txt) · [m2](prompts/m2.txt) · [m3](prompts/m3.txt) · [m4](prompts/m4.txt) · [m5](prompts/m5.txt) · [m6](prompts/m6.txt) · [m7](prompts/m7.txt) · [m8](prompts/m8.txt) · [m9](prompts/m9.txt) |

---

## 🔑 Quick Status

| Area | Current state |
|------|--------------|
| Active agents | none |
| Open worktrees | none |
| Latest tag | `v0.13.0` |
| Recently merged work | M-025 (`v0.13.0`) · M-024 (`v0.12.0`) |
| Next planned work | backlog empty |
| CI | ✅ fmt + clippy + test (Linux · macOS · Windows) |
