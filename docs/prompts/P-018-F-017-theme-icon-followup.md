# P-018 — F-017 Follow-up: Restore Theme Switcher Icons

## Prompt ID
P-018

## Linked Item
F-017 — Use `src/index.html` as the rendered page template

## Task Objective
Fix the review regression in the current `F-017` implementation so the in-page theme switcher once again shows both an icon and a text label for `system`, `light`, and `dark`, consistent with the shipped `F-016` behavior.

---

## Worktree Instructions

The existing `F-017` worktree and branch already exist and contain the unmerged template-migration work:

```sh
cd /Users/sebastian/Documents/repos/mark/.worktrees/F-017-template-shell
git branch --show-current   # should be feat/F-017-template-shell
```

Work only inside `.worktrees/F-017-template-shell`.

Do **not** create a fresh branch from `main` for this follow-up unless the existing worktree is unavailable.

---

## Exact Scope

The current `F-017` implementation is close, but Product Owner review found a regression against the released `F-016` contract:

- `F-016` requires the theme switcher options to render **icon + label**
- the current `F-017` implementation appears to render only labels in the theme menu

### Required work

1. Update the current `F-017` implementation so the theme switcher again shows both an icon and a text label for:
   - `system`
   - `light`
   - `dark`
2. Preserve the template-driven shell migration already implemented.
3. Preserve existing theme switching behavior and sidebar behavior.
4. Add or restore regression coverage so this icon+label requirement is checked explicitly.

### Out of scope

- Do not rework unrelated parts of the template migration unless necessary for this fix.
- Do not edit `README.md`.
- Do not merge or tag releases.

---

## Files to Inspect

- `src/render.rs`
- `tests/view_controls.rs`
- any render tests in `src/render.rs`
- `docs/features/F-016-sidebar-theme-controls.md`
- `docs/features/F-017-template-shell.md`
- `README.md` (read-only)

---

## Implementation Constraints

- Keep the existing `F-017` worktree/branch as the base of this follow-up.
- Preserve the checked-in template shell approach from `F-017`.
- Restore parity with the shipped `F-016` theme-switcher icon+label behavior.
- Do not change scope/status docs.

---

## Acceptance Criteria

1. Theme switcher options render both an icon and a text label for `system`, `light`, and `dark`.
2. Template-shell rendering from `src/index.html` remains intact.
3. Existing theme switching and sidebar behavior still work.
4. Tests explicitly cover the icon+label expectation.
5. `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` all pass.

---

## Testing Requirements

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

All must pass.

---

## Forbidden Actions

- Do **not** edit `README.md`
- Do **not** merge or push to `main`
- Do **not** create or move git tags
- Do **not** change approved scope or status documents
- Do **not** declare work complete without the full handoff
- Do **not** continue through a blocking API rate-limit condition

---

## Rate-Limit Stop and Notify Instructions

If an API rate limit, token cap, service throttling response, or similar limitation prevents reliable continuation:

```text
Item ID:        F-017
Prompt ID:      P-018
Worktree path:  .worktrees/F-017-template-shell
Branch name:    feat/F-017-template-shell
Completed:      <what was finished before stopping>
Blocked:        <what remains blocked>
Rate limit:     <exact limitation, if known>
Resume point:   <safe next step once limits clear>
```

Stop immediately and do not report the task as complete.

---

## Completion Report Format

```text
Item ID:        F-017
Prompt ID:      P-018
Worktree path:  .worktrees/F-017-template-shell
Branch name:    feat/F-017-template-shell
Summary:        <what was implemented>
Changed files:  <list>
Checks run:     cargo fmt --all, cargo clippy --all-targets --all-features -- -D warnings, cargo test
Check results:  <pass or describe failures>
Known issues:   <caveats>
README update needed: <yes / no, and why>
```
