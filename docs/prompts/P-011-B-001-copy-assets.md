# P-011 — B-001: Non-Markdown Linked Files Not Copied

## Prompt ID
P-011

## Linked Item
B-001 — Non-Markdown linked files not copied to rendered output directory

## Task Objective
Fix the bug where local non-Markdown file links (`.txt`, `.pdf`, `.png`, `.csv`, etc.) in rendered Markdown are left pointing at their original source path, making them unreachable from the browser when viewing files in `~/.mark/rendered/`.

---

## Worktree Instructions

Create a dedicated worktree before making any changes:

```sh
cd /Users/sebastian/Documents/repos/mark
git worktree add .worktrees/B-001-copy-assets -b fix/B-001-copy-assets main
cd .worktrees/B-001-copy-assets
```

All work must happen inside `.worktrees/B-001-copy-assets`. Never touch the main checkout.

---

## Exact Scope

### What to fix
In `src/main.rs`, the BFS render loop currently only discovers and renders `.md`/`.markdown` link targets. All other local file links are left with their original relative paths in the rendered HTML — meaning they cannot be resolved from `~/.mark/rendered/`.

### Required changes

#### 1. Extract non-md local links (`src/render.rs`)
Add a new public function `extract_local_asset_links(markdown: &str, source_dir: &Path) -> Vec<(String, PathBuf)>` that:
- Parses the Markdown AST using `pulldown-cmark`
- Collects link and image targets that:
  - are not external URLs (`http://`, `https://`, `//`, `mailto:`, `#`)
  - do NOT end with `.md` or `.markdown`
  - resolve to an existing file relative to `source_dir`
- Returns `(original_url_string, canonical_absolute_path)` pairs
- Deduplicates by canonical path

#### 2. Copy assets and rewrite links (`src/main.rs`)
In the BFS Phase 1 / Phase 3 loop, after collecting md links for each file, also call `extract_local_asset_links` and for each result:
- Copy the file to `~/.mark/rendered/<filename>` using `std::fs::copy`
  - If a file with that name already exists, skip the copy (idempotent)
  - If copy fails, print a warning and continue (best-effort, do not abort)
- Add an entry to the `link_map` for this file: `original_url_string → absolute_path_in_rendered_dir`

The existing `render_markdown_rewriting_links` function already rewrites any key in `link_map` — asset links will be rewritten automatically once added to the map.

#### 3. Broken links
If `extract_local_asset_links` finds a link target that does NOT resolve to an existing file, skip it silently (do not add to link_map, do not panic).

### What NOT to change
- Do not change the `.md` link discovery or rendering logic
- Do not change `render_markdown` or `render_markdown_rewriting_links` signatures
- Anchor fragments on asset links should be preserved the same way as md links (strip for copy lookup, re-append on rewritten href)

---

## Files to Inspect

- `src/render.rs` — `extract_local_md_links`, `render_markdown_rewriting_links`, `is_external_url`, `split_fragment`, `is_md_extension`
- `src/main.rs` — BFS Phase 1, Phase 2 (filename assignment), Phase 3 (render loop)
- `src/storage.rs` — `AppPaths`, rendered dir path
- `docs/bugs/B-001-non-md-files-not-copied.md` — acceptance criteria

---

## Acceptance Criteria

1. A Markdown file linking to `prompts/m1.txt` causes `m1.txt` to be copied to `~/.mark/rendered/m1.txt`.
2. The rendered HTML link for `prompts/m1.txt` is rewritten to the absolute path of the copied file.
3. Clicking the link in the browser opens/downloads the file successfully.
4. Re-running with the same asset does not error (idempotent copy).
5. A link to a non-existent file does not cause a panic or error exit.
6. External URLs are unchanged.
7. All pre-existing tests pass (69 tests).
8. New tests cover:
   - `extract_local_asset_links` returns correct pairs for local non-md links
   - external URLs excluded
   - `.md` links excluded (handled by the other extractor)
   - missing file is skipped

---

## Testing Requirements

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

All must pass with zero errors before reporting completion.

---

## Forbidden Actions

- Do **not** edit `README.md`
- Do **not** merge or push to `main`
- Do **not** create or move git tags
- Do **not** change scope, milestone, or feature documents
- Do **not** declare work complete without the full handoff report

---

## Completion Report Format

```
Item ID:        B-001
Prompt ID:      P-011
Worktree path:  .worktrees/B-001-copy-assets
Branch name:    fix/B-001-copy-assets
Summary:        <what was implemented>
Changed files:  <list>
Checks run:     cargo fmt --all, cargo clippy --all-targets --all-features -- -D warnings, cargo test
Check results:  <pass or describe failures>
Known issues:   <any caveats>
README update needed: no (internal fix, no new user-facing flags)
```
