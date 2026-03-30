# P-019 — M-017 Reader Controls Polish

## Prompt ID
P-019

## Linked Items
- Milestone: M-017 — Reader Controls Polish
- Feature: F-019 — Config menu hotkeys and live reader-layout preview polish
- Bug: B-006 — Stray unlabeled checkbox appears in the reader shell

## Task Objective
Refine the in-page reader control experience shipped in M-016 / F-018. This
covers seven distinct but co-located changes all in `src/render.rs` and
`src/style.css`. All changes must land in a single worktree.

## Exact Scope

### 1. `t` hotkey — toggle between light and dark only
The current `t` hotkey cycles through light → dark → system. Change it so it
only toggles between light and dark. Remove any system-mode path from the
keyboard handler. The `--theme system` CLI flag and programmatic system theme
are unaffected; only the in-page keyboard toggle changes.

### 2. Config/settings icon replaces theme icon on the top-right button
Replace the current SVG moon/sun icon on the top-right reader control button
with a gear or sliders SVG icon that communicates "settings" or "config". The
button still opens the same panel. Update any aria-label or title attribute
to say something like "Open config menu" rather than "Toggle theme".

### 3. New `c` hotkey — opens config menu
Add a keyboard handler so pressing `c` (when no input is focused) opens the
config/settings panel. Document the new hotkey in the in-page hotkey footer
alongside `t`, `s`, etc.

### 4. Reader-layout changes apply live to the current document
The reader-layout form currently only generates a CLI command for the user to
copy and run. In addition to that, changes to the sliders/inputs must now
immediately update the current page's CSS custom properties so the user can
see the effect live without re-rendering. Wire each layout control's `input`
event to update the corresponding CSS variable on `document.documentElement`.
The persistence command generation must still work exactly as before.

### 5. Reader width shown in `rem`, not inches
The letter-width control currently labels its value in inches (e.g.
`6.5 in`). Change the display unit to `rem`. Update:
- the label/output text next to the slider
- any placeholder or default text
- the JavaScript that formats the displayed value
The underlying stored value in `~/.mark/config.toml` and the CSS variable
(`--letter-width`) continue to use `rem` as they already do internally; only
the display label on the rendered page needs the update (remove "in", show
"rem").

### 6. Tone down the white radial background visual
The reader shell currently has a prominent white radial gradient or glow on
the page background. Reduce its opacity or scale substantially so it is a
very subtle, barely-there effect rather than a dominant visual element. The
goal is a clean, calm reading surface. Adjust the relevant CSS in
`src/style.css` and/or inline styles in `src/render.rs`.

### 7. Remove the stray unlabeled checkbox (B-006)
There is an unlabeled checkbox rendered in the top-left of the page chrome.
Find it in `src/render.rs` (it is likely a leftover `<input type="checkbox">`
that was used as a CSS-only toggle hack for the sidebar or some other panel).
If the behavior it drives is now handled by JavaScript, remove the element
entirely. If it still drives functional CSS state that has not yet been
migrated, add a visually-hidden `<label>` and `aria-label` and move it
off-screen with CSS so it is not visible but remains functional. Prefer
removing it entirely if it is truly unused.

## Files to Inspect
- `src/render.rs` — all rendered HTML markup, inline JS, and injected CSS
- `src/style.css` — shared styles embedded into the render shell
- `tests/` — any existing tests that assert rendered HTML output; update
  these to match the new markup

## Implementation Constraints
- Stable Rust only; no nightly features
- No new crate dependencies
- Do not change the `mark config set-layout` CLI command signature or the
  config.toml key names
- Do not change the underlying CSS variable names (`--letter-width`,
  `--font-size`, etc.)
- Keep all existing keyboard shortcuts working (`s` sidebar, `t` theme,
  `r` render mode, etc.)
- Do not edit `README.md`
- Do not merge into `main`
- Do not tag any release

## Acceptance Criteria
1. Pressing `t` switches only between light and dark themes. Pressing it
   repeatedly toggles between the two; it never enters a "system" mode.
2. The top-right config button displays a settings/gear icon (not a moon/sun).
3. Pressing `c` opens the config/settings panel.
4. Adjusting any reader-layout slider or input immediately updates the page
   layout (CSS custom properties) so the user can see the change live.
5. The letter-width label shows `rem` (e.g. `65 rem`), not `in`.
6. The page background radial gradient/glow is visibly much more subtle than
   before — roughly 10–20 % opacity or less, not the current dominant glow.
7. No unlabeled checkbox is visible anywhere in the rendered page chrome.
8. The `mark config set-layout` command output in the panel still reflects the
   current control values and is copyable.
9. All existing tests pass. Any tests that assert on rendered HTML output are
   updated to match the new markup. New tests are added where appropriate to
   cover changed behavior (e.g. hotkey hints, icon presence, checkbox absence).
10. `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`,
    and `cargo test` all pass with zero warnings.

## Testing Requirements
- Run `cargo fmt --all` (must produce no diff)
- Run `cargo clippy --all-targets --all-features -- -D warnings` (zero warnings)
- Run `cargo test` (all tests pass)
- If existing snapshot or HTML-content tests break due to markup changes,
  update them to match the correct new output

## Worktree Instructions
```
git worktree add .worktrees/M-017-reader-polish -b feat/M-017-reader-polish main
cd .worktrees/M-017-reader-polish
```
Do all work inside `.worktrees/M-017-reader-polish`. Do not touch the main
checkout.

## Forbidden Actions
- Do NOT edit `README.md`
- Do NOT merge to `main`
- Do NOT tag any release
- Do NOT change approved scope or item statuses in `docs/`
- Do NOT close or declare the task complete without Product Owner review
- Do NOT continue working if you hit an API rate limit — stop and notify
  the master via output text

## Completion Report Format
When done, provide a completion handoff with:
- Item IDs: M-017, F-019, B-006
- Prompt ID: P-019
- Worktree path: `.worktrees/M-017-reader-polish`
- Branch name: `feat/M-017-reader-polish`
- Summary of each change made
- List of changed files
- Tests run and results
- Known issues or follow-ups
- Whether `README.md` needs updating (do not edit it yourself)

## Rate-Limit Stop and Notify Instructions
If at any point you receive an API rate limit, token cap, or service
throttling error that prevents reliable continuation:
1. Stop all work immediately.
2. Do not retry.
3. Output a rate-limit notice to the master containing:
   - Agent role: anvil
   - Affected item IDs: M-017, F-019, B-006
   - Prompt ID: P-019
   - Worktree path and branch
   - What was completed before stopping
   - What remains blocked
   - The safe resume point
4. Do not continue as if nothing happened.
