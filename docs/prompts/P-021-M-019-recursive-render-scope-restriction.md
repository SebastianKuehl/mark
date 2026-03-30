# P-021 — M-019 Recursive Render Scope Restriction

## Prompt ID
P-021

## Linked Items
- M-019 — Recursive Render Scope Restriction
- F-021 — Restrict recursive rendering to entry file's directory subtree

## Task Objective
Restrict the BFS recursive link-following in `mark` so it only renders and
copies files that reside within the entry file's parent directory (and its
subdirectories). Links to files outside that subtree are silently skipped;
their HTML link targets are left unchanged in the output.

## Worktree Instructions
```
git worktree add .worktrees/M-019-render-scope -b feat/M-019-render-scope main
```
Work exclusively in `.worktrees/M-019-render-scope`. Do not touch the main
checkout.

## Files to Inspect
- `src/main.rs` — BFS discovery loop where linked files are queued for rendering
- `src/render.rs` — `extract_local_md_links` function and link rewriting logic
- `tests/` — existing integration tests for recursive rendering

## Exact Scope

### Scope-restriction logic
When `mark` is invoked on `path/to/entry.md`, compute the **entry directory**
as `path/to/` (canonical absolute path of the entry file's parent directory).

For every Markdown or asset link discovered during BFS:
1. Resolve the link to a canonical absolute path.
2. If the canonical path starts with (is under) the entry directory → process
   normally (render for Markdown, copy for assets).
3. If the canonical path does NOT start with the entry directory → skip the
   file; do not render, do not copy; leave the original HTML link target
   unchanged in the output.

### Where to apply it
- **`src/main.rs` BFS loop**: before queuing a discovered linked file, check
  whether its canonical path is within the entry directory. If not, skip.
- **`src/render.rs` `extract_local_md_links` / link rewriting**: ensure that
  when a link is skipped (out of scope), the rewritten HTML link is left as
  the original relative/absolute reference — not rewritten to a `.html` path.
  Pass the entry directory (or a closure/predicate) down to the rendering
  call so it can make the same in/out-of-scope decision when rewriting links.

### Canonical path comparison
Use `std::fs::canonicalize` where the file exists, or construct the canonical
path manually (resolve `..` segments) for paths that may not exist yet on
disk. Ensure the comparison uses a trailing separator or `starts_with` on path
components to avoid false positives (e.g. `docs2/` matching `docs/`).

### Single-file mode
Single-file rendering (`--single` flag or equivalent) is unaffected — it
never follows links. No change needed for that path.

### Out-of-scope links in HTML output
When a Markdown link to an out-of-scope file appears in the source, the
rendered HTML should retain the original link href (the relative Markdown
link path as written), not rewrite it to `.html`. This means the link will
likely be broken in the rendered output — that is acceptable and expected.
Do not add warning text or error markup to the output.

## Implementation Constraints
- Changes are primarily in `src/main.rs` and `src/render.rs`.
- Do NOT touch `README.md`.
- Do NOT merge to `main`.
- Do NOT tag releases.
- Do NOT change scope beyond what is listed above.
- All existing recursive rendering behavior for in-scope files must be
  unchanged.

## Acceptance Criteria
1. `mark docs/my-file.md` in recursive mode only renders files under `docs/`
   and its subdirectories.
2. A link to `../other/page.md` (outside entry dir) is not rendered; its HTML
   link is left as the original reference.
3. A link to `../images/logo.png` (outside entry dir) is not copied; its HTML
   link is left unchanged.
4. A link to `docs/subdir/page.md` (inside entry dir) is rendered normally.
5. `cargo fmt --all` passes.
6. `cargo clippy --all-targets --all-features -- -D warnings` passes with zero
   warnings.
7. `cargo test` passes — all existing tests green, new tests cover:
   - in-scope link is followed and rendered
   - out-of-scope link is skipped and its href is unchanged in output

## Forbidden Actions
- Edit `README.md`
- Merge to `main`
- Tag releases
- Change scope
- Close the task independently
- Continue after a blocking API rate-limit event

## Completion Report Format
Return a report containing:
- Item IDs: F-021, M-019
- Prompt ID: P-021
- Worktree: `.worktrees/M-019-render-scope`
- Branch: `feat/M-019-render-scope`
- Summary of changes made
- Files changed
- Tests run and results
- Known issues or follow-ups
- Whether README.md needs updating (recommendation only)

## Rate-Limit Stop and Notify
If you hit an API rate limit, token cap, or service throttling that prevents
reliable continuation:
1. Stop work immediately
2. Do not retry
3. Preserve current state
4. Output a rate-limit notice to the master containing: your role (anvil agent),
   item IDs (F-021/M-019), prompt ID (P-021), worktree path, branch,
   what was completed, what remains blocked, and the safe resume point
5. Do not mark the task as complete
