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
