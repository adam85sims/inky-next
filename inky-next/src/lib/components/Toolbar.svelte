<script>
  import { open, save } from '@tauri-apps/api/dialog';
  import { invoke } from '@tauri-apps/api/tauri';
  import { editorContent, theme, sidebarVisible, projectRoot, activeFilePath, mainInkPath } from '$lib/stores';
  import SnippetMenu from './SnippetMenu.svelte';

  let snippetMenuVisible = false;

  async function handleOpen() {
    try {
      const selected = await open({ 
        filters: [{ name: 'Ink', extensions: ['ink'] }] 
      });
      if (selected && typeof selected === 'string') {
        const content = await invoke('open_file', { path: selected });
        editorContent.set(content);
        activeFilePath.set(selected);
        
        // Set project root to parent directory
        const parentDir = selected.substring(0, selected.lastIndexOf('/'));
        projectRoot.set(parentDir);
        mainInkPath.set(selected); // Assume opened file is main for now
      }
    } catch (err) {
      console.error('Failed to open file:', err);
    }
  }

  async function handleSave() {
    if ($activeFilePath) {
      try {
        await invoke('save_file', { path: $activeFilePath, content: $editorContent });
      } catch (err) {
        console.error('Failed to save file:', err);
      }
    } else {
      handleSaveAs();
    }
  }

  async function handleSaveAs() {
    try {
      const path = await save({ 
        filters: [{ name: 'Ink', extensions: ['ink'] }] 
      });
      if (path) {
        await invoke('save_file', { path, content: $editorContent });
        activeFilePath.set(path);
      }
    } catch (err) {
      console.error('Failed to save file:', err);
    }
  }

  async function handleSaveProject() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select Project Directory'
      });

      if (selected && typeof selected === 'string' && $projectRoot) {
        await invoke('save_project', { 
          sourcePath: $projectRoot, 
          targetPath: selected 
        });
        projectRoot.set(selected);
        // Update paths to new location
        if ($activeFilePath) {
          const fileName = $activeFilePath.split('/').pop();
          activeFilePath.set(`${selected}/${fileName}`);
        }
        if ($mainInkPath) {
          const fileName = $mainInkPath.split('/').pop();
          mainInkPath.set(`${selected}/${fileName}`);
        }
        alert('Project saved successfully!');
      }
    } catch (err) {
      alert(`Failed to save project: ${err}`);
    }
  }

  function toggleTheme() {
    theme.update(t => t === 'dark' ? 'light' : 'dark');
  }
</script>

<div class="flex gap-2 p-2 bg-slate-900 border-b border-slate-800">
  <button 
    on:click={() => $sidebarVisible = !$sidebarVisible}
    class="px-3 py-1 bg-slate-800 hover:bg-slate-700 rounded text-slate-200 text-sm transition-colors"
  >
    {$sidebarVisible ? 'Hide Sidebar' : 'Show Sidebar'}
  </button>

  <div class="relative">
    <button 
      on:click={() => snippetMenuVisible = !snippetMenuVisible}
      class="px-3 py-1 bg-slate-800 hover:bg-slate-700 rounded text-slate-200 text-sm transition-colors"
    >
      Ink
    </button>
    <SnippetMenu bind:active={snippetMenuVisible} />
  </div>

  <div class="w-px bg-slate-800 mx-1"></div>

  <button 
    on:click={handleOpen} 
    class="px-3 py-1 bg-slate-800 hover:bg-slate-700 rounded text-slate-200 text-sm transition-colors"
  >
    Open
  </button>
  <button 
    on:click={handleSave} 
    class="px-3 py-1 bg-slate-800 hover:bg-slate-700 rounded text-slate-200 text-sm transition-colors"
  >
    Save File
  </button>
  <button 
    on:click={handleSaveProject} 
    class="px-3 py-1 bg-blue-700 hover:bg-blue-600 rounded text-white text-sm transition-colors"
  >
    Save Project
  </button>
  <div class="flex-1"></div>
  <button 
    on:click={toggleTheme} 
    class="px-3 py-1 bg-slate-800 hover:bg-slate-700 rounded text-slate-200 text-sm transition-colors"
  >
    Toggle Theme ({$theme})
  </button>
</div>
