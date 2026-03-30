# F-019 — Config Menu Hotkeys and Live Reader-Layout Preview Polish

## Feature ID
F-019

## Title
Clarify the reader config entry point and make layout changes preview live

## User Value
Users should be able to discover and operate reader customization without guessing what the theme button does, and they should be able to see layout tweaks on the current page before deciding whether to persist them.

## Scope Details
- Replace the current top-right theme button icon with a config/settings-style icon.
- Reframe that control as the entry point for a config menu containing both theme controls and reader-layout controls.
- Change the `t` hotkey so it only toggles between light and dark themes.
- Add a new `c` hotkey that opens the config menu.
- Apply in-page reader-layout changes immediately to the currently viewed document instead of waiting for a later render.
- Present reader letter width in `rem` units instead of inches.
- Tone down the current white radial background visual substantially so the page chrome is less visually loud.
- Preserve the ability to emit the exact `mark config set-layout` command needed to save the chosen values.

## Dependencies
- F-018 — Persistent reader appearance controls
- F-016 — Sidebar and theme controls

## Acceptance Criteria
1. The top-right reader control uses a config-oriented icon and label treatment appropriate for a general settings menu.
2. The config menu contains both theme and reader-layout controls.
3. Pressing `t` switches only between light and dark themes.
4. Pressing `c` opens the config menu.
5. Changing reader-layout controls updates the current document view immediately.
6. The generated persistence command continues to reflect the currently selected values.
7. Reader width is shown in `rem`.
8. The background radial highlight is visibly reduced without harming readability.
9. Tests cover the updated shell behavior and any changed formatting/output expectations.

## Priority
High

## Milestone
M-017

## Status
released
