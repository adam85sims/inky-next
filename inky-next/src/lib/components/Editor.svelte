<script>
  import { onMount, onDestroy } from 'svelte';
  import * as monaco from 'monaco-editor';
  import { invoke } from '@tauri-apps/api/tauri';
  import { listen } from '@tauri-apps/api/event';
  import { editorContent, storyHistory } from '$lib/stores';

  /** @type {HTMLElement} */
  let container;
  /** @type {import('monaco-editor').editor.IStandaloneCodeEditor} */
  let editor;

  let debounceTimer;
  let unlisten;

  async function handleCompile(content) {
    try {
      await invoke('compile_ink', { content });
    } catch (err) {
      console.error('Failed to compile ink:', err);
    }
  }

  onMount(async () => {
    editor = monaco.editor.create(container, {
      value: $editorContent,
      language: 'markdown', // We'll refine this to 'ink' later
      theme: 'vs-dark',
      automaticLayout: true,
    });

    editor.onDidChangeModelContent(() => {
      const currentValue = editor.getValue();
      editorContent.set(currentValue);

      clearTimeout(debounceTimer);
      debounceTimer = setTimeout(() => handleCompile(currentValue), 300);
    });

    unlisten = await listen('ink-output', (event) => {
      try {
        const data = JSON.parse(event.payload);
        if (data.text) {
          storyHistory.update(h => [...h, { type: 'text', content: data.text }]);
        }
      } catch (e) {
        console.error("Failed to parse ink output:", e, event.payload);
      }
    });
  });

  onDestroy(() => {
    if (editor) {
      editor.dispose();
    }
    if (unlisten) {
      unlisten();
    }
    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }
  });
</script>

<div bind:this={container} class="h-full w-full"></div>
