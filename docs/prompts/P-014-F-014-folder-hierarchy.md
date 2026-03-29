# P-014 — F-014: Preserve Folder Hierarchy in Rendered Output and Sidebar

## Prompt ID
P-014

## Linked Item
F-014 — Preserve folder hierarchy in rendered output and sidebar tree
Milestone: M-013

## Task Objective
Change `mark` so that multi-file renders preserve the source directory hierarchy inside a per-invocation output directory, and the sidebar renders a collapsible folder tree instead of a flat list.

---

## Worktree Instructions

```sh
cd /Users/sebastian/Documents/repos/mark
git worktree add .worktrees/F-014-folder-hierarchy -b feat/F-014-folder-hierarchy main
cd .worktrees/F-014-folder-hierarchy
```

All work must happen inside `.worktrees/F-014-folder-hierarchy`.

---

## Exact Scope

### 1. Per-invocation render directory (`src/storage.rs`, `src/main.rs`)

Add a new function to `src/storage.rs`:

```rust
/// Create and return a unique per-invocation run directory inside `rendered_dir`.
/// Name pattern: `<entry-stem>-<timestamp>-<hash>/`
pub fn make_run_dir(rendered_dir: &Path, entry_path: &Path) -> Result<PathBuf, MarkError>
```

This uses the same stem/timestamp/hash scheme as the old `output_filename`, but produces a **directory** name instead of a file name.

In `src/main.rs`, replace the current Phase 2 (flat filename assignment) with:
1. Call `storage::make_run_dir(&paths.rendered, &entry_canonical)` to get `run_dir: PathBuf`.
2. For every file in `ordered` (BFS list), compute its **relative path from the entry-point's directory**:
   ```
   relative = file_canonical.strip_prefix(entry_dir)
               .unwrap_or(file_canonical.file_name().as_path())
   html_relative = relative.with_extension("html")
   output_path = run_dir.join(html_relative)
   ```
   Store `(canonical → output_path)` in `output_path_map: HashMap<PathBuf, PathBuf>`.
3. Ensure each output file's parent directory exists (`std::fs::create_dir_all`).

### 2. Asset copying (`src/main.rs` Phase 3)

Update the asset-copy block (B-001) to also preserve relative paths:

```rust
let relative = asset_canonical.strip_prefix(entry_dir)
               .unwrap_or_else(|_| Path::new(asset_canonical.file_name().unwrap()));
let dest = run_dir.join(relative);
std::fs::create_dir_all(dest.parent().unwrap())?;
```

### 3. Link rewriting

The `link_map` values are `PathBuf` absolute paths to the output files — this already works. No change to the rewriting logic itself; just ensure the paths in `link_map` are the new `run_dir`-relative absolute paths.

### 4. Sidebar tree rendering (`src/render.rs`)

Replace the current flat `all_files: Vec<(String, PathBuf, bool)>` with a tree structure.

**New type** (can be defined inline or in a new small module):

```rust
pub struct SidebarNode {
    pub name: String,           // display name (file stem or dir name)
    pub path: Option<PathBuf>,  // Some(html_path) for files, None for directories
    pub is_current: bool,
    pub children: Vec<SidebarNode>,
}
```

**New helper** in `src/render.rs`:

```rust
/// Build a SidebarNode tree from the flat all_files list.
/// Each entry's relative position in the tree is derived from its relative path
/// under the run_dir (strip run_dir prefix → relative path → tree position).
pub fn build_sidebar_tree(
    all_files: &[(String, PathBuf, bool)],   // (display_name, html_abs_path, is_current)
    run_dir: &Path,
) -> Vec<SidebarNode>
```

Pass `run_dir` through from `main.rs` into `render_markdown_rewriting_links`.

**HTML output** for the tree:

