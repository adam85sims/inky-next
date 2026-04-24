# Inky Next (Tauri Migration) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task. It will decide whether each batch should run in parallel or serial subagent mode and will pass only task-local context to each subagent. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Migrate Inky from Electron to a stable Tauri + SvelteKit + Monaco stack with a dark-first UI.

**Architecture:** A Rust-based Tauri backend managing the `inklecate` sidecar and file I/O, paired with a SvelteKit frontend using Monaco Editor for code input and a reactive story previewer.

**Tech Stack:** Tauri (Rust), SvelteKit, Monaco Editor, TailwindCSS (for layout).

---

### Task 1: Foundation - Project Initialization

**Files:**
- Create: `inky-next/package.json`
- Create: `inky-next/src-tauri/Cargo.toml`
- Create: `inky-next/src-tauri/tauri.conf.json`

- [ ] **Step 1: Create the Tauri + SvelteKit skeleton**
Run: `npx create-tauri-app inky-next --template svelte --package-manager npm`
(Select SvelteKit if prompted)

- [ ] **Step 2: Configure SvelteKit for Static Site Generation (SSG)**
Modify `inky-next/svelte.config.js`:
```javascript
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter({
			fallback: 'index.html'
		})
	}
};
export default config;
```

- [ ] **Step 3: Run the initial dev build to verify setup**
Run: `cd inky-next && npm run tauri dev`
Expected: App launches with default Tauri/Svelte screen.

- [ ] **Step 4: Commit**
```bash
git add inky-next
git commit -m "chore: initialize tauri sveltekit project"
```

---

### Task 2: Sidecar Configuration

**Files:**
- Create: `inky-next/src-tauri/binaries/inklecate_linux`
- Modify: `inky-next/src-tauri/tauri.conf.json`

- [ ] **Step 1: Copy existing inklecate binaries to the new project**
Run: 
```bash
mkdir -p inky-next/src-tauri/binaries
cp app/main-process/ink/inklecate_linux inky-next/src-tauri/binaries/inklecate-x86_64-unknown-linux-gnu
chmod +x inky-next/src-tauri/binaries/inklecate-x86_64-unknown-linux-gnu
```

- [ ] **Step 2: Register the sidecar in tauri.conf.json**
Add to `bundle` section:
```json
"externalBin": [
  "binaries/inklecate"
]
```

- [ ] **Step 3: Test sidecar execution from Rust**
Add a temporary test command in `src-tauri/src/main.rs`:
```rust
#[tauri::command]
fn test_sidecar() -> String {
    let output = tauri::api::process::Command::new_sidecar("inklecate")
        .expect("failed to setup sidecar")
        .args(["--help"])
        .output()
        .expect("failed to run sidecar");
    output.stdout
}
```

- [ ] **Step 4: Commit**
```bash
git add inky-next/src-tauri/binaries inky-next/src-tauri/tauri.conf.json
git commit -m "feat: configure inklecate sidecar"
```

---

### Task 3: Backend - Compilation Command

**Files:**
- Modify: `inky-next/src-tauri/src/main.rs`

- [ ] **Step 1: Implement `compile_ink` command**
```rust
use tauri::api::process::Command;
use tauri::api::process::CommandEvent;

#[tauri::command]
async fn compile_ink(window: tauri::Window, content: String) -> Result<(), String> {
    // Write content to temp file
    let temp_path = std::env::temp_dir().join("compile.ink");
    std::fs::write(&temp_path, content).map_err(|e| e.to_string())?;

    let (mut rx, _child) = Command::new_sidecar("inklecate")
        .expect("failed to setup sidecar")
        .args(["-j", "-c", temp_path.to_str().unwrap()])
        .spawn()
        .map_err(|e| e.to_string())?;

    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line) = event {
                window.emit("ink-output", line).unwrap();
            }
        }
    });

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compile_ink])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 2: Commit**
```bash
git add inky-next/src-tauri/src/main.rs
git commit -m "feat: implement compile_ink tauri command"
```

---

### Task 4: Frontend - Svelte Stores

**Files:**
- Create: `inky-next/src/lib/stores.js`

- [ ] **Step 1: Create reactive stores for editor and story state**
```javascript
import { writable } from 'svelte/store';

