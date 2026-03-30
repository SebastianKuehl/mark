# B-002 — Render Cache Misses Linked-File Changes

## Bug ID
B-002

## Title
Render cache only checks entry-point mtime — stale linked files serve outdated HTML

## Severity
Medium — user edits a linked file (e.g. `chapter2.md`), re-runs `mark overview.md`, gets a cache hit and is served the old render. They must explicitly answer `y` to re-render, but there is no indication that a linked file changed.

## Symptoms
1. User renders `overview.md` → cache entry written.
2. User edits `chapter2.md` (linked from overview).
3. User re-runs `mark overview.md`.
4. Cache hit (overview.md mtime unchanged) → prompted "Re-render?"
5. User presses Enter (default N) → old rendered HTML opened; `chapter2.md` changes are invisible.

## Expected Behavior
If any transitively linked `.md` file has changed since the last render, the cache should be treated as stale and re-render should happen automatically (no prompt).

## Actual Behavior
Only the entry-point file's mtime is checked. Linked file changes are invisible to the cache.

## Reproduction Steps
1. `mark overview.md` (overview links to chapter2.md)
2. Edit `chapter2.md`
3. `mark overview.md` again — observe cache hit prompt despite linked file change

## Affected Area
- `src/cache.rs` — `CacheEntry` schema (no linked-file mtimes stored)
- `src/main.rs` — cache check block (only reads entry-point mtime)

## Acceptance Criteria for Fix
1. `CacheEntry` stores a map of `{ canonical_path: mtime }` for all files rendered in the last invocation (entry-point + all linked files).
2. Cache is stale if any stored mtime no longer matches the current mtime on disk.
3. Stale cache → re-render silently (no prompt), same as a cache miss.
4. Cache is updated with all rendered file mtimes after each successful render.
5. All existing tests pass; new tests cover multi-file mtime staleness detection.

## Status
released