```html
<nav class="mark-sidebar">
  <ul class="mark-sidebar-tree">
    <li class="mark-sidebar-file mark-sidebar-current"><span>overview</span></li>
    <li class="mark-sidebar-dir">
      <input type="checkbox" id="sd-chapters" checked>
      <label for="sd-chapters">chapters/</label>
      <ul>
        <li class="mark-sidebar-file"><a href="/abs/path/intro.html">intro</a></li>
        <li class="mark-sidebar-dir">
          <input type="checkbox" id="sd-api" checked>
          <label for="sd-api">api/</label>
          <ul>
            <li class="mark-sidebar-file"><a href="/abs/path/endpoints.html">endpoints</a></li>
          </ul>
        </li>
      </ul>
    </li>
  </ul>
</nav>
```

- Directory nodes use a CSS checkbox toggle (same pattern as the outer sidebar).
- All directory checkboxes default to `checked` (open).
- IDs for directory checkboxes must be unique (use a counter or path-based slug).

Update `src/style.css` with styles for `.mark-sidebar-tree`, `.mark-sidebar-dir`, `.mark-sidebar-file`, indentation, and tree connector lines (optional, tasteful).

### 5. Signature update for `render_markdown_rewriting_links`

Add `run_dir: &Path` parameter:

```rust
pub fn render_markdown_rewriting_links(
    markdown: &str,
    title: &str,
    theme: Theme,
    link_map: &HashMap<String, PathBuf>,
    breadcrumb: &[(String, PathBuf)],
    all_files: &[(String, PathBuf, bool)],
    run_dir: &Path,                          // NEW
) -> String
```

### 6. Cleanup (`src/cleanup.rs`)

Update the 30-day cleanup to iterate over **subdirectories** of `~/.mark/rendered/` (the per-invocation run dirs) and delete entire directories whose **oldest file mtime** (or the directory's own mtime) is older than 30 days. Previously it deleted individual `.html` files.

### 7. Render cache (`src/cache.rs`, `src/main.rs`)

`CacheEntry.rendered_html` should store the **run directory path** (not an individual file path), so `remove_missing_entries` checks if the run directory exists. Update accordingly.

---

## Files to Inspect

- `src/storage.rs` — `output_filename`, `write_rendered`, `AppPaths`
- `src/main.rs` — Phase 1 (BFS), Phase 2 (filename assignment), Phase 3 (render loop), asset copy, cleanup call
- `src/render.rs` — `render_markdown_rewriting_links`, sidebar HTML generation, `build_sidebar_tree` (new)
- `src/style.css` — sidebar tree styles
- `src/cleanup.rs` — 30-day cleanup logic
- `src/cache.rs` — `CacheEntry`, `remove_missing_entries`
- `docs/features/F-014-folder-hierarchy.md` — acceptance criteria

---

## Implementation Constraints

- Do not add new external crate dependencies.
- `render_markdown` (no-nav variant) does not need `run_dir` — keep its signature unchanged; it can pass an empty slice and a dummy path to the inner builder.
- Ensure `cargo test` passes with all existing tests; add new tests for the new helpers.
- The old flat `output_filename` function in `storage.rs` can be repurposed or replaced by `make_run_dir` — just ensure no test or code path still depends on the old flat scheme.

---

## Acceptance Criteria

1. `mark overview.md` creates `~/.mark/rendered/overview-<ts>-<hash>/` and all files inside mirror source relative paths.
2. `mark --cleanup` removes per-invocation directories older than 30 days (not individual files).
3. Sidebar shows a collapsible folder tree; directories are collapsible via CSS checkboxes.
4. Current page highlighted in tree; all other file entries are clickable links.
5. Files in different subdirs with the same basename do not collide.
6. Asset files copied preserving relative path under run dir.
7. All pre-existing tests pass.
8. New tests cover: `make_run_dir`, relative path computation, `build_sidebar_tree` structure.

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
Item ID:        F-014
Prompt ID:      P-014
Worktree path:  .worktrees/F-014-folder-hierarchy
Branch name:    feat/F-014-folder-hierarchy
Summary:        <what was implemented>
Changed files:  <list>
Checks run:     cargo fmt --all, cargo clippy --all-targets --all-features -- -D warnings, cargo test
Check results:  <pass or describe failures>
Known issues:   <caveats>
README update needed: yes — document per-invocation run dir layout and sidebar tree
```
