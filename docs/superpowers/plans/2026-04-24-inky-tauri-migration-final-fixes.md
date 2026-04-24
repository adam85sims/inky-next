# Inky Next Migration Final Fixes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task. It will decide whether each batch should run in parallel or serial subagent mode and will pass only task-local context to each subagent. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix the JSON output flag, editor sync, and add theme persistence.

---

### Task 1: Backend - Fix Sidecar Flags

**Files:**
- Modify: `inky-next/src-tauri/src/main.rs`

- [ ] **Step 1: Add `-j` flag back to interactive sidecar call**
```rust
.args(["-i", "-j", temp_path.to_str().unwrap()])
```

- [ ] **Step 2: Commit**
```bash
git add inky-next/src-tauri/src/main.rs
git commit -m "fix: restore json flag to sidecar"
```

---

### Task 2: Frontend - Editor Sync & Theme Persistence

**Files:**
- Modify: `inky-next/src/lib/components/Editor.svelte`
- Modify: `inky-next/src/lib/stores.js`

- [ ] **Step 1: Subscribe to `editorContent` store in Monaco**
```javascript
editorContent.subscribe(value => {
  if (editor && value !== editor.getValue()) {
    editor.setValue(value);
  }
});
```

- [ ] **Step 2: Implement Theme Persistence in `stores.js`**
```javascript
const savedTheme = typeof localStorage !== 'undefined' ? localStorage.getItem('theme') || 'dark' : 'dark';
export const theme = writable(savedTheme);
theme.subscribe(v => {
  if (typeof localStorage !== 'undefined') localStorage.setItem('theme', v);
});
```

- [ ] **Step 3: Commit**
```bash
git add inky-next/src/lib/components/Editor.svelte inky-next/src/lib/stores.js
git commit -m "fix: sync editor content and add theme persistence"
```
