# P-013 — F-013: Render Memory and Re-render Confirmation

## Prompt ID
P-013

## Linked Item
F-013 — Render memory and re-render confirmation
Milestone: M-012

## Task Objective
Add a render cache to `mark` so that re-running `mark <file>` on an unchanged source file prompts the user to confirm re-rendering rather than blindly producing a duplicate rendered file.

**Prerequisite:** M-011 must already be merged into `main` before you begin.

---

## Worktree Instructions

```sh
cd /Users/sebastian/Documents/repos/mark
git worktree add .worktrees/F-013-render-memory -b feat/F-013-render-memory main
cd .worktrees/F-013-render-memory
```

---

## Exact Scope

### Cache file
- Location: `~/.mark/render-cache.toml`
- Format: TOML, keyed by canonical source path string
- Each entry stores:
  ```toml
  ["/abs/path/to/overview.md"]
  rendered_html = "/abs/path/to/rendered/overview-ts-hash.html"
  source_mtime_secs = 1711648523   # u64 Unix timestamp
  ```

### New module: `src/cache.rs`
Implement:
- `CacheEntry { rendered_html: PathBuf, source_mtime_secs: u64 }`
- `RenderCache` struct with load/save/get/set/remove methods
- Cache file path via `AppPaths`
- `load()` — reads from disk, returns empty cache if file missing or parse error (never panic)
- `save()` — writes to disk; best-effort, print warning on failure
- `get(source: &Path) -> Option<CacheEntry>`
- `set(source: &Path, entry: CacheEntry)`
- `remove_missing_entries(&mut self)` — drops entries whose `rendered_html` no longer exists on disk (used by `--cleanup`)

### Changes to `src/main.rs`
In the normal render flow (after resolving the entry-point path):
1. Load the cache.
2. Get the entry-point's current mtime (`std::fs::metadata(path)?.modified()?`).
3. Look up the cache:
   - **Hit + mtime unchanged + rendered HTML exists**: prompt `Already rendered: <path>\nRe-render? [y/N]: `
     - Read one line from stdin
     - `n` / empty / anything except `y`/`Y`: open existing HTML, exit
     - `y`/`Y`: proceed with full render
   - **Miss or mtime changed or rendered HTML missing**: proceed with render silently
4. After a successful render, update the cache entry for the entry-point file and save.
5. If `--no-open` is active, skip the prompt entirely and always re-render.

### Changes to `src/cleanup.rs`
Call `cache.remove_missing_entries()` and `cache.save()` during cleanup so stale entries are pruned.

### Changes to `src/cleanup_home.rs`
No change needed — `cleanup-home` removes all of `~/.mark` including the cache file.

---

## Files to Inspect

- `src/main.rs` — render entry point
- `src/storage.rs` — `AppPaths`; add `render_cache` path field
- `src/cleanup.rs` — add cache pruning
- `src/cleanup_home.rs` — verify cache file is inside `~/.mark` (it is, no change needed)
- `src/lib.rs` — add `pub mod cache`
- `docs/features/F-013-render-memory.md`

---

## Acceptance Criteria

1. Re-running `mark overview.md` on unchanged file shows prompt.
2. `n` / Enter opens existing rendered file, skips render.
3. `y` re-renders, updates cache.
4. Changed mtime → skips prompt, re-renders automatically.
5. Missing rendered HTML → skips prompt, re-renders automatically.
6. `--no-open` → no prompt, always re-renders.
7. `--cleanup` prunes cache entries whose HTML no longer exists.
8. `cleanup-home` removes `~/.mark` entirely (cache included).
9. All existing tests pass.
10. New tests cover: cache load/save/get/set, mtime hit, mtime miss, missing html miss, `--no-open` bypass, cleanup pruning.

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
- Do **not** change scope documents
- Do **not** declare work complete without the full handoff

---

## Completion Report Format

```
Item ID:        F-013
Prompt ID:      P-013
Worktree path:  .worktrees/F-013-render-memory
Branch name:    feat/F-013-render-memory
Summary:        <what was implemented>
Changed files:  <list>
Checks run:     cargo fmt --all, cargo clippy --all-targets --all-features -- -D warnings, cargo test
Check results:  <pass or describe failures>
Known issues:   <caveats>
README update needed: yes — document re-render prompt behaviour
```
