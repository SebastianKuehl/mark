# P-025 — B-013 Zen Mode Background Sync

## Prompt ID
P-025

## Linked Items
- B-013 — Zen mode background does not adapt to the current letter color
- F-025 — Zen mode

## Task Objective
Fix zen mode so the full page background always adopts the effective current
letter background color when zen mode is enabled, then restore the normal shell
cleanly when zen mode is disabled.

## Worktree Instructions
```sh
git worktree add .worktrees/B-013-zen-bg -b fix/B-013-zen-bg main
```
Work **only** inside `.worktrees/B-013-zen-bg`. Never commit to `main`
directly.

## Files to Inspect
- `src/render.rs`
- `src/style.css`
- `docs/bugs/B-013-zen-mode-background-does-not-match-letter.md`
- Existing zen-mode tests in `src/render.rs`

## Exact Scope
- Update zen-mode client-side behavior so enabling zen mode reads/applies the
  effective current letter background color rather than leaving the page on a
  stale or mismatched outer background.
- Ensure the behavior works correctly across supported themes.
- Ensure disabling zen mode restores the normal shell/background behavior.
- Add or update regression tests for the zen-mode background synchronization.

## Implementation Constraints
- Keep scope limited to B-013.
- Prefer targeted changes in `src/render.rs`; touch `src/style.css` only if
  truly necessary.
- Do not edit `README.md`.
- Do not merge to `main`.
- Do not tag releases.

## Acceptance Criteria
1. Enabling zen mode updates the page background to the effective current letter
   background color.
2. The background remains correct across supported themes.
3. Disabling zen mode restores the normal reader shell styling.
4. Tests cover the zen-mode background behavior where practical.
5. `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`,
   and `cargo test` all pass.

## Testing Requirements
- Update or add render tests covering zen-mode background synchronization.
- Run:
  - `cargo fmt --all`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test`

## Forbidden Actions
- Do not edit `README.md`
- Do not merge to `main`
- Do not tag releases
- Do not change scope beyond B-013

## Rate-Limit Notice
If you hit an API rate limit that prevents reliable continuation, stop
immediately and notify the master via output text. Do not continue.

## Completion Report Format
Report back with:
- Item ID completed: B-013
- Prompt ID: P-025
- Worktree: `.worktrees/B-013-zen-bg`
- Branch: `fix/B-013-zen-bg`
- Summary of changes made
- Files changed
- Tests/checks run and results
- Any known issues or follow-ups
- Whether `README.md` needs updating (recommendation only)
