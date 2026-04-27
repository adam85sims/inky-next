<script>
  import { projectFiles, activeFilePath, sidebarVisible, mainInkPath, editorContent, projectRoot } from '$lib/stores';
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  let isCreating = $state(false);
  let newFileName = $state("");
  
  // Context menu state
  let menuVisible = $state(false);
  let menuPos = $state({ x: 0, y: 0 });
  let menuFile = $state("");

  async function refreshFiles() {
    if (!$projectRoot) {
      const tempDir = await invoke('get_temp_project_dir');
      projectRoot.set(tempDir);
      
      // Create initial main.ink
      const mainPath = `${tempDir}/main.ink`;
      try {
        await invoke('create_file', { path: mainPath });
        await invoke('save_file', { path: mainPath, content: "Once upon a time..." });
        mainInkPath.set(mainPath);
        activeFilePath.set(mainPath);
      } catch (err) {
        console.error("Failed to create initial file:", err);
      }
    }

    /** @type {string[]} */
    const files = await invoke('list_project_files', { projectPath: $projectRoot });
    projectFiles.set(files);

    // Auto-set main file if none set
    if (!$mainInkPath && files.length > 0) {
      // Prefer main.ink if it exists
      const main = files.find(f => f.endsWith('main.ink')) || files[0];
      mainInkPath.set(main);
    }
  }

  async function createNewFile() {
    if (!newFileName) {
      isCreating = false;
      return;
    }
    const name = newFileName.endsWith('.ink') ? newFileName : `${newFileName}.ink`;
    const path = `${$projectRoot}/${name}`;
    try {
      await invoke('create_file', { path });
      await refreshFiles();
      
      // Auto-set as active if it's the only file or if we want to switch
      activeFilePath.set(path);

      // Auto-include in main file
      if ($mainInkPath && path !== $mainInkPath) {
        const includeLine = `INCLUDE ${name}\n`;
        // In a real app, we'd read/write the main file via Tauri
        // For now, if active is main, update editorContent
        if ($activeFilePath === $mainInkPath) {
          editorContent.update(c => includeLine + c);
        } else {
          // If not active, we should still append to the main file on disk
          const currentMainContent = await invoke('open_file', { path: $mainInkPath });
          await invoke('save_file', { 
            path: $mainInkPath, 
            content: includeLine + currentMainContent 
          });
        }
      } else if (!$mainInkPath) {
        mainInkPath.set(path);
      }

      isCreating = false;
      newFileName = "";
    } catch (err) {
      alert(err);
    }
  }

  function cancelCreation() {
    isCreating = false;
    newFileName = "";
  }

  /** @param {KeyboardEvent} event */
  function handleKeyDown(event) {
    if (event.key === "Enter") {
      createNewFile();
    } else if (event.key === "Escape") {
      cancelCreation();
    }
  }

  /** @param {HTMLElement} node */
  function autofocus(node) {
    node.focus();
  }

  /**
   * @param {MouseEvent} e
   * @param {string} file
   */
  function handleContextMenu(e, file) {
    e.preventDefault();
    menuPos = { x: e.clientX, y: e.clientY };
    menuFile = file;
    menuVisible = true;
  }

  async function deleteFile() {
    if (!menuFile) return;
    const fileName = menuFile.split('/').pop();
    if (!confirm(`Are you sure you want to delete ${fileName}?`)) {
      menuVisible = false;
      return;
    }

    try {
      await invoke('delete_file', { path: menuFile });
      
      if ($activeFilePath === menuFile) {
        activeFilePath.set(null);
      }
      if ($mainInkPath === menuFile) {
        mainInkPath.set(null);
      }
      
      await refreshFiles();
    } catch (err) {
      alert(err);
    } finally {
      menuVisible = false;
    }
  }

  onMount(() => {
    refreshFiles();
    const closeMenu = () => menuVisible = false;
    window.addEventListener('click', closeMenu);
    return () => window.removeEventListener('click', closeMenu);
  });
</script>

<div class="h-full bg-slate-800 text-slate-300 p-2 flex flex-col border-r border-slate-700 relative">
  <div class="flex justify-between items-center mb-4 px-2">
    <span class="text-xs font-bold uppercase tracking-wider">Files</span>
    <div class="flex gap-2">
      <button onclick={() => isCreating = true} class="hover:text-white" title="New File">+</button>
      <button onclick={refreshFiles} class="hover:text-white" title="Refresh">↻</button>
    </div>
  </div>
  <div class="flex-1 overflow-y-auto">
    {#if isCreating}
      <div class="px-2 mb-2">
        <input
          type="text"
          bind:value={newFileName}
          onkeydown={handleKeyDown}
          onblur={cancelCreation}
          use:autofocus
          placeholder="filename.ink"
          class="w-full bg-slate-900 border border-slate-700 text-white text-sm px-2 py-1 rounded outline-none focus:border-blue-500"
        />
      </div>
    {/if}
    {#each $projectFiles as file}
      <button 
        onclick={() => activeFilePath.set(file)}
        oncontextmenu={(e) => handleContextMenu(e, file)}
        class="block w-full text-left px-2 py-1 text-sm hover:bg-slate-700 rounded {$activeFilePath === file ? 'bg-slate-700 text-white' : ''}"
      >
        {file.split('/').pop()}
      </button>
    {/each}
  </div>

  {#if menuVisible}
    <div 
      class="fixed z-50 bg-slate-800 border border-slate-700 rounded shadow-xl py-1 w-32"
      style="top: {menuPos.y}px; left: {menuPos.x}px"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.key === 'Escape' && (menuVisible = false)}
      role="menu"
      tabindex="-1"
    >
      <button 
        onclick={deleteFile}
        class="w-full text-left px-4 py-1.5 text-sm hover:bg-red-900/50 hover:text-red-200 text-slate-200"
      >
        Delete
      </button>
    </div>
  {/if}
</div>