export const editorContent = writable('Once upon a time...');
export const storyHistory = writable([]);
export const compilerErrors = writable([]);
export const theme = writable('dark');
```

- [ ] **Step 2: Commit**
```bash
git add inky-next/src/lib/stores.js
git commit -m "feat: add svelte stores for state management"
```

---

### Task 5: Frontend - Monaco Editor Integration

**Files:**
- Create: `inky-next/src/lib/components/Editor.svelte`
- Modify: `inky-next/src/routes/+page.svelte`

- [ ] **Step 1: Install Monaco**
Run: `npm install monaco-editor`

- [ ] **Step 2: Create Monaco Component with Dark Theme**
```svelte
<script>
  import { onMount } from 'svelte';
  import * as monaco from 'monaco-editor';
  import { editorContent } from '$lib/stores';

  let container;
  let editor;

  onMount(() => {
    editor = monaco.editor.create(container, {
      value: $editorContent,
      language: 'markdown', // We'll refine this to 'ink' later
      theme: 'vs-dark',
      automaticLayout: true,
    });

    editor.onDidChangeModelContent(() => {
      editorContent.set(editor.getValue());
    });
  });
</script>

<div bind:this={container} class="h-full w-full"></div>
```

- [ ] **Step 3: Commit**
```bash
git add inky-next/src/lib/components/Editor.svelte
git commit -m "feat: integrate monaco editor"
```

---

### Task 6: Frontend - Preview UI

**Files:**
- Create: `inky-next/src/lib/components/Preview.svelte`

- [ ] **Step 1: Implement reactive preview list**
```svelte
<script>
  import { storyHistory } from '$lib/stores';
  import { afterUpdate } from 'svelte';

  let scrollContainer;

  afterUpdate(() => {
    scrollContainer.scrollTo(0, scrollContainer.scrollHeight);
  });
</script>

<div bind:this={scrollContainer} class="h-full overflow-y-auto p-4 bg-slate-900 text-slate-200">
  {#each $storyHistory as item}
    <div class="mb-4">
      {#if item.type === 'text'}
        <p>{item.content}</p>
      {:else if item.type === 'choice'}
        <button class="block w-full text-left p-2 border border-slate-700 hover:bg-slate-800 rounded">
          {item.content}
        </button>
      {/if}
    </div>
  {/each}
</div>
```

- [ ] **Step 2: Commit**
```bash
git add inky-next/src/lib/components/Preview.svelte
git commit -m "feat: implement preview ui"
```

---

### Task 7: Layout & Dark Mode Polishing

**Files:**
- Modify: `inky-next/src/routes/+page.svelte`
- Create: `inky-next/src/app.css`

- [ ] **Step 1: Define base dark styles**
```css
@tailwind base;
@tailwind components;
@tailwind utilities;

body {
  @apply bg-slate-950 text-slate-100 overflow-hidden;
}
```

- [ ] **Step 2: Assemble Main Layout**
```svelte
<script>
  import Editor from '$lib/components/Editor.svelte';
  import Preview from '$lib/components/Preview.svelte';
  import '../app.css';
</script>

<main class="flex h-screen w-screen overflow-hidden">
  <div class="flex-1 border-r border-slate-800">
    <Editor />
  </div>
  <div class="flex-1">
    <Preview />
  </div>
</main>
```

- [ ] **Step 3: Commit**
```bash
git add inky-next/src/routes/+page.svelte inky-next/src/app.css
git commit -m "feat: assemble main layout with dark theme"
```

---

### Task 8: Bridge - Real-time Compilation

**Files:**
- Modify: `inky-next/src/lib/components/Editor.svelte`

- [ ] **Step 1: Connect editor changes to Tauri backend**
```javascript
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { storyHistory } from '$lib/stores';

let debounceTimer;

async function handleCompile(content) {
  await invoke('compile_ink', { content });
}

// Inside onMount or editor listener:
clearTimeout(debounceTimer);
debounceTimer = setTimeout(() => handleCompile(currentValue), 300);

// Listen for output
listen('ink-output', (event) => {
  const data = JSON.parse(event.payload);
  if (data.text) {
     storyHistory.update(h => [...h, { type: 'text', content: data.text }]);
  }
});
```

- [ ] **Step 2: Commit**
```bash
git add inky-next/src/lib/components/Editor.svelte
git commit -m "feat: connect editor to backend compilation"
```

---

### Task 9: Final Polish - Error Markers

**Files:**
- Modify: `inky-next/src/lib/components/Editor.svelte`

- [ ] **Step 1: Render compiler errors as Monaco decorations**
```javascript
listen('ink-output', (event) => {
  const data = JSON.parse(event.payload);
  if (data.issues) {
    const markers = data.issues.map(issue => ({
      message: issue.message,
      severity: monaco.MarkerSeverity.Error,
      startLineNumber: issue.lineNumber,
      endLineNumber: issue.lineNumber,
      startColumn: 1,
      endColumn: 100
    }));
    monaco.editor.setModelMarkers(editor.getModel(), 'ink', markers);
  }
});
```

- [ ] **Step 2: Commit**
```bash
git add inky-next/src/lib/components/Editor.svelte
git commit -m "feat: add error markers to editor"
```
