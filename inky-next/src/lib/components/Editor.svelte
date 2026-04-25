<script>
  import { onMount, onDestroy } from 'svelte';
  import * as monaco from 'monaco-editor';
  import { invoke } from '@tauri-apps/api/tauri';
  import { listen } from '@tauri-apps/api/event';
  import { get } from 'svelte/store';
  import { editorContent, storyHistory, compilerErrors, theme, activeFilePath, mainInkPath } from '$lib/stores';

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
  /** @type {import('svelte/store').Unsubscriber} */
  let unsubscribeActiveFile;

  /** @param {string} content */
  async function handleCompile(content) {
    const activePath = get(activeFilePath);
    const mainPath = get(mainInkPath);
    
    if (activePath) {
      try {
        await invoke('save_file', { path: activePath, content });
      } catch (err) {
        console.error('Failed to save file before compilation:', err);
      }
    }

    if (!mainPath) return;

    try {
      storyHistory.set([]);
      await invoke('compile_ink', { mainPath });
    } catch (err) {
      console.error('Failed to compile ink:', err);
    }
  }

  /** @param {string} template */
  export function insertSnippet(template) {
    if (!editor) return;
    /** @type {any} */
    const contribution = editor.getContribution('snippetController2');
    if (contribution) {
      editor.focus();
      contribution.insert(template);
    }
  }

  onMount(async () => {
    (/** @type {any} */ (window)).insertInkSnippet = insertSnippet;
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
      tabSize: 4,
      insertSpaces: true,
      detectIndentation: true,
      scrollBeyondLastLine: false,
      wordWrap: 'on',
      snippetSuggestions: 'top',
      suggestOnTriggerCharacters: true,
      folding: true,
      lineNumbers: 'on',
      minimap: { enabled: false },
      unicodeHighlight: { ambiguousCharacters: false }
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

    unsubscribeActiveFile = activeFilePath.subscribe(async (path) => {
      if (path && editor) {
        try {
          const content = await invoke('open_file', { path });
          editorContent.set(content);
        } catch (err) {
          console.error('Failed to load file:', err);
        }
      }
    });

    unlisten = await listen('ink-output', (event) => {
      const payload = event.payload;
      
      // Robustly split and parse multiple concatenated JSON objects
      const jsonStrings = [];
      let depth = 0;
      let start = 0;
      let inString = false;
      let escaped = false;

      for (let i = 0; i < payload.length; i++) {
        const char = payload[i];
        
        if (escaped) {
          escaped = false;
          continue;
        }

        if (char === '\\') {
          escaped = true;
          continue;
        }

        if (char === '"') {
          inString = !inString;
          continue;
        }

        if (!inString) {
          if (char === '{') {
            if (depth === 0) start = i;
            depth++;
          } else if (char === '}') {
            depth--;
            if (depth === 0) {
              jsonStrings.push(payload.substring(start, i + 1));
            }
          }
        }
      }

      for (const jsonStr of jsonStrings) {
        try {
          const data = JSON.parse(jsonStr);
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
              ...data.choices.map((/** @type {any} */ c, /** @type {number} */ i) => ({ type: 'choice', content: c.text || c, index: i + 1 }))
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
          console.error("Failed to parse ink output chunk:", e, jsonStr);
        }
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
    if (unsubscribeActiveFile) {
      unsubscribeActiveFile();
    }
    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }
  });
</script>

<div bind:this={container} class="h-full w-full"></div>
