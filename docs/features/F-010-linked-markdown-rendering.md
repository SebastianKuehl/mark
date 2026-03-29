# F-010 — Linked Markdown Rendering

## Feature ID
F-010

## Title
Recursive linked Markdown file rendering with HTML link rewriting

## User Value
Users can run `mark overview.md` on a file that links to other local Markdown files and navigate the fully rendered documentation in the browser without any extra commands. All inter-document links work as standard HTML `<a href>` links pointing to the rendered HTML files in `~/.mark/rendered/`.

## Scope Details

### Rendering behaviour
- After rendering the entry-point `.md` file, scan the rendered Markdown AST (or the source) for links whose target ends in `.md` (or `.markdown`) and resolves to a local file path.
- Render each discovered linked file recursively.
- Track visited files by canonical absolute path to avoid infinite loops from circular references.
- All linked `.md` files are rendered using the same theme and stored in `~/.mark/rendered/` with the same filename scheme as the entry-point file.

### Link rewriting
- In the final HTML output, rewrite every local `.md` link so it points to the corresponding rendered `.html` file path in `~/.mark/rendered/`.
- Links to external URLs (`http://`, `https://`, `mailto:`, etc.) are left unchanged.
- Links to non-Markdown local files (images, PDFs, etc.) are left unchanged.
- Anchor fragments (`#section`) appended to a Markdown link are preserved on the rewritten HTML link.

### CLI behaviour
- No new flags required. Recursive rendering is automatic whenever a local `.md` link is found.
- On each run, only files that are transitively reachable from the entry-point are rendered. Other stale rendered files are handled by the existing 30-day cleanup.
- Print a summary line for each additionally rendered file (e.g. `  → rendered: chapter1.md → ~/.mark/rendered/chapter1-<ts>-<hash>.html`).

## Dependencies
- Existing render pipeline (`src/render.rs`, `src/storage.rs`)
- `pulldown-cmark` link/image event parsing

## Acceptance Criteria
1. `mark overview.md` renders `overview.md` and all transitively linked local `.md` files.
2. In the rendered `overview.html`, every local `.md` link is rewritten to the corresponding `.html` path.
3. Clicking a link in the browser opens the correct rendered HTML file.
4. Circular links (A → B → A) do not cause infinite recursion or duplicate renders.
5. External URLs are not modified.
6. Anchor fragments are preserved.
7. All existing tests continue to pass.
8. New unit tests cover: link extraction, link rewriting, circular-reference deduplication.

## Priority
High

## Milestone
M-010

## Status
ready
