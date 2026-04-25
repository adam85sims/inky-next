# Module Cards: Inky Next Core

## Frontend Components (`inky-next/src/lib/components/`)

### `Editor.svelte`
- **Responsibility:** Main Monaco editor instance. Handles Ink language registration, syntax highlighting, and content persistence.
- **Key Methods:**
    - `insertSnippet(template)`: Uses Monaco's `snippetController2` to insert templates at cursor.
- **Stores:** `editorContent`, `activeFilePath`, `theme`.
- **Global Bridge:** Exposes `window.insertInkSnippet`.

### `FileExplorer.svelte`
- **Responsibility:** Sidebar file navigation and creation.
- **Actions:**
    - Lists `.ink` files in project root.
    - Creates new files with "Auto-Include" logic (appends `INCLUDE` to main file).
    - Switches active file in editor.
- **Stores:** `projectFiles`, `activeFilePath`, `mainInkPath`.

### `SnippetMenu.svelte`
- **Responsibility:** Categorized dropdown for Ink code snippets.
- **Categories:** Structure, Content, Logic.
- **Integration:** Calls `window.insertInkSnippet` to trigger insertion in Monaco.

## Backend Commands (`inky-next/src-tauri/src/main.rs`)

### `list_project_files`
- **Input:** `project_path: String`
- **Output:** `Vec<String>` (paths to all `.ink` files).

### `create_file`
- **Input:** `path: String`
- **Behavior:** Checks for existence before writing an empty string. Returns error if file exists.

---
**Last Updated:** 2026-04-25
**Verified Commit:** d3c0446
