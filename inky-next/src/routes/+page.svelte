<script>
  import Editor from '$lib/components/Editor.svelte';
  import Preview from '$lib/components/Preview.svelte';
  import Toolbar from '$lib/components/Toolbar.svelte';
  import '../app.css';
</script>

<div class="flex flex-col h-screen w-screen overflow-hidden">
  <Toolbar />
  <main class="flex flex-1 overflow-hidden">
    <div class="flex-1 border-r border-slate-800">
      <Editor />
    </div>
    <div class="flex-1">
      <Preview />
    </div>
  </main>
</div>
