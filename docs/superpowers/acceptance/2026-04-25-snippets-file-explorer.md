# Acceptance Criteria: Inky Next Core Features (Snippets & File Explorer)

**Spec:** `docs/superpowers/specs/2026-04-25-inky-next-core-features-design.md`
**Date:** 2026-04-25
**Status:** Draft

---

## Criteria

| ID | Description | Test Type | Preconditions | Expected Result |
|----|-------------|-----------|---------------|-----------------|
| AC-001 | Sidebar Toggling | UI interaction | App is open | Clicking the sidebar toggle button in the toolbar shows/hides the File Explorer pane. |
| AC-002 | File Tree Listing | UI interaction | Project with multiple `.ink` files is open | The sidebar displays a list of all `.ink` files in the current project directory. |
| AC-003 | File Switching | UI interaction | Multiple files in sidebar | Clicking a file name in the sidebar switches the Monaco editor content to that file. |
| AC-004 | Highlight Active File | UI interaction | A file is open in editor | The file currently being edited is visually highlighted in the sidebar file tree. |
| AC-005 | New File Creation | UI interaction | Sidebar is visible | Clicking "New File" in the sidebar creates a new `.ink` file on the disk in the project directory. |
| AC-006 | Auto-Include Logic | UI interaction | "Main Ink" file is designated | Creating a new file `new.ink` automatically adds `INCLUDE new.ink` to the designated main ink file. |
| AC-007 | Snippet Menu Visibility | UI interaction | App is open | Clicking the "Ink" button in the toolbar opens a categorized dropdown menu of snippets. |
| AC-008 | Snippet Insertion | UI interaction | Monaco editor is focused | Selecting a snippet (e.g., "Knot") inserts the corresponding template (e.g., `=== name ===`) at the cursor position. |
| AC-009 | Indentation Respect | UI interaction | Cursor is indented | Selecting a snippet on an indented line preserves the current indentation level for the inserted template. |
| AC-010 | Placeholder Selection | UI interaction | Snippet with placeholder inserted | After snippet insertion, any placeholder (like `${name}`) is automatically selected for immediate overwrite. |
| AC-011 | Duplicate File Check | UI interaction | File `test.ink` already exists | Attempting to create `test.ink` via the sidebar shows an error message and does not overwrite the existing file. |
| AC-012 | Rust Command: List Files | API | Project path exists | The `list_project_files` Tauri command returns a JSON array of `.ink` file paths. |
| AC-013 | Rust Command: Create File | API | Valid path provided | The `create_file` Tauri command successfully writes a new empty file to the filesystem. |
