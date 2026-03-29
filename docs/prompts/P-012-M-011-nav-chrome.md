# P-012 — M-011: Breadcrumbs and Sidebar Navigation Chrome

## Prompt ID
P-012

## Linked Items
F-011 — Breadcrumb navigation on linked pages
F-012 — Sidebar with rendered file hierarchy
Milestone: M-011

## Task Objective
Add two navigation UI elements to every rendered HTML page:
1. **Breadcrumbs** — shown on pages below the entry-point, indicating the discovery path from root to current file.
2. **Sidebar** — shown on every page, listing all rendered files as a hierarchy with the current page highlighted and all others as clickable links. Collapsible.

**Prerequisite:** B-001 must already be merged into `main` before you begin. Confirm with `git log --oneline -3` that the B-001 fix commit is present.

---

## Worktree Instructions

```sh
cd /Users/sebastian/Documents/repos/mark
git worktree add .worktrees/M-011-nav-chrome -b feat/M-011-nav-chrome main
cd .worktrees/M-011-nav-chrome
```

All work must happen inside `.worktrees/M-011-nav-chrome`.

---

## Exact Scope

### Data to pass from `main.rs` into the renderer

The BFS loop in `main.rs` already knows:
- the ordered list of all rendered files (BFS discovery order)
- the parent of each file (which file linked to it)

You need to thread two additional pieces of data into the render call for each file:

1. **`breadcrumb: Vec<(String, PathBuf)>`** — ordered list of `(display_name, rendered_html_path)` from the entry-point down to (but not including) the current file. Empty for the entry-point itself.
2. **`all_files: Vec<(String, PathBuf, bool)>`** — full list of `(display_name, rendered_html_path, is_current)` for the sidebar, in BFS order.

`display_name` = the source file stem (e.g. `overview`, `chapter1`).

### Changes to `src/render.rs`

#### Update `render_markdown_rewriting_links` signature
Add two parameters:

```rust
pub fn render_markdown_rewriting_links(
    markdown: &str,
    title: &str,
    theme: Theme,
    link_map: &HashMap<String, PathBuf>,
    breadcrumb: &[(String, PathBuf)],          // NEW
    all_files: &[(String, PathBuf, bool)],      // NEW
) -> String
```

#### Breadcrumb HTML
If `breadcrumb` is non-empty, inject before the main `<body>` content:

```html
<nav class="mark-breadcrumb">
  <a href="/absolute/path/to/entry.html">entry</a> &rsaquo;
  <a href="/absolute/path/to/chapter1.html">chapter1</a> &rsaquo;
  <span class="mark-breadcrumb-current">appendix</span>
</nav>
```

Style with embedded CSS (in `style.css` or inline in the template) — consistent with light/dark themes.

#### Sidebar HTML
Inject a sidebar div. Use a CSS checkbox toggle for collapsibility (no external JS):

```html
<input type="checkbox" id="mark-sidebar-toggle" class="mark-sidebar-toggle" checked>
<label for="mark-sidebar-toggle" class="mark-sidebar-label">☰</label>
<nav class="mark-sidebar">
  <ul>
    <li><a href="/abs/path/overview.html">overview</a></li>
    <li class="mark-sidebar-current"><span>chapter1</span></li>
    <li><a href="/abs/path/appendix.html">appendix</a></li>
  </ul>
</nav>
```

The main content area must be offset/flex so the sidebar and content sit side-by-side. On narrow viewports (max-width: 700px), the sidebar overlays or stacks — pick whichever looks cleaner.

#### Update `render_markdown` (no-navigation variant)
Keep `render_markdown` working for standalone renders (used in tests). You may add defaulted/empty parameters or keep it as a wrapper that calls `render_markdown_rewriting_links` with empty slices.

### Changes to `src/main.rs`

Track parent relationships during BFS discovery (Phase 1). Build the breadcrumb and all_files vectors for each file before calling `render_markdown_rewriting_links` in Phase 3. Pass them through.

---

## Files to Inspect

- `src/render.rs` — HTML template, `render_markdown`, `render_markdown_rewriting_links`
- `src/main.rs` — BFS Phase 1 (add parent tracking), Phase 3 (build and pass nav data)
- `src/style.css` — existing embedded styles (breadcrumb + sidebar CSS goes here or inline)
- `docs/features/F-011-breadcrumbs.md`
- `docs/features/F-012-sidebar.md`

---

## Acceptance Criteria

1. Entry-point page: no breadcrumb, sidebar present with all files listed, entry highlighted.
2. Linked page (1 level deep): breadcrumb shows `entry ›`, sidebar present with correct highlight.
3. Linked page (2+ levels): full breadcrumb chain, sidebar correct.
4. All sidebar links navigate to correct rendered files.
5. Sidebar is collapsible (toggle button works via CSS, no JS required).
6. Light and dark themes both display breadcrumb and sidebar legibly.
7. Single-file renders (no linked files) still work correctly — sidebar shows only one entry (or is hidden), no breadcrumb.
8. All pre-existing tests pass.
9. New tests cover: breadcrumb HTML present for depth-1 page, absent for entry-point; sidebar HTML present on all pages; current-page highlight correct.

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
Item ID:        M-011 (F-011 + F-012)
Prompt ID:      P-012
Worktree path:  .worktrees/M-011-nav-chrome
Branch name:    feat/M-011-nav-chrome
Summary:        <what was implemented>
Changed files:  <list>
Checks run:     cargo fmt --all, cargo clippy --all-targets --all-features -- -D warnings, cargo test
Check results:  <pass or describe failures>
Known issues:   <caveats>
README update needed: yes — document breadcrumbs and sidebar
```
