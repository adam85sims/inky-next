# Acceptance Criteria: Inky Next (Tauri Migration)

**Spec:** `docs/superpowers/specs/2026-04-24-inky-tauri-migration-design.md`
**Date:** 2026-04-24
**Status:** Draft

---

## Criteria

| ID | Description | Test Type | Preconditions | Expected Result |
|----|-------------|-----------|---------------|-----------------|
| AC-001 | Application launches and displays the main window without sandboxing errors or segfaults on Linux. | UI interaction | `npm run tauri dev` executed. | Main window appears with Editor and Preview panes. |
| AC-002 | The `inklecate` sidecar can be successfully executed by the Tauri backend. | API | Tauri project initialized with `inklecate_linux` in `src-tauri/binaries`. | Rust log confirms successful sidecar execution and version check. |
| AC-003 | Monaco Editor initializes with a dark theme by default. | UI interaction | App launched for the first time. | Editor pane background is dark (charcoal/slate palette). |
| AC-004 | Typing valid Ink script in Monaco displays syntax highlighting. | UI interaction | Monaco Editor loaded. | Keywords like `VAR`, `KNOT`, `==`, `->` are colored differently from plain text. |
| AC-005 | Debounced changes in the editor trigger a compilation event in the backend. | Logic | Text entered in Editor. | Tauri backend receives the `compile_ink` command after ~300ms of inactivity. |
| AC-006 | Story text from the compiler is rendered in the Preview pane. | UI interaction | Valid Ink script `Hello world` in Editor. | "Hello world" appears in the right-hand Preview pane. |
| AC-007 | Choices in the Ink script are rendered as interactive buttons in the Preview pane. | UI interaction | Ink script `* [Choice]` in Editor. | A button labeled "Choice" appears in the Preview pane. |
| AC-008 | Clicking a choice button in the Preview pane sends the choice number to the compiler and updates the story. | UI interaction | Story with choices rendered in Preview. | Clicking button triggers new story content to append to the Preview. |
| AC-009 | Invalid Ink syntax causes red squiggly error markers to appear in the Monaco editor. | UI interaction | Invalid Ink (e.g., `-> non_existent_knot`) in Editor. | Red markers appear on the line containing the error. |
| AC-010 | "Save" action writes the current editor content to a `.ink` file on the local file system. | UI interaction | Content in Editor, "Save" clicked. | File is created/updated at the selected path with exact editor content. |
| AC-011 | "Open" action loads content from a `.ink` file into the editor. | UI interaction | Valid `.ink` file exists on disk. | Selecting file in open dialog populates Monaco Editor with file content. |
| AC-012 | High-contrast dark theme is used by default and persists across app restarts. | UI interaction | Theme set to dark (default). | App closes and reopens with dark theme still active. |
