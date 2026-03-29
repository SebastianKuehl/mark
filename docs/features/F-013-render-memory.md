# F-013 — Render Memory and Re-render Confirmation

## Feature ID
F-013

## Title
Remember previously rendered files; prompt user before re-rendering

## User Value
`mark` remembers the last rendered output for each source file. If a file was rendered recently and hasn't changed, the user is asked whether to re-render or just open the existing result — saving time and avoiding duplicate rendered files.

## Scope Details

### Storage
- A simple state file is stored at `~/.mark/render-cache.toml` (or similar).
- The cache maps canonical source path → `{ rendered_html_path, source_mtime_or_hash, rendered_at }`.
- Only the most recent rendered output per source file is tracked.

### Behaviour
- On every `mark <file>` invocation, check the cache for the entry-point file.
- If a cached entry exists **and** the source file's mtime matches (or content hash matches), prompt:
  ```
  Already rendered: ~/.mark/rendered/overview-<ts>-<hash>.html
  Re-render? [y/N]:
  ```
  - Default (Enter or `n`): open the existing rendered file, skip rendering.
  - `y`: re-render as normal, update cache.
- If `--no-open` is set, skip the prompt and always re-render (non-interactive mode).
- The `--cleanup` and `cleanup-home` subcommands clear the cache as appropriate.
- Cache is updated after every successful render.

### Cache invalidation
- If the source file's mtime has changed since the last render, skip the prompt and re-render silently (the file changed, so the cache is stale).
- If the rendered HTML file no longer exists on disk, skip the prompt and re-render.

## Dependencies
- F-010 (merged)
- M-011 (must be merged first — same `main.rs` area)

## Acceptance Criteria
1. Re-running `mark overview.md` on an unchanged file prompts for confirmation.
2. Answering `n` (or Enter) opens the existing rendered file without re-rendering.
3. Answering `y` re-renders and updates the cache.
4. A changed source file skips the prompt and re-renders automatically.
5. `--no-open` never prompts; always re-renders.
6. `--cleanup` removes stale cache entries whose HTML files no longer exist.
7. `cleanup-home` removes the cache file along with the rest of `~/.mark`.
8. All existing tests pass; new tests cover cache hit, cache miss, stale cache, `--no-open` bypass.

## Priority
Medium

## Milestone
M-012

## Status
blocked — waiting on M-011 merge
