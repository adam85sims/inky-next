<script>
  import { open, save } from '@tauri-apps/api/dialog';
  import { invoke } from '@tauri-apps/api/tauri';
  import { editorContent, theme, sidebarVisible } from '$lib/stores';
  import SnippetMenu from './SnippetMenu.svelte';

  let snippetMenuVisible = false;

  async function handleOpen() {
    try {
      const selected = await open({ 
        filters: [{ name: 'Ink', extensions: ['ink'] }] 
      });
      if (selected) {
        const content = await invoke('open_file', { path: selected });
        editorContent.set(content);
      }
    } catch (err) {
      console.error('Failed to open file:', err);
    }
  }

  async function handleSave() {
    try {
      const path = await save({ 
        filters: [{ name: 'Ink', extensions: ['ink'] }] 
      });
      if (path) {
        await invoke('save_file', { path, content: $editorContent });
      }
    } catch (err) {
      console.error('Failed to save file:', err);
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
    Save
  </button>
  <div class="flex-1"></div>
  <button 
    on:click={toggleTheme} 
    class="px-3 py-1 bg-slate-800 hover:bg-slate-700 rounded text-slate-200 text-sm transition-colors"
  >
    Toggle Theme ({$theme})
  </button>
</div>
