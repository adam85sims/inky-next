# Design Doc: Inky Next Core Features (Snippets & File Explorer)

**Date:** 2026-04-25
**Topic:** Implementing Ink Snippets and a File Explorer in Inky Next.
**Status:** Approved

## 1. Overview
This design covers the addition of two core features to Inky Next: an expanded "Ink" snippets menu for rapid content creation and a sidebar-based File Explorer for managing multi-file projects with automatic `INCLUDE` management.

## 2. Goals
- **Productivity:** Provide a comprehensive set of Ink language templates.
- **Project Management:** Support multi-file projects with a visual file tree.
- **Ease of Use:** Automate the `INCLUDE` process for new project files.
- **Modern UI:** Maintain a clean, professional "IDE-like" layout.

## 3. Features

### 3.1 Ink Snippets (Toolbar Menu)
- **UI:** A dropdown menu in the top toolbar labeled "Ink".
- **Categories & Content:**
    - **Structure:** Knot (`=== ${name} ===`), Stitch (`= ${name}`), Include (`INCLUDE ${file}`).
    - **Content:** Choice (`* ${text}`), Sticky Choice (`+ ${text}`), Sequence (`{...|...}`), Cycle (`{&...|...}`), Shuffle (`{~...|...}`).
    - **Logic:** Global Variable (`VAR ${name} = ${val}`), Local Variable (`~ temp ${name} = ${val}`), Function (`=== function ${name} ===`), Tunnel (`-> ${target} ->`), Thread (`<- ${target}`).
    - **Advanced:** External Function (`EXTERNAL ${name}`), List (`LIST ${name} = ...`), Constant (`CONST ${name} = ${val}`).
- **Behavior:** Inserts the template at the cursor in Monaco. Auto-selects placeholders for immediate editing. Respects current line indentation.

### 3.2 File Explorer (Sidebar)
- **UI:** A collapsible left sidebar toggled via a toolbar button.
- **Functionality:**
    - Visual tree of `.ink` files in the current project directory.
    - Highlight active file.
    - Click to open/switch files in the editor.
    - "New File" button that:
        1. Creates a new `.ink` file on disk.
        2. Automatically appends `INCLUDE filename.ink` to the designated "Main Ink" file.
- **Backend (Tauri):** Rust commands for `list_project_files`, `create_file`, and `designate_main_ink`.

## 4. Architecture Updates

### State Management (`lib/stores.js`)
- `projectFiles`: Writable store tracking the current file tree.
- `activeFilePath`: Writable store for the currently edited file.
- `mainInkPath`: Writable store identifying the root project file.
- `sidebarVisible`: Boolean store for toggling the sidebar layout.

### Layout (`+page.svelte`)
- Updated to a three-column layout:
  ```
  [Sidebar (200px)] | [Editor (Flexible)] | [Preview (Flexible)]
  ```
- Sidebar visibility is reactive based on `sidebarVisible`.

## 5. Implementation Phases
1. **Sidebar Foundation:** Build the UI for the sidebar and the Tauri command to list files.
2. **File Operations:** Implement file switching and creation with "Auto-Include" logic.
3. **Snippets Menu:** Create the `SnippetMenu.svelte` component and integrate it into the `Toolbar`.
4. **Editor Integration:** Implement the `insertAtCursor` logic for Monaco.
5. **Polishing:** Add icons, keyboard shortcuts (toggle sidebar), and refinement for snippet indentation.

## 6. Success Criteria
- User can toggle a sidebar that shows all `.ink` files in the project directory.
- Creating a new file via the sidebar correctly adds an `INCLUDE` line to the main file.
- Selecting a snippet from the "Ink" menu inserts correctly formatted code at the editor's cursor.
- The editor respects indentation and placeholders within inserted snippets.
