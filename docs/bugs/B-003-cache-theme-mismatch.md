# B-003 — Theme Change Not Reflected on Cache Hit

## Bug ID
B-003

## Title
Cache hit ignores `--theme` change — wrong-theme HTML served without warning

## Severity
Low — functionally incorrect output but requires an uncommon usage pattern (explicitly switching themes between runs)

## Symptoms
1. User renders `file.md` with default (light) theme → cache entry written.
2. User re-runs `mark --theme dark file.md`.
3. Cache hit (mtime unchanged) → prompted "Re-render? [y/N]"
4. User presses Enter (default N) → old light-theme HTML opened, not dark.

## Expected Behavior
If the requested theme differs from the theme used in the cached render, the cache should be treated as stale and re-render automatically.

## Actual Behavior
Theme is not stored in `CacheEntry`. Any theme-change invocation can produce a cache hit on a wrong-theme file.

## Reproduction Steps
1. `mark file.md` (light theme, default)
2. `mark --theme dark file.md` → observe prompt and default-N serves wrong theme

## Affected Area
- `src/cache.rs` — `CacheEntry` schema (no theme field)
- `src/main.rs` — cache key check (theme not compared)

## Acceptance Criteria for Fix
1. `CacheEntry` stores the `theme` used at render time (as a string: `"light"` / `"dark"`).
2. If the current resolved theme differs from the cached theme, cache is treated as stale → re-render silently.
3. All existing tests pass; new test covers theme-mismatch cache invalidation.

## Status
planned
