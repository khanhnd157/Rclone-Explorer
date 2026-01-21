<script lang="ts">
  let showHelp = $state(false);

  function toggleHelp(e: KeyboardEvent) {
    if (e.key === 'F1') {
      e.preventDefault();
      showHelp = !showHelp;
    }
    if (e.key === 'Escape' && showHelp) {
      showHelp = false;
    }
  }

  $effect(() => {
    document.addEventListener('keydown', toggleHelp);
    return () => document.removeEventListener('keydown', toggleHelp);
  });
</script>

{#if showHelp}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" onclick={() => showHelp = false}>
    <div class="bg-dark-panel border border-dark-border rounded-lg p-6 max-w-md" onclick={(e) => e.stopPropagation()}>
      <h3 class="text-xl font-semibold mb-4">Keyboard Shortcuts</h3>
      <div class="space-y-2 text-sm">
        <div class="flex justify-between">
          <span class="text-dark-text-dim">New Tab</span>
          <kbd class="bg-dark-bg px-2 py-1 rounded">Ctrl+T</kbd>
        </div>
        <div class="flex justify-between">
          <span class="text-dark-text-dim">Close Tab</span>
          <kbd class="bg-dark-bg px-2 py-1 rounded">Ctrl+W</kbd>
        </div>
        <div class="flex justify-between">
          <span class="text-dark-text-dim">Switch Panel</span>
          <kbd class="bg-dark-bg px-2 py-1 rounded">Tab</kbd>
        </div>
        <div class="flex justify-between">
          <span class="text-dark-text-dim">Copy</span>
          <kbd class="bg-dark-bg px-2 py-1 rounded">F5</kbd>
        </div>
        <div class="flex justify-between">
          <span class="text-dark-text-dim">Move</span>
          <kbd class="bg-dark-bg px-2 py-1 rounded">F6</kbd>
        </div>
        <div class="flex justify-between">
          <span class="text-dark-text-dim">Delete</span>
          <kbd class="bg-dark-bg px-2 py-1 rounded">Delete</kbd>
        </div>
        <div class="flex justify-between">
          <span class="text-dark-text-dim">Help</span>
          <kbd class="bg-dark-bg px-2 py-1 rounded">F1</kbd>
        </div>
      </div>
      <button class="mt-6 w-full bg-dark-accent hover:bg-opacity-80 px-4 py-2 rounded transition" onclick={() => showHelp = false}>
        Close
      </button>
    </div>
  </div>
{/if}
