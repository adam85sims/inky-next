# Inky Next Core Features (Snippets & File Explorer) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task. It will decide whether each batch should run in parallel or serial subagent mode and will pass only task-local context to each subagent. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement an expanded "Ink" snippets menu and a sidebar-based File Explorer with auto-include logic in Inky Next.

**Architecture:** Use Svelte stores for cross-component state management, Tauri commands for filesystem operations, and Monaco editor's API for code insertion. The UI will use a three-pane layout (Sidebar | Editor | Preview).

**Tech Stack:** Svelte, Tauri (Rust), Monaco Editor, Tailwind CSS.

---

### Task 1: State & Stores Initialization

**Files:**
- Modify: `inky-next/src/lib/stores.js`

- [ ] **Step 1: Add new stores to `lib/stores.js`**

```javascript
import { writable } from 'svelte/store';

/** @type {import('svelte/store').Writable<string>} */
export const editorContent = writable('Once upon a time...');
/** @type {import('svelte/store').Writable<any[]>} */
export const storyHistory = writable([]);
/** @type {import('svelte/store').Writable<any[]>} */
export const compilerErrors = writable([]);

const initialTheme = (typeof window !== 'undefined' && localStorage.getItem('theme')) || 'dark';
/** @type {import('svelte/store').Writable<string>} */
export const theme = writable(initialTheme);

// New stores
/** @type {import('svelte/store').Writable<string[]>} */
export const projectFiles = writable([]);
/** @type {import('svelte/store').Writable<string|null>} */
export const activeFilePath = writable(null);
/** @type {import('svelte/store').Writable<string|null>} */
export const mainInkPath = writable(null);
/** @type {import('svelte/store').Writable<boolean>} */
export const sidebarVisible = writable(true);

if (typeof window !== 'undefined') {
    theme.subscribe(value => {
        localStorage.setItem('theme', value);
    });
}
```

- [ ] **Step 2: Commit**

```bash
git add inky-next/src/lib/stores.js
git commit -m "feat: add stores for file management and sidebar"
```

---

### Task 2: Rust Backend Commands

**Files:**
- Modify: `inky-next/src-tauri/src/main.rs`

- [ ] **Step 1: Implement `list_project_files`, `create_file` commands in `main.rs`**

```rust
use std::path::PathBuf;

#[tauri::command]
async fn list_project_files(project_path: String) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    let dir = PathBuf::from(&project_path);
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("ink") {
                if let Some(p) = path.to_str() {
                    files.push(p.to_string());
                }
            }
        }
    }
    Ok(files)
}

#[tauri::command]
async fn create_file(path: String) -> Result<(), String> {
    if std::path::Path::new(&path).exists() {
        return Err("File already exists".to_string());
    }
    std::fs::write(&path, "").map_err(|e| e.to_string())?;
    Ok(())
}
```

- [ ] **Step 2: Register commands in `main` function**

```rust
        .invoke_handler(tauri::generate_handler![
            compile_ink,
            choose_path,
            open_file,
            save_file,
            list_project_files,
            create_file
        ])
```

- [ ] **Step 3: Commit**

```bash
git add inky-next/src-tauri/src/main.rs
git commit -m "feat: implement list_project_files and create_file Tauri commands"
```

---

### Task 3: Sidebar UI & Layout

**Files:**
- Create: `inky-next/src/lib/components/FileExplorer.svelte`
- Modify: `inky-next/src/routes/+page.svelte`

- [ ] **Step 1: Create `FileExplorer.svelte` with basic list**

