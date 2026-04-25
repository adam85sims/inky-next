<script>
  import { onMount, onDestroy } from 'svelte';
  import * as monaco from 'monaco-editor';
  import { invoke } from '@tauri-apps/api/tauri';
  import { listen } from '@tauri-apps/api/event';
  import { editorContent, storyHistory, compilerErrors, theme } from '$lib/stores';

  /** @type {HTMLElement} */
  let container;
  /** @type {import('monaco-editor').editor.IStandaloneCodeEditor} */
  let editor;

  /** @type {any} */
  let debounceTimer;
  /** @type {any} */
  let unlisten;
  /** @type {import('svelte/store').Unsubscriber} */
  let unsubscribeContent;
  /** @type {import('svelte/store').Unsubscriber} */
  let unsubscribeTheme;

  /** @param {string} content */
  async function handleCompile(content) {
    try {
      storyHistory.set([]);
      await invoke('compile_ink', { content });
    } catch (err) {
      console.error('Failed to compile ink:', err);
    }
  }

  onMount(async () => {
    // Register Ink language if it's not already registered
    if (!monaco.languages.getLanguages().some(lang => lang.id === 'ink')) {
      monaco.languages.register({ id: 'ink' });
      monaco.languages.setMonarchTokensProvider('ink', {
        tokenizer: {
          root: [
            [/VAR\s+/, "keyword"],
            [/CONST\s+/, "keyword"],
            [/LIST\s+/, "keyword"],
            [/KNOT\s+/, "keyword"],
            [/==+/, "keyword"],
            [/->/, "keyword"],
            [/<-/, "keyword"],
            [/\[/, "bracket"],
            [/\]/, "bracket"],
            [/\*| \+/, "keyword"],
            [/\/\/.*/, "comment"],
            [/\/\*[\s\S]*?\*\//, "comment"],
          ]
        }
      });
    }

    editor = monaco.editor.create(container, {
      value: $editorContent,
      language: 'ink',
      theme: $theme === 'dark' ? 'vs-dark' : 'vs',
      automaticLayout: true,
    });

    editor.onDidChangeModelContent(() => {
      const currentValue = editor.getValue();
      if (currentValue !== $editorContent) {
        editorContent.set(currentValue);
      }

      clearTimeout(debounceTimer);
      debounceTimer = setTimeout(() => handleCompile(currentValue), 300);
    });

    unsubscribeContent = editorContent.subscribe(value => {
      if (editor && editor.getValue() !== value) {
        editor.setValue(value);
      }
    });

    unsubscribeTheme = theme.subscribe(value => {
      if (editor) {
        monaco.editor.setTheme(value === 'dark' ? 'vs-dark' : 'vs');
      }
    });

    unlisten = await listen('ink-output', (event) => {
      try {
        const data = JSON.parse(event.payload);
        if (data.text) {
          storyHistory.update(h => [...h, { type: 'text', content: data.text }]);
          // Successful compilation, clear errors
          const model = editor.getModel();
          if (model) {
            monaco.editor.setModelMarkers(model, 'ink', []);
          }
          compilerErrors.set([]);
        }
        if (data.choices) {
          storyHistory.update(h => [
            ...h,
            ...data.choices.map((/** @type {any} */ c, /** @type {number} */ i) => ({ type: 'choice', content: c, index: i + 1 }))
          ]);
        }
        if (data.issues) {
          const markers = data.issues.map((/** @type {any} */ issue) => ({
            message: issue.message,
            severity: monaco.MarkerSeverity.Error,
            startLineNumber: issue.lineNumber,
            endLineNumber: issue.lineNumber,
            startColumn: 1,
            endColumn: 100
          }));
          const model = editor.getModel();
          if (model) {
            monaco.editor.setModelMarkers(model, 'ink', markers);
          }
          compilerErrors.set(data.issues);
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
    if (unsubscribeContent) {
      unsubscribeContent();
    }
    if (unsubscribeTheme) {
      unsubscribeTheme();
    }
    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }
  });
</script>

<div bind:this={container} class="h-full w-full"></div>
