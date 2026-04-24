# Inky Next Migration Fixes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task. It will decide whether each batch should run in parallel or serial subagent mode and will pass only task-local context to each subagent. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Address critical and important issues identified during code review to ensure functional parity with the original Inky.

**Architecture:** Refactor the backend to support session-based interactive compilation and implement file I/O commands. Enhance the frontend with proper preview management and Ink syntax support.

---

### Task 1: Backend - Unique Temp Files & Error Handling

**Files:**
- Modify: `inky-next/src-tauri/src/main.rs`

- [ ] **Step 1: Replace `.expect()` with `Result` and use unique temp files**
```rust
use tauri::api::process::Command;
use tauri::api::process::CommandEvent;
use uuid::Uuid;

#[tauri::command]
async fn compile_ink(window: tauri::Window, content: String) -> Result<(), String> {
    let session_id = Uuid::new_v4().to_string();
    let temp_dir = std::env::temp_dir().join("inky_next").join(&session_id);
    std::fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;
    let temp_path = temp_dir.join("main.ink");
    std::fs::write(&temp_path, content).map_err(|e| e.to_string())?;

    // ... rest of logic with better error handling ...
    Ok(())
}
```

- [ ] **Step 2: Commit**
```bash
git add inky-next/src-tauri/src/main.rs
git commit -m "fix: use unique temp files and better error handling"
```

---

### Task 2: Backend - Interactive Play Mode Support

**Files:**
- Modify: `inky-next/src-tauri/src/main.rs`

- [ ] **Step 1: Update sidecar call to include `-p` flag and handle state**
```rust
// In compile_ink command, update args:
.args(["-j", "-p", "-c", temp_path.to_str().unwrap()])
```

- [ ] **Step 2: Commit**
```bash
git add inky-next/src-tauri/src/main.rs
git commit -m "feat: enable play mode in sidecar"
```

---

### Task 3: Frontend - Preview Logic Fix (No Duplication)

**Files:**
- Modify: `inky-next/src/lib/components/Editor.svelte`

- [ ] **Step 1: Clear `storyHistory` before each compilation**
```javascript
async function handleCompile(content) {
  storyHistory.set([]); // Clear existing history
  await invoke('compile_ink', { content });
}
```

- [ ] **Step 2: Commit**
```bash
git add inky-next/src/lib/components/Editor.svelte
git commit -m "fix: clear story history before re-compilation"
```

---

### Task 4: Backend - File I/O Commands

**Files:**
- Modify: `inky-next/src-tauri/src/main.rs`
- Modify: `inky-next/src-tauri/tauri.conf.json`

- [ ] **Step 1: Implement `open_file` and `save_file` commands**
```rust
#[tauri::command]
async fn open_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_file(path: String, content: String) -> Result<(), String> {
    std::fs::write(path, content).map_err(|e| e.to_string())
}
```

- [ ] **Step 2: Register commands in handler**
```rust
.invoke_handler(tauri::generate_handler![compile_ink, open_file, save_file])
```

- [ ] **Step 3: Commit**
```bash
git add inky-next/src-tauri/src/main.rs
git commit -m "feat: implement open and save file commands"
```

---

### Task 5: Frontend - Ink Syntax Highlighting

**Files:**
- Modify: `inky-next/src/lib/components/Editor.svelte`

- [ ] **Step 1: Add a basic Ink language definition to Monaco**
```javascript
monaco.languages.register({ id: 'ink' });
monaco.languages.setMonarchTokensProvider('ink', {
    tokenizer: {
        root: [
            [/VAR\s+/, "keyword"],
            [/CONST\s+/, "keyword"],
            [/LIST\s+/, "keyword"],
            [/KNOT\s+/, "keyword"],
            [/==+/, "keyword"],
            [/->/, "keyword"],
            [/<-/, "keyword"],
            [/\[/, "bracket"],
            [/\]/, "bracket"],
            [/\*| \+/, "keyword"],
            [/\/\/.*/, "comment"],
            [/\/\*[\s\S]*?\*\//, "comment"],
        ]
    }
});
// Update editor initialization to use 'ink'
```

- [ ] **Step 2: Commit**
```bash
git add inky-next/src/lib/components/Editor.svelte
git commit -m "feat: add basic ink syntax highlighting to monaco"
```
