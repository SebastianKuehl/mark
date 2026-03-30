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
Current active milestone: `—` (M-017 released as v0.7.0).

---

## 📦 Releases

| Version | Type | Highlights |
|---------|------|------------|
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
| M-018 | Reader Shell UX Refinements | 📝 ready | [M-018](milestones/M-018-reader-shell-ux-refinements.md) |
| M-019 | Recursive Render Scope Restriction | 📝 ready | [M-019](milestones/M-019-recursive-render-scope-restriction.md) |
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
| [F-020](features/F-020-reader-shell-ux-refinements.md) | Reader shell UX refinements (letter alignment, accordion, save button) | 📝 ready | [F-020](features/F-020-reader-shell-ux-refinements.md) |
| [F-021](features/F-021-restrict-recursive-render-scope.md) | Restrict recursive rendering to entry file's directory subtree | 📝 ready | [F-021](features/F-021-restrict-recursive-render-scope.md) |

---

## 🐛 Bugs

| ID | Title | Severity | Status | Doc |
|----|-------|----------|--------|-----|
| [B-001](bugs/B-001-non-md-files-not-copied.md) | Non-Markdown linked files not copied | Medium | ✅ released | [B-001](bugs/B-001-non-md-files-not-copied.md) |
| [B-002](bugs/B-002-cache-linked-file-staleness.md) | Render cache misses linked-file changes | Medium | 📝 planned | [B-002](bugs/B-002-cache-linked-file-staleness.md) |
| [B-003](bugs/B-003-cache-theme-mismatch.md) | Theme change not reflected on cache hit | Low | 📝 planned | [B-003](bugs/B-003-cache-theme-mismatch.md) |
| [B-004](bugs/B-004-completion-subcommands-after-file.md) | Completion suggests subcommands after FILE | Medium | ✅ released | [B-004](bugs/B-004-completion-subcommands-after-file.md) |
| [B-005](bugs/B-005-sidebar-footer-hotkeys-hidden.md) | Sidebar footer hotkeys can disappear behind tall hierarchies | Medium | ✅ released | [B-005](bugs/B-005-sidebar-footer-hotkeys-hidden.md) |
| [B-006](bugs/B-006-stray-checkbox-in-reader-shell.md) | Stray unlabeled checkbox appears in the reader shell | Medium | ✅ released | [B-006](bugs/B-006-stray-checkbox-in-reader-shell.md) |
| [B-007](bugs/B-007-c-hotkey-not-toggling.md) | `c` hotkey only opens config pane, does not toggle it | Medium | 📝 ready | [B-007](bugs/B-007-c-hotkey-not-toggling.md) |

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
| m1–m9 | Original milestones | [m1](prompts/m1.txt) · [m2](prompts/m2.txt) · [m3](prompts/m3.txt) · [m4](prompts/m4.txt) · [m5](prompts/m5.txt) · [m6](prompts/m6.txt) · [m7](prompts/m7.txt) · [m8](prompts/m8.txt) · [m9](prompts/m9.txt) |

---

## 🔑 Quick Status

| Area | Current state |
|------|--------------|
| Active agents | none |
| Open worktrees | none |
| Latest tag | `v0.7.0` |
| Recently merged work | M-017 · F-019 · B-006 (v0.7.0) |
| Next planned work | M-018 (B-007 + F-020) · M-019 (F-021) · B-002 · B-003 |
| CI | ✅ fmt + clippy + test (Linux · macOS · Windows) |
