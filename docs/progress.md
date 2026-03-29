# Progress Ledger

## Legend

| Status | Meaning |
|--------|---------|
| `planned` | Identified, not yet ready to start |
| `ready` | Unblocked, can be picked up |
| `in_progress` | Actively being worked |
| `blocked` | Cannot proceed — see notes |
| `review` | Awaiting Product Owner review |
| `merged` | Merged to main |
| `released` | Tagged and released |

---

## Active Items

| ID | Type | Title | Status | Agent | Worktree | Branch | Blockers | Updated |
|----|------|-------|--------|-------|----------|--------|----------|---------|
| B-001 | bug | Non-md linked files not copied | `released` | anvil | `.worktrees/B-001-copy-assets` | `fix/B-001-copy-assets` | — | 2026-03-29 |
| M-011 | milestone | Navigation Chrome (breadcrumbs + sidebar) | `in_progress` | anvil | `.worktrees/M-011-nav-chrome` | `feat/M-011-nav-chrome` | awaiting merge (rebase onto v0.2.1) | 2026-03-29 |
| F-011 | feature | Breadcrumb navigation | `in_progress` | anvil | `.worktrees/M-011-nav-chrome` | `feat/M-011-nav-chrome` | — | 2026-03-29 |
| F-012 | feature | Sidebar hierarchy | `in_progress` | anvil | `.worktrees/M-011-nav-chrome` | `feat/M-011-nav-chrome` | — | 2026-03-29 |
| F-013 | feature | Render memory + re-render confirmation | `review` | anvil | `.worktrees/F-013-render-memory` | `feat/F-013-render-memory` | ✅ reviewed — hold merge until M-011 merges | 2026-03-29 |
| M-012 | milestone | Render Memory | `review` | anvil | `.worktrees/F-013-render-memory` | `feat/F-013-render-memory` | hold merge until M-011 merges | 2026-03-29 |

## Released Items

| ID | Type | Title | Status | Release |
|----|------|-------|--------|---------|
| F-010 | feature | Recursive linked Markdown rendering | `released` | v0.2.0 |
| M-010 | milestone | Linked Markdown Navigation | `released` | v0.2.0 |

---

## Released Items

| ID | Type | Title | Status | Release |
|----|------|-------|--------|---------|
| M1 | milestone | Project scaffold and CLI skeleton | `released` | v0.1.0 |
| M2 | milestone | Markdown rendering and output writing | `released` | v0.1.0 |
| M3 | milestone | Browser opening and cleanup | `released` | v0.1.0 |
| M4 | milestone | Install and uninstall scripts | `released` | v0.1.0 |
| M5 | milestone | Documentation, polish, CI | `released` | v0.1.0 |
| M6 | milestone | Shell autocomplete | `released` | v0.1.0 |
| M7 | milestone | Code block copy actions | `released` | v0.1.0 |
| M8 | milestone | Persistent theme configuration | `released` | v0.1.0 |
| M9 | milestone | Home folder cleanup command | `released` | v0.1.2 |
