# B-010 — User-Selected Theme Resets on Hierarchy Navigation

## Bug ID
B-010

## Title
Selecting another file from the hierarchy restores the original rendered theme

## Severity
High — it breaks the user's active reading preference inside the rendered app
and makes the theme toggle feel unreliable.

## Symptoms
1. Render a document with one theme.
2. Use `t` or the config controls to switch to the other theme in the browser.
3. Click another file in the hierarchy sidebar.
4. The newly opened page falls back to the theme embedded at render time rather
   than preserving the user's last explicit choice.

## Expected Behavior
Once the user explicitly chooses a theme in the rendered reader, that choice
should persist across in-app navigation until changed again.

## Actual Behavior
Hierarchy navigation reopens pages with the original rendered theme, discarding
the user's override.

## Reproduction Steps
1. Run `mark` on a directory tree with multiple linked Markdown files.
2. Open the rendered page.
3. Press `t` to change theme.
4. Click another file in the left hierarchy.
5. Observe the theme reverting.

## Affected Area
- `src/render.rs` — client-side theme persistence and navigation shell behavior

## Acceptance Criteria for Fix
1. A user-selected theme is stored in browser state that survives opening other
   rendered pages from the hierarchy.
2. On page load, the stored user theme takes precedence over the render-time
   theme when present.
3. Existing theme controls and cache behavior remain correct.
4. Tests cover the persisted-theme precedence behavior where practical.

## Status
released
