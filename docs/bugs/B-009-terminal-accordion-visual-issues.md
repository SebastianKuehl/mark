# B-009 — Terminal Command Accordion Visual Issues

## Bug ID
B-009

## Title
Terminal command accordion has wrong styling and Copy button placement

## Severity
Medium — the accordion looks like a separate card/island within the config
pane rather than being flat and integrated; the Copy button is hidden inside
the collapsed content instead of being always visible in the header.

## Symptoms
1. The terminal command `<details>` element has visible card styling (border,
   background colour, or box-shadow) that makes it appear as a nested island
   inside the config pane rather than a seamless section of it.
2. The Copy button is only accessible when the accordion is expanded — it is
   inside the collapsible content rather than in the `<summary>` row.

## Expected Behavior
1. The accordion has no border, no background colour, and no box-shadow. It
   sits flat on the config pane surface, visually identical to the surrounding
   content.
2. The Copy button is permanently visible, right-aligned inside the
   `<summary>` row (the accordion header), regardless of whether the accordion
   is expanded or collapsed.

## Actual Behavior
1. The accordion renders with card-like visual treatment that separates it from
   the rest of the config pane.
2. The Copy button is only visible/accessible when the accordion is open.

## Reproduction Steps
1. Run `mark <file>.md` and open the rendered page.
2. Press `c` to open the config pane.
3. Observe the "Terminal command" accordion — note the card/island styling.
4. Note that the Copy button is not visible when the accordion is collapsed.

## Affected Area
- `src/render.rs` — HTML structure of the terminal command `<details>` block
  and the `<summary>` element; Copy button placement
- `src/style.css` or inline CSS in `src/render.rs` — accordion styling
  (border, background, box-shadow on the `<details>` or its container)

## Acceptance Criteria for Fix
1. The `<details>` element and its container have no border, no background
   colour different from the config pane, and no box-shadow.
2. The Copy button is a child of `<summary>`, right-aligned, and visible at
   all times (collapsed and expanded states).
3. The expanded accordion content (the `<code>` block with the command) still
   displays correctly below the summary row when open.
4. No existing config pane functionality is broken.
5. `cargo fmt`, `cargo clippy`, `cargo test` pass with zero warnings.

## Status
ready
