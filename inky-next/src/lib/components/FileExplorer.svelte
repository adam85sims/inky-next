<script>
  import { projectFiles, activeFilePath, sidebarVisible, mainInkPath, editorContent } from '$lib/stores';
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  async function refreshFiles() {
    // For now, assume current dir for demo or pass actual path
    const files = await invoke('list_project_files', { projectPath: "./" });
    projectFiles.set(files);
  }

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

  onMount(refreshFiles);
</script>

<div class="h-full bg-slate-800 text-slate-300 p-2 flex flex-col border-r border-slate-700">
  <div class="flex justify-between items-center mb-4 px-2">
    <span class="text-xs font-bold uppercase tracking-wider">Files</span>
    <div class="flex gap-2">
      <button on:click={createNewFile} class="hover:text-white" title="New File">+</button>
      <button on:click={refreshFiles} class="hover:text-white" title="Refresh">↻</button>
    </div>
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
