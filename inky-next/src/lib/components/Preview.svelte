<script>
  import { storyHistory } from '$lib/stores';
  import { afterUpdate } from 'svelte';

  let scrollContainer;

  afterUpdate(() => {
    if (scrollContainer) {
      scrollContainer.scrollTo(0, scrollContainer.scrollHeight);
    }
  });
</script>

<div bind:this={scrollContainer} class="h-full overflow-y-auto p-4 bg-slate-900 text-slate-200">
  {#each $storyHistory as item}
    <div class="mb-4">
      {#if item.type === 'text'}
        <p>{item.content}</p>
      {:else if item.type === 'choice'}
        <button class="block w-full text-left p-2 border border-slate-700 hover:bg-slate-800 rounded">
          {item.content}
        </button>
      {/if}
    </div>
  {/each}
</div>
