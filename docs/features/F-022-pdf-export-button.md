# F-022 — PDF Export Button

## Feature ID
F-022

## Title
In-page PDF export button with file picker

## User Value
Users can export the currently viewed rendered document as a PDF file, choosing
the save path and filename via the browser's native file picker. This removes
the need to use browser print-to-PDF and gives users a direct, labeled export
action in the reader chrome.

## Scope Details

### Button placement
- Add a new button to the header bar, positioned to the **left** of the
  existing config (⚙) button.
- The button should use a recognisable download/export icon (e.g. a downward
  arrow with a tray, or a document-with-arrow SVG).
- It should match the visual style and sizing of the existing sidebar toggle
  and config buttons.

### PDF generation
- Use the browser's built-in `window.print()` API with a print stylesheet
  scoped to `@media print` that hides the reader chrome (header bar, sidebar,
  config pane, all buttons) and renders only the letter content.
- The print dialogue that appears must have "Save as PDF" / "Print to PDF" as
  the default or obvious path — this is standard browser behaviour on all
  major platforms.

### File picker / save path
- Before triggering the print dialogue, use the
  [`window.showSaveFilePicker`](https://developer.mozilla.org/en-US/docs/Web/API/Window/showSaveFilePicker)
  API (File System Access API) to let the user choose a file name and path.
  - Suggested default file name: the current page title or the source `.md`
    filename (available from the rendered `<title>` tag).
  - Filter: `{ description: "PDF files", accept: { "application/pdf": [".pdf"] } }`
- If `showSaveFilePicker` is not available (unsupported browser), fall back
  gracefully to calling `window.print()` directly without the picker — the
  browser will handle the save location itself.
- After the user selects a path and confirms, trigger `window.print()`.
  The browser writes the PDF output to the chosen location.

### Print stylesheet
- Add `@media print` CSS rules (in `src/style.css` or as an inline `<style>`
  in the rendered page) that:
  - Hide: header bar (`.mark-header` or equivalent), sidebar, config pane,
    all buttons, any fixed/sticky UI elements.
  - Show: only the letter content area (`.mark-letter` or equivalent), with
    sensible print margins.
  - Preserve: code block backgrounds and syntax colouring if feasible (use
    `-webkit-print-color-adjust: exact`).

## Dependencies
- M-018 — header button layout (released v0.7.1); button must fit the
  established header chrome pattern

## Acceptance Criteria
1. A download/export icon button appears in the header bar, left of the ⚙
   button.
2. Clicking it invokes `showSaveFilePicker` (where supported) with a `.pdf`
   filter and a sensible default filename.
3. After path selection, `window.print()` is triggered.
4. The print output contains only the letter content — no header, sidebar,
   config pane, or buttons.
5. On browsers without `showSaveFilePicker`, `window.print()` is called
   directly (graceful fallback).
6. The button's visual style is consistent with existing header buttons.
7. No existing functionality (sidebar, config, theme, hotkeys) is broken.
8. `cargo fmt`, `cargo clippy`, `cargo test` pass with zero warnings.
9. Tests cover: PDF button present in rendered HTML, print stylesheet present
   (`@media print`).

## Priority
Medium

## Milestone
M-020 (new — to be created)

## Status
released
