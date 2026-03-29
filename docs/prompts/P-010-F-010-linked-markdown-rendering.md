# P-010 — F-010: Recursive Linked Markdown Rendering

## Prompt ID
P-010

## Linked Item
F-010 — Recursive linked Markdown rendering with HTML link rewriting
Milestone: M-010

## Task Objective
Extend the `mark` CLI so that when it renders a Markdown file, it also discovers, renders, and rewrites all local `.md` links recursively. The result must be a fully browser-navigable set of HTML files stored in `~/.mark/rendered/`.

---

## Worktree Instructions

Before making any changes, create a dedicated git worktree:

```sh
cd /Users/sebastian/Documents/repos/mark
git worktree add .worktrees/F-010-linked-md -b feat/F-010-linked-md main
cd .worktrees/F-010-linked-md
```

All work must happen inside `.worktrees/F-010-linked-md`. Never touch the main checkout.

---

## Exact Scope

### 1. Link extraction
- After parsing Markdown (or during rendering), collect every link whose target:
  - does **not** start with `http://`, `https://`, `mailto:`, `//`, or `#`
  - ends with `.md` or `.markdown` (case-insensitive)
  - resolves to an existing file relative to the source file's directory

### 2. Recursive rendering
- Maintain a `visited: HashSet<PathBuf>` of canonical absolute paths.
- For each discovered unvisited `.md` link, render it the same way as the entry-point (same theme, same output dir, same filename scheme).
- Recurse into newly rendered files to find their links.
- Never render the same canonical path twice.

### 3. HTML link rewriting
- After rendering each file, rewrite local `.md` links in the HTML output so they point to the rendered `.html` path for that file.
- Anchor fragments (`#section`) must be preserved: `./api.md#endpoints` → `/Users/…/api-<ts>-<hash>.html#endpoints`.
- External URLs, image links, and non-Markdown file links must not be modified.

### 4. Output and messaging
- For each additionally rendered file (beyond the entry-point), print:
  `  → rendered: <relative-source-path> → <rendered-html-path>`
- No new CLI flags are required.

---

## Files to Inspect

- `src/main.rs` — entry point, CLI dispatch
- `src/render.rs` — HTML rendering pipeline; this is the primary area of change
- `src/storage.rs` — output path generation (`output_path` fn)
- `src/cli.rs` — CLI struct (read-only; no new flags needed)
- `src/lib.rs` — public API surface
- `docs/features/F-010-linked-markdown-rendering.md` — acceptance criteria

---

## Implementation Constraints

- Use `pulldown-cmark` events to extract link targets from the Markdown AST — do not regex-parse the raw source.
- Do not add new external crate dependencies unless absolutely necessary. Prefer std + existing deps.
- The visited-set must use **canonical absolute paths** (`std::fs::canonicalize`) to handle `./a.md` vs `a.md` pointing to the same file.
- Rewriting links must happen in the HTML string after initial rendering (a post-render string replace keyed on the original relative target → new HTML path is acceptable if robust; using a pulldown-cmark pass to emit modified link events is preferred).
- Maintain backward compatibility: files with no local `.md` links must behave exactly as before.
- Keep existing public function signatures where possible; add new helpers as needed.

---

## Acceptance Criteria

1. `mark overview.md` renders `overview.md` and all transitively reachable local `.md` files.
2. Every local `.md` link in the rendered HTML is rewritten to the corresponding `.html` rendered path.
3. Clicking links in the browser opens the correct rendered file.
4. Circular links (A → B → A) do not cause a stack overflow or duplicate renders.
5. External URLs are unchanged in rendered HTML.
6. Anchor fragments are preserved after link rewriting.
7. All pre-existing tests pass.
8. New tests cover:
   - local `.md` link extraction from a Markdown source string
   - link rewriting in rendered HTML output
   - circular-reference deduplication via visited set
   - non-Markdown and external links are not rewritten

---

## Testing Requirements

Run before reporting completion:

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

All three must pass with zero errors.

---

## Forbidden Actions

- Do **not** edit `README.md`
- Do **not** merge or push to `main`
- Do **not** create or move git tags
- Do **not** change scope, milestone, or feature documents
- Do **not** declare work complete without reporting the full handoff to the Product Owner Agent

---

## Completion Report Format

When done, report to the Product Owner Agent:

```
Item ID:        F-010
Prompt ID:      P-010
Worktree path:  .worktrees/F-010-linked-md
Branch name:    feat/F-010-linked-md
Summary:        <what was implemented>
Changed files:  <list every changed file>
Checks run:     cargo fmt --all, cargo clippy --all-targets --all-features -- -D warnings, cargo test
Check results:  <pass or describe failures>
Known issues:   <any caveats or follow-ups>
README update needed: yes — document recursive rendering and link rewriting behaviour
```
