# M-012 — Render Memory

## Milestone ID
M-012

## Title
Render Memory

## Objective
`mark` remembers previously rendered files and prompts the user before redundantly re-rendering unchanged source files.

## Included Features
- F-013 — Render memory and re-render confirmation

## Dependencies
- M-011 must be merged before M-012 work begins (overlapping `main.rs` changes)

## Acceptance Criteria
- Re-running `mark` on an unchanged file prompts the user.
- Cache is respected, updated, and cleared correctly.
- All checks pass.
- README updated to document the re-render prompt.

## Priority
Medium

## Status
blocked — waiting on M-011

## Target Release
v0.4.0 (minor)
