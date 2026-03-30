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

_No active items._

---

## Ready Items

| ID | Type | Title | Status | Agent | Worktree | Branch | Blockers | Updated |
|----|------|-------|--------|-------|----------|--------|----------|---------|
| M-023 | milestone | Default current-directory Markdown entry | `in_progress` | anvil | `.worktrees/M-023-default-cwd` | `feat/M-023-default-cwd` | — | 2026-03-30 |
| F-026 | feature | Default current-directory Markdown render | `in_progress` | anvil | `.worktrees/M-023-default-cwd` | `feat/M-023-default-cwd` | M-023 | 2026-03-30 |
| P-026 | task | M-023 default current-directory Markdown entry prompt | `in_progress` | anvil | `.worktrees/M-023-default-cwd` | `feat/M-023-default-cwd` | — | 2026-03-30 |
| B-013 | bug | Zen mode background does not match the effective letter color | `in_progress` | anvil | `.worktrees/B-013-zen-bg` | `fix/B-013-zen-bg` | F-025 follow-up after `v0.10.0` | 2026-03-30 |
| P-025 | task | B-013 zen-mode background synchronization prompt | `in_progress` | anvil | `.worktrees/B-013-zen-bg` | `fix/B-013-zen-bg` | — | 2026-03-30 |

---

## Recently Released Items

| ID | Type | Title | Status | Agent | Worktree | Branch | Release | Updated |
|----|------|-------|--------|-------|----------|--------|---------|---------|
| M-022 | milestone | CLI PDF export and command shape cleanup | `released` | Product Owner Agent | `main checkout` | `main` | `v0.10.0` | 2026-03-30 |
| F-024 | feature | CLI PDF export subcommand | `released` | Product Owner Agent | `main checkout` | `main` | `v0.10.0` | 2026-03-30 |
| B-012 | bug | Root CLI mixes optional file and command forms | `released` | Product Owner Agent | `main checkout` | `main` | `v0.10.0` | 2026-03-30 |
| P-024 | task | M-022 CLI shape and PDF subcommand prompt | `released` | Product Owner Agent | `main checkout` | `main` | `v0.10.0` | 2026-03-30 |
| M-021 | milestone | Sidebar search, theme persistence, and config panel polish | `released` | Product Owner Agent | `main checkout` | `main` | `v0.10.0` | 2026-03-30 |
| F-023 | feature | Sidebar search and reader-shell polish | `released` | Product Owner Agent | `main checkout` | `main` | `v0.10.0` | 2026-03-30 |
| F-025 | feature | Zen mode | `released` | Product Owner Agent | `main checkout` | `main` | `v0.10.0` | 2026-03-30 |
| B-010 | bug | User-selected theme resets after hierarchy navigation | `released` | Product Owner Agent | `main checkout` | `main` | `v0.10.0` | 2026-03-30 |
| B-011 | bug | Config sidebar overlaps header controls and lacks matching motion polish | `released` | Product Owner Agent | `main checkout` | `main` | `v0.10.0` | 2026-03-30 |
| P-023 | task | M-021 reader shell polish prompt | `released` | Product Owner Agent | `main checkout` | `main` | `v0.10.0` | 2026-03-30 |
| M-020 | milestone | PDF Export and Letter Alignment Polish | `released` | Product Owner Agent | `main checkout` | `main` | `v0.9.0` | 2026-03-30 |
| F-022 | feature | PDF export button with file picker | `released` | Product Owner Agent | `main checkout` | `main` | `v0.9.0` | 2026-03-30 |
| B-008 | bug | Letter top edge misaligned with header buttons | `released` | Product Owner Agent | `main checkout` | `main` | `v0.9.0` | 2026-03-30 |
| B-009 | bug | Terminal command accordion has wrong styling and Copy button placement | `released` | Product Owner Agent | `main checkout` | `main` | `v0.9.0` | 2026-03-30 |
| B-002 | bug | Render cache misses linked-file changes | `released` | Product Owner Agent | `main checkout` | `main` | `v0.9.1` | 2026-03-30 |
| B-003 | bug | Theme change not reflected on cache hit | `released` | Product Owner Agent | `main checkout` | `main` | `v0.6.1` | 2026-03-30 |

---

## Notes

- The M-021 and M-022 implementation commits were already present on `main` when this ledger was reconciled.
- Verification for the reconciled `v0.10.0` release state passed with `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- Full historical release history remains in `docs/releases.md`.
