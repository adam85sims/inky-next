# Design Doc: Inky Next (Tauri Migration)

**Date:** 2026-04-24
**Topic:** Migrating Inky from Electron to Tauri + SvelteKit + Monaco
**Status:** Draft

## 1. Overview
Inky is the editor for the ink narrative scripting language. The goal of this project is to migrate the application from its current Electron-based architecture to a modern, stable stack using Tauri (Rust backend) and SvelteKit (Frontend). This migration specifically targets resolving long-standing stability issues on Linux (e.g., full-screen crashes and segfaults) while modernizing the development experience.

## 2. Goals
- **Stability:** Eliminate Electron-related crashes on Linux.
- **Performance:** Reduce resource footprint using Tauri's lightweight architecture.
- **Modernization:** Replace jQuery/Ace with SvelteKit and Monaco Editor.
- **Dark Mode First:** Default the UI to a high-quality dark theme.
- **Platform Parity:** Ensure the `inklecate` compiler works seamlessly as a sidecar across Linux, macOS, and Windows.

## 3. Architecture

### Backend (Rust / Tauri)
- **Tauri Commands:**
    - `compile_ink`: Receives string content, writes to a temporary file, and executes the `inklecate` sidecar.
    - `save_file` / `open_file`: Native file system operations for `.ink` projects.
- **Sidecar Integration:**
    - Bundles `inklecate_linux`, `inklecate_mac`, and `inklecate_win.exe`.
    - Uses Tauri's `Command` API to manage the compiler process.
- **Event System:**
    - Emits `ink-output` events containing parsed JSON from `inklecate` stdout.
    - Emits `ink-error` for compiler crashes or unexpected stderr.

### Frontend (SvelteKit / Monaco)
- **Framework:** SvelteKit (Static Site Generation mode for Tauri).
- **Editor:** Monaco Editor.
- **State Management:**
    - `editorStore`: Tracks current file content and undo/redo state.
    - `storyStore`: Tracks the linear history of text and choices for the preview.
    - `settingsStore`: Tracks theme (Dark/Light) and persistence.
- **Preview Engine:**
    - Renders a reactive list of "Story Cards" (text paragraphs, choice buttons, tags).
    - Auto-scrolls to the latest content.

## 4. UI/UX Design
- **Default Theme:** Dark mode using a charcoal/slate palette.
- **Layout:**
    - Two main panes with a draggable splitter.
    - **Left Pane:** Monaco Editor with Ink syntax highlighting.
    - **Right Pane:** "Player View" for real-time preview.
    - **Top Bar:** File operations, Export options, and Theme toggle.

## 5. Implementation Phases
1. **Foundation:** Initialize Tauri + SvelteKit project and configure `inklecate` sidecar.
2. **Editor Core:** Integrate Monaco with a basic Ink syntax highlighter.
3. **Bridge:** Implement the Rust commands for compilation and event streaming.
4. **Preview UI:** Build the reactive story history renderer in Svelte.
5. **Polishing:** Add dark mode styling, file save/load, and error markers in the editor.

## 6. Success Criteria
- The app launches and functions on Linux without `--no-sandbox` or segfaults.
- Writing in the left pane updates the right pane preview within <500ms.
- Compiling an invalid Ink script highlights errors directly in the Monaco editor.
- The binary size is significantly smaller than the original Electron app.
