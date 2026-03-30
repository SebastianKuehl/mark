# B-005 — Sidebar Footer Hotkeys Can Disappear Behind Tall Hierarchies

## Bug ID
B-005

## Title
Sidebar footer hotkeys stop being visible when the hierarchy exceeds the viewport height

## Severity
Medium — keyboard discoverability degrades on larger documentation trees and the footer hint can effectively disappear.

## Symptoms
1. Render a recursive documentation set with a tall sidebar hierarchy.
2. Open the sidebar.
3. Scroll state and content height can push the footer hint out of view or behind overflowing content.

## Expected Behavior
The footer hotkeys remain visible at the bottom of the sidebar, with a background treatment that keeps the text readable even if tree content scrolls beneath it.

## Actual Behavior
The footer is part of the scrollable layout and can be obscured by long hierarchy content.

## Reproduction Steps
1. Render a recursive Markdown set with enough linked files to overflow the sidebar.
2. Open the sidebar.
3. Observe that the hotkey footer is no longer reliably visible.

## Affected Area
- `src/render.rs` — sidebar shell markup and injected shell CSS

## Acceptance Criteria for Fix
1. The footer is pinned to the bottom of the sidebar shell.
2. It remains visible regardless of hierarchy height.
3. It uses a proper background and separation treatment so the hint stays legible.
4. Existing sidebar visibility and keyboard-toggle behavior remain intact.

## Status
merged
