# F-014 — Preserve Folder Hierarchy in Rendered Output and Sidebar

## Feature ID
F-014

## Title
Preserve source folder hierarchy in rendered output directory and sidebar tree

## User Value
When rendering a multi-file Markdown project, `mark` currently dumps all files flat into `~/.mark/rendered/`. This loses the source structure, causes filename collisions for files in different subdirectories with the same name, and makes the sidebar show a flat unordered list. Users want to see their real folder structure preserved in both the output directory and the sidebar navigation tree.

## Scope Details

### Per-invocation render directory
Instead of writing all files directly into `~/.mark/rendered/`, create one subdirectory per invocation:

```
~/.mark/rendered/<entry-stem>-<ts>-<hash>/
```

All files for that run live inside it. The 30-day cleanup deletes entire per-invocation directories (not individual HTML files).

### Relative path preservation
Within the per-invocation directory, each rendered file mirrors its path relative to the entry-point's directory:

```
Source:                      Rendered:
overview.md               →  <run>/overview.html
chapters/intro.md         →  <run>/chapters/intro.html
chapters/api/endpoints.md →  <run>/chapters/api/endpoints.html
assets/logo.png           →  <run>/assets/logo.png
```

The entry-point's own directory is the root. All paths are computed relative to it.

### Asset copying
The same relative structure applies to non-Markdown asset files (B-001 behaviour). `assets/logo.png` copies to `<run>/assets/logo.png`. The filename-collision issue (two files with the same basename in different dirs) is resolved by this change.

### Sidebar tree
The sidebar currently renders a flat `<ul>` list. It must be updated to render a folder tree that mirrors the directory hierarchy:

```
▾ overview          ← entry-point (root)
  ▾ chapters/
      intro
    ▾ api/
        endpoints
assets/
  logo.png          ← linked assets shown as non-clickable or grayed entries (optional)
```

- Directories are shown as collapsible groups (CSS-only, no JS).
- Files within a directory are shown as links (or bold/highlighted if current).
- The current page is highlighted.
- The tree reflects the actual folder structure of the source files, not the BFS discovery order.

### Link rewriting
All `href` values in rendered HTML must point to the correct relative or absolute path within the per-invocation run directory. Existing absolute-path rewriting logic continues to apply.

### Render cache
`CacheEntry` stores the run directory path (the per-invocation directory). The cache check and update logic is otherwise unchanged. The per-invocation directory effectively namespaces cached results.

## Dependencies
- F-010, B-001, M-011, F-013 (all merged in v0.3.0)

## Acceptance Criteria
1. Running `mark overview.md` creates `~/.mark/rendered/overview-<ts>-<hash>/` containing all rendered files in their source-relative subdirectory structure.
2. `mark --cleanup` deletes per-invocation directories (not individual files) older than 30 days.
3. The sidebar renders a folder tree matching the real directory hierarchy.
4. Directories in the sidebar are collapsible (CSS only).
5. The current page is highlighted in the tree.
6. Files in different subdirectories with the same basename no longer collide.
7. Asset files (images, txt, etc.) are copied preserving their relative path.
8. All existing tests pass; new tests cover: relative path computation, per-invocation dir creation, sidebar tree HTML structure.

## Priority
High

## Milestone
M-013

## Status
released
