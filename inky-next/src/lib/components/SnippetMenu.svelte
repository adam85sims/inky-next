<script>
  import { snippets } from '$lib/snippets';
  export let active = false;

  /** @param {string} content */
  function insert(content) {
    if ((/** @type {any} */ (window)).insertInkSnippet) {
      (/** @type {any} */ (window)).insertInkSnippet(content);
    }
    active = false;
  }
</script>

{#if active}
  <div class="absolute top-full left-0 mt-1 w-48 bg-slate-800 border border-slate-700 rounded shadow-xl z-50 py-1">
    {#each snippets as cat}
      <div class="px-3 py-1 text-[10px] font-bold text-slate-500 uppercase border-b border-slate-700/50">{cat.category}</div>
      {#each cat.items as item}
        <button 
          on:click={() => insert(item.content)}
          class="w-full text-left px-4 py-1.5 text-sm hover:bg-slate-700 text-slate-200"
        >
          {item.label}
        </button>
      {/each}
    {/each}
  </div>
{/if}
