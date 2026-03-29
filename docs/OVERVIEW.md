# mark — Owner Overview

> Quick-navigation hub for all project documentation.
> Open this file with `mark docs/OVERVIEW.md` to browse everything in the browser.

---

## 🗂 Project

| Document | Purpose |
|----------|---------|
| [scope.md](scope.md) | Project summary, in/out of scope, constraints, success criteria |
| [project-spec.md](project-spec.md) | Original build specification |
| [worker-agent-rules.md](worker-agent-rules.md) | Rules governing all anvil implementation agents |
| [../README.md](../README.md) | Public-facing user documentation |

---

## 📦 Releases

| Version | Type | Highlights |
|---------|------|------------|
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
| M-011 | Navigation Chrome (breadcrumbs + sidebar) | 🔒 blocked on B-001 | [M-011](milestones/M-011-navigation-chrome.md) |
| M-012 | Render Memory | 🔒 blocked on M-011 | [M-012](milestones/M-012-render-memory.md) |
| M1–M9 | Original build milestones (scaffold → cleanup-home) | ✅ released | [milestones.md](milestones.md) |

---

## ✨ Features

| ID | Title | Status | Doc |
|----|-------|--------|-----|
| [F-010](features/F-010-linked-markdown-rendering.md) | Recursive linked Markdown rendering | ✅ released | [F-010](features/F-010-linked-markdown-rendering.md) |
| [F-011](features/F-011-breadcrumbs.md) | Breadcrumb navigation | 🔒 blocked on B-001 | [F-011](features/F-011-breadcrumbs.md) |
| [F-012](features/F-012-sidebar.md) | Sidebar hierarchy | 🔒 blocked on B-001 | [F-012](features/F-012-sidebar.md) |
| [F-013](features/F-013-render-memory.md) | Render memory + re-render confirmation | 🔒 blocked on M-011 | [F-013](features/F-013-render-memory.md) |

---

## 🐛 Bugs

| ID | Title | Severity | Status | Doc |
|----|-------|----------|--------|-----|
| [B-001](bugs/B-001-non-md-files-not-copied.md) | Non-Markdown linked files not copied | Medium | 🔧 in_progress | [B-001](bugs/B-001-non-md-files-not-copied.md) |

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
| m1–m9 | Original milestones | [m1](prompts/m1.txt) · [m2](prompts/m2.txt) · [m3](prompts/m3.txt) · [m4](prompts/m4.txt) · [m5](prompts/m5.txt) · [m6](prompts/m6.txt) · [m7](prompts/m7.txt) · [m8](prompts/m8.txt) · [m9](prompts/m9.txt) |

---

## 🔑 Quick Status

| Area | Current state |
|------|--------------|
| Active agents | None |
| Open worktrees | None |
| Latest tag | `v0.3.0` |
| Next planned work | B-002 (cache linked-file staleness) · B-003 (cache theme mismatch) |
| CI | ✅ fmt + clippy + test (Linux · macOS · Windows) |