```svelte
<script>
  import { projectFiles, activeFilePath, sidebarVisible } from '$lib/stores';
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  async function refreshFiles() {
    // For now, assume current dir for demo or pass actual path
    const files = await invoke('list_project_files', { projectPath: "./" });
    projectFiles.set(files);
  }

  onMount(refreshFiles);
</script>

<div class="h-full bg-slate-800 text-slate-300 p-2 flex flex-col border-r border-slate-700">
  <div class="flex justify-between items-center mb-4 px-2">
    <span class="text-xs font-bold uppercase tracking-wider">Files</span>
    <button on:click={refreshFiles} class="hover:text-white">↻</button>
  </div>
  <div class="flex-1 overflow-y-auto">
    {#each $projectFiles as file}
      <button 
        on:click={() => activeFilePath.set(file)}
        class="block w-full text-left px-2 py-1 text-sm hover:bg-slate-700 rounded {$activeFilePath === file ? 'bg-slate-700 text-white' : ''}"
      >
        {file.split('/').pop()}
      </button>
    {/each}
  </div>
</div>
```

- [ ] **Step 2: Update `+page.svelte` to include Sidebar**

```svelte
<script>
  import Editor from '$lib/components/Editor.svelte';
  import Preview from '$lib/components/Preview.svelte';
  import Toolbar from '$lib/components/Toolbar.svelte';
  import FileExplorer from '$lib/components/FileExplorer.svelte';
  import { sidebarVisible } from '$lib/stores';
  import '../app.css';
</script>

<div class="flex flex-col h-screen w-screen overflow-hidden">
  <Toolbar />
  <main class="flex flex-1 overflow-hidden">
    {#if $sidebarVisible}
      <div class="w-48 flex-shrink-0">
        <FileExplorer />
      </div>
    {if}
    <div class="flex-1 border-r border-slate-800">
      <Editor />
    </div>
    <div class="flex-1">
      <Preview />
    </div>
  </main>
</div>
```

- [ ] **Step 3: Commit**

```bash
git add inky-next/src/lib/components/FileExplorer.svelte inky-next/src/routes/+page.svelte
git commit -m "feat: add FileExplorer component and update main layout"
```

---

### Task 4: File Creation & Auto-Include

**Files:**
- Modify: `inky-next/src/lib/components/FileExplorer.svelte`

- [ ] **Step 1: Implement `createNewFile` with Auto-Include logic**

```javascript
  import { mainInkPath, editorContent } from '$lib/stores';
  
  async function createNewFile() {
    const name = prompt("Enter file name (e.g. story.ink):");
    if (!name) return;
    const path = `./${name}`;
    try {
      await invoke('create_file', { path });
      await refreshFiles();
      
      // Auto-include in main file
      if ($mainInkPath) {
        const includeLine = `INCLUDE ${name}\n`;
        // In a real app, we'd read/write the main file via Tauri
        // For now, if active is main, update editorContent
        if ($activeFilePath === $mainInkPath) {
          editorContent.update(c => includeLine + c);
        }
      }
    } catch (err) {
      alert(err);
    }
  }
```

- [ ] **Step 2: Add "+" button to UI**

```svelte
  <div class="flex justify-between items-center mb-4 px-2">
    <span class="text-xs font-bold uppercase tracking-wider">Files</span>
    <div class="flex gap-2">
      <button on:click={createNewFile} class="hover:text-white">+</button>
      <button on:click={refreshFiles} class="hover:text-white">↻</button>
    </div>
  </div>
```

- [ ] **Step 3: Commit**

```bash
git add inky-next/src/lib/components/FileExplorer.svelte
git commit -m "feat: implement file creation and auto-include logic"
```

---

### Task 5: Snippets Infrastructure

**Files:**
- Create: `inky-next/src/lib/snippets.js`
- Modify: `inky-next/src/lib/components/Editor.svelte`

- [ ] **Step 1: Create `lib/snippets.js`**

```javascript
export const snippets = [
  {
    category: "Structure",
    items: [
      { label: "Knot", content: "=== ${1:knot_name} ===\n${0}" },
      { label: "Stitch", content: "= ${1:stitch_name}\n${0}" },
      { label: "Include", content: "INCLUDE ${1:filename}.ink\n${0}" }
    ]
  },
  {
    category: "Content",
    items: [
      { label: "Choice", content: "* ${1:choice_text}\n    ${0}" },
      { label: "Sticky Choice", content: "+ ${1:choice_text}\n    ${0}" },
      { label: "Sequence", content: "{ ${1:item1} | ${2:item2} | ${3:item3} }${0}" }
    ]
  }
  // ... add more as needed
];
```

