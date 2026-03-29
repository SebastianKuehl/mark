# P-017 — F-017: Use `src/index.html` as the Rendered Page Template

## Prompt ID
P-017

## Linked Item
F-017 — Use `src/index.html` as the rendered page template

## Task Objective
Replace the hand-built HTML shell in `src/render.rs` with a template-driven render flow based on `src/index.html`, while preserving the template's special shadcn-flavored structure and keeping shipped navigation/theme behavior intact.

---

## Worktree Instructions

```sh
cd /Users/sebastian/Documents/repos/mark
git worktree add .worktrees/F-017-template-shell -b feat/F-017-template-shell main
cd .worktrees/F-017-template-shell
```

All work must happen inside `.worktrees/F-017-template-shell`.

---

## Exact Scope

Implement `F-017` as documented in `docs/features/F-017-template-shell.md` and `docs/milestones/M-015-template-render-shell.md`.

The current state is:

- `src/render.rs` assembles a full HTML document string directly in Rust.
- `src/index.html` already contains the desired application shell and styling structure.
- `src/index.html` also still contains placeholder content:
  - rendered body placeholder in `.markdown-prose` (`<put rendered html here>`)
  - sidebar placeholder entries (`overview.html`, `docs/`, `getting-started.html`)

### Required work

1. Use `src/index.html` as the rendered page shell.
2. Replace the `.markdown-prose` placeholder with actual rendered Markdown HTML.
3. Replace the sidebar placeholder entries with the generated navigation tree for the current render.
4. Preserve the template's existing structural wrappers/classes for the content shell and sidebar so the provided styling remains in use.
5. Preserve shipped behavior from `v0.5.0`:
   - hidden-by-default sidebar unless configured visible
   - current sidebar tree population
   - files-first recursive ordering
   - current page highlighting
   - in-page theme switching
   - compatibility with recursive and single-file render modes
6. Add or update tests to cover the template substitution path and the preserved navigation/theme behavior.

### Allowed implementation direction

Use the smallest robust integration that keeps the checked-in template as the source of truth for page-shell markup. It is acceptable to normalize or remove template placeholder demo entries as part of the integration, but do not replace the template with a new hand-built shell.

---

## Files to Inspect

- `src/index.html`
- `src/render.rs`
- `src/style.css`
- `src/main.rs`
- `tests/render_integration.rs`
- `tests/view_controls.rs`
- `docs/features/F-017-template-shell.md`
- `docs/milestones/M-015-template-render-shell.md`
- `README.md` (read-only; do not edit)

---

## Implementation Constraints

- Do not edit `README.md`.
- Do not merge or tag releases.
- Do not change approved scope documents.
- Keep the template-based shell as the source of layout markup rather than re-encoding the same shell entirely in Rust.
- Preserve existing CLI behavior unless a change is required for this feature.
- Keep single-file mode sidebar behavior coherent with the released design.

---

## Acceptance Criteria

1. `mark` uses `src/index.html` as the source template for rendered pages.
2. The content placeholder in the template is replaced with rendered Markdown output.
3. The sidebar placeholder links/folders are replaced with generated navigation using the template's existing sidebar markup/styling structure.
4. Existing sidebar and theme features from `v0.5.0` remain intact after the migration.
5. Recursive renders still show the generated navigation tree; single-file renders still avoid sidebar content.
6. `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` all pass.

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

1. Stop work immediately
2. Do not keep retrying blindly
3. Preserve the current worktree state
4. Notify the master / Product Owner Agent via output text using this format:

```text
Item ID:        F-017
Prompt ID:      P-017
Worktree path:  .worktrees/F-017-template-shell
Branch name:    feat/F-017-template-shell
Completed:      <what was finished before stopping>
Blocked:        <what remains blocked>
Rate limit:     <exact limitation, if known>
Resume point:   <safe next step once limits clear>
```

Do not report the task as complete if rate limiting blocked reliable completion.

---

## Completion Report Format

```text
Item ID:        F-017
Prompt ID:      P-017
Worktree path:  .worktrees/F-017-template-shell
Branch name:    feat/F-017-template-shell
Summary:        <what was implemented>
Changed files:  <list>
Checks run:     cargo fmt --all, cargo clippy --all-targets --all-features -- -D warnings, cargo test
Check results:  <pass or describe failures>
Known issues:   <caveats>
README update needed: <yes / no, and why>
```
