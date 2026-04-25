<script>
  import { storyHistory } from '$lib/stores';
  import { afterUpdate } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  /** @type {HTMLElement} */
  let scrollContainer;

  afterUpdate(() => {
    if (scrollContainer) {
      scrollContainer.scrollTo(0, scrollContainer.scrollHeight);
    }
  });

  /** @param {number} index */
  async function makeChoice(index) {
    try {
      await invoke('choose_path', { choiceIndex: index });
    } catch (err) {
      console.error('Failed to make choice:', err);
    }
  }
</script>

<div bind:this={scrollContainer} class="h-full overflow-y-auto p-4 bg-slate-900 text-slate-200">
  {#each $storyHistory as item}
    <div class="mb-4">
      {#if item.type === 'text'}
        <p>{item.content}</p>
      {:else if item.type === 'choice'}
        <button 
          on:click={() => makeChoice(item.index)}
          class="block w-full text-left p-2 border border-slate-700 hover:bg-slate-800 rounded"
        >
          {item.content}
        </button>
      {/if}
    </div>
  {/each}
</div>