- [ ] **Step 2: Expose `insertSnippet` in `Editor.svelte` via window or custom event**

```javascript
  // In Editor.svelte <script>
  export function insertSnippet(template) {
    if (!editor) return;
    const contribution = editor.getContribution('snippetController2');
    if (contribution) {
      contribution.insert(template);
      editor.focus();
    }
  }
  
  // Expose to window for Toolbar to access (simple bridge)
  onMount(() => {
    window.insertInkSnippet = insertSnippet;
  });
```

- [ ] **Step 3: Commit**

```bash
git add inky-next/src/lib/snippets.js inky-next/src/lib/components/Editor.svelte
git commit -m "feat: add snippets data and editor insertion logic"
```

---

### Task 6: Snippets UI

**Files:**
- Create: `inky-next/src/lib/components/SnippetMenu.svelte`
- Modify: `inky-next/src/lib/components/Toolbar.svelte`

- [ ] **Step 1: Create `SnippetMenu.svelte`**

```svelte
<script>
  import { snippets } from '$lib/snippets';
  export let active = false;

  function insert(content) {
    if (window.insertInkSnippet) {
      window.insertInkSnippet(content);
    }
    active = false;
  }
</script>

{#if active}
  <div class="absolute top-full left-0 mt-1 w-48 bg-slate-800 border border-slate-700 rounded shadow-xl z-50 py-1">
    {#each snippets as cat}
      <div class="px-3 py-1 text-[10px] font-bold text-slate-500 uppercase border-b border-slate-700/50">{cat.category}</div>
      {#each cat.items as item}
        <button 
          on:click={() => insert(item.content)}
          class="w-full text-left px-4 py-1.5 text-sm hover:bg-slate-700 text-slate-200"
        >
          {item.label}
        </button>
      {/each}
    {/each}
  </div>
{/if}
```

- [ ] **Step 2: Add "Ink" button and toggle sidebar in `Toolbar.svelte`**

```svelte
<script>
  import { sidebarVisible } from '$lib/stores';
  import SnippetMenu from './SnippetMenu.svelte';
  let snippetMenuActive = false;
</script>

<div class="flex items-center gap-4 bg-slate-950 p-2 border-b border-slate-800">
  <button on:click={() => $sidebarVisible = !$sidebarVisible} class="p-1 hover:bg-slate-800 rounded">
    <!-- Sidebar Toggle Icon -->
    <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20"><path d="M3 4h14v2H3V4zm0 5h14v2H3V9zm0 5h14v2H3v-2z"/></svg>
  </button>
  
  <div class="relative">
    <button on:click={() => snippetMenuActive = !snippetMenuActive} class="text-sm font-medium hover:text-white px-2 py-1">
      Ink
    </button>
    <SnippetMenu bind:active={snippetMenuActive} />
  </div>
  
  <h1 class="text-sm font-bold ml-auto pr-4">Inky Next</h1>
</div>
```

- [ ] **Step 3: Commit**

```bash
git add inky-next/src/lib/components/SnippetMenu.svelte inky-next/src/lib/components/Toolbar.svelte
git commit -m "feat: implement SnippetMenu UI and toolbar integration"
```

---

### Task 7: Final Polish & Verification

**Files:**
- Modify: `inky-next/src/lib/components/Editor.svelte`

- [ ] **Step 1: Ensure indentation is handled correctly**
Monaco's `snippetController2` handles basic indentation automatically if the template content has appropriate newlines and tabs.

- [ ] **Step 2: Verify AC-001 through AC-013**
- [ ] AC-001: Sidebar toggles.
- [ ] AC-006: Auto-include works.
- [ ] AC-010: Placeholders are selected.

- [ ] **Step 3: Commit**

```bash
git add inky-next/src/lib/components/Editor.svelte
git commit -m "fix: final polish for snippets and sidebar"
```
