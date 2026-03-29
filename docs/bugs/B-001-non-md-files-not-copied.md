# B-001 — Non-Markdown Linked Files Not Accessible After Render

## Bug ID
B-001

## Title
Non-Markdown linked files not copied to rendered output directory

## Severity
Medium — linked files silently fail to load in the browser; the HTML link is present but the target is unreachable

## Symptoms
When a rendered Markdown file contains a link to a local non-Markdown file (e.g. a `.txt`, `.pdf`, `.png`, `.csv`), clicking that link in the browser results in a file-not-found error. The HTML link points to the original source path, which may be a relative path that does not resolve correctly from the `~/.mark/rendered/` directory.

## Expected Behavior
Clicking a link to a local non-Markdown file in a rendered page opens or downloads that file successfully in the browser.

## Actual Behavior
The browser cannot find the file because the link still points to the original relative path, which is not accessible from `~/.mark/rendered/`.

## Reproduction Steps
1. Create `overview.md` containing a link to a local file, e.g. `[Prompt](prompts/m1.txt)`.
2. Run `mark overview.md`.
3. Open the rendered HTML in the browser.
4. Click the link — browser shows file-not-found or similar error.

## Affected Area
- `src/main.rs` — BFS render loop; only handles `.md` links
- `src/render.rs` — `extract_local_md_links` only extracts `.md` targets; non-md links are left with their original relative paths in HTML

## Acceptance Criteria for Fix
1. During the BFS render loop, any local link target that is NOT a `.md`/`.markdown` file and resolves to an existing file is copied into `~/.mark/rendered/` (flat copy, preserving filename).
2. The HTML link is rewritten to point to the copied file's absolute path in `~/.mark/rendered/`.
3. If a file with the same name already exists in `~/.mark/rendered/`, the copy is skipped (idempotent) or a collision-safe name is used.
4. External URLs are not affected.
5. Broken links (pointing to files that do not exist) are left unchanged and do not cause a panic.
6. All existing tests continue to pass.
7. New tests cover: non-md file copy + link rewrite, missing file no-op, external URL unchanged.

## Status
in_progress
