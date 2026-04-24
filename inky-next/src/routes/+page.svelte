<script>
  import Editor from '$lib/components/Editor.svelte';
</script>

<main class="editor-container">
  <Editor />
</main>

<style>
  :root {
    margin: 0;
    padding: 0;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }

  .editor-container {
    height: 100vh;
    width: 100vw;
  }
</style>
