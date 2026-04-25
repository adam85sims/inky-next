# Contracts: Inky Next Core

## Multi-File Synchronization Contract

- **State Owner:** Svelte stores in `lib/stores.js`.
- **Producer:** `FileExplorer.svelte` sets `activeFilePath` and `projectFiles`.
- **Consumer:** `Editor.svelte` subscribes to `activeFilePath`. When it changes, the editor clears the current content and calls the `open_file` Tauri command to load the new file content into the Monaco buffer.
- **Side Effect:** Any change to `editorContent` (synced with Monaco) triggers the `compile_ink` command via a 300ms debounce in `Editor.svelte`.

## Snippet Insertion Bridge

- **Contract:** The `Editor` component must expose a global function `window.insertInkSnippet(template)` upon mounting.
- **Interface:**
    - `template`: A string following Monaco's snippet syntax (e.g., `${1:placeholder}`).
- **Requirement:** `Editor.svelte` must ensure the Monaco instance is ready and focused before performing the insertion via `snippetController2`.

---
**Last Updated:** 2026-04-25
**Verified Commit:** d3c0446
