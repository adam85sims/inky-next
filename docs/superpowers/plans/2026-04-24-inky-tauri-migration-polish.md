# Inky Next Migration Final Polish Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task. It will decide whether each batch should run in parallel or serial subagent mode and will pass only task-local context to each subagent. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Complete the interactive loop (choices), add File I/O UI, and ensure proper resource management.

**Architecture:** Use Tauri's `process` management to kill stale sidecars. Implement a new Tauri command for sending choices to stdin. Add a simple toolbar for file actions.

---

### Task 1: Backend - Process Management & Choice Support

**Files:**
- Modify: `inky-next/src-tauri/src/main.rs`

- [ ] **Step 1: Track the active sidecar child and implement `choose_path` command**
```rust
use std::sync::Mutex;
use tauri::api::process::CommandChild;

struct AppState {
    active_process: Mutex<Option<CommandChild>>,
}

#[tauri::command]
async fn choose_path(state: tauri::State<'_, AppState>, choice_index: usize) -> Result<(), String> {
    let mut lock = state.active_process.lock().unwrap();
    if let Some(child) = lock.as_mut() {
        child.write(format!("{}\n", choice_index).as_bytes()).map_err(|e| e.to_string())?;
    }
    Ok(())
}
```

- [ ] **Step 2: Kill existing process before starting new compilation**
```rust
// In compile_ink:
let mut lock = state.active_process.lock().unwrap();
if let Some(child) = lock.take() {
    child.kill().ok();
}
// ... after spawn ...
*lock = Some(child);
```

- [ ] **Step 3: Commit**
```bash
git add inky-next/src-tauri/src/main.rs
git commit -m "feat: implement choice interaction and process management"
```

---

### Task 2: Frontend - Choice Interaction

**Files:**
- Modify: `inky-next/src/lib/components/Preview.svelte`

- [ ] **Step 1: Add click handler to choice buttons**
```svelte
<script>
  import { invoke } from '@tauri-apps/api/tauri';
  async function makeChoice(index) {
    await invoke('choose_path', { choiceIndex: index });
  }
</script>

<!-- In template -->
<button on:click={() => makeChoice(item.index)}>...</button>
```

- [ ] **Step 2: Commit**
```bash
git add inky-next/src/lib/components/Preview.svelte
git commit -m "feat: enable interactive choices in preview"
```

---

### Task 3: UI - Toolbar & File Actions

**Files:**
- Create: `inky-next/src/lib/components/Toolbar.svelte`
- Modify: `inky-next/src/routes/+page.svelte`

- [ ] **Step 1: Implement Toolbar with Open/Save**
```svelte
<script>
  import { open, save } from '@tauri-apps/api/dialog';
  import { invoke } from '@tauri-apps/api/tauri';
  import { editorContent } from '$lib/stores';

  async function handleOpen() {
    const selected = await open({ filters: [{ name: 'Ink', extensions: ['ink'] }] });
    if (selected) {
      const content = await invoke('open_file', { path: selected });
      editorContent.set(content);
    }
  }

  async function handleSave() {
    const path = await save({ filters: [{ name: 'Ink', extensions: ['ink'] }] });
    if (path) {
      await invoke('save_file', { path, content: $editorContent });
    }
  }
</script>

<div class="flex gap-2 p-2 bg-slate-900 border-b border-slate-800">
  <button on:click={handleOpen} class="px-3 py-1 bg-slate-800 hover:bg-slate-700 rounded">Open</button>
  <button on:click={handleSave} class="px-3 py-1 bg-slate-800 hover:bg-slate-700 rounded">Save</button>
</div>
```

- [ ] **Step 2: Commit**
```bash
git add inky-next/src/lib/components/Toolbar.svelte inky-next/src/routes/+page.svelte
git commit -m "feat: add toolbar with file actions"
```

---

### Task 4: Backend - Cleanup (Temp Files)

**Files:**
- Modify: `inky-next/src-tauri/src/main.rs`

- [ ] **Step 1: Add cleanup logic to remove temp directory on exit**
```rust
// Use a shutdown hook or simple cleanup in main.rs
// For now, let's at least ensure we don't leak thousands of files by cleaning up the root temp dir on start.
fn cleanup_temp() {
    let temp_dir = std::env::temp_dir().join("inky_next");
    if temp_dir.exists() {
        std::fs::remove_dir_all(temp_dir).ok();
    }
}
```

- [ ] **Step 2: Commit**
```bash
git add inky-next/src-tauri/src/main.rs
git commit -m "fix: cleanup temp directories on startup"
```
