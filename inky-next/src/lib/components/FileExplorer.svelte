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
