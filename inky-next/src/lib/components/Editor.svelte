<script>
  import { onMount } from 'svelte';
  import * as monaco from 'monaco-editor';
  import { editorContent } from '$lib/stores';

  /** @type {HTMLElement} */
  let container;
  /** @type {import('monaco-editor').editor.IStandaloneCodeEditor} */
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
