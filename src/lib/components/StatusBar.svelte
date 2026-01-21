<script lang="ts">
  import { activeLeftTab, activeRightTab, activePanelSide } from '$lib/stores/tabs';
  import { jobs, showTasksModal } from '$lib/stores/jobs';

  const runningJobs = $derived($jobs.filter((j) => j.status === 'running'));
  const activePanel = $derived($activePanelSide === 'left' ? $activeLeftTab?.panel : $activeRightTab?.panel);
</script>

<div class="bg-dark-panel border-t border-dark-border h-8 flex items-center px-4 text-xs text-dark-text-dim justify-between">
  <div class="flex items-center gap-6">
    {#if activePanel}
      <span>
        {activePanel.remote}:{activePanel.path}
      </span>
      <span>
        {activePanel.files.length} items
      </span>
      <span>
        Selected: {activePanel.selectedFiles.size}
      </span>
    {/if}
  </div>

  <div class="flex items-center gap-2">
    {#if runningJobs.length > 0}
      <button
        class="flex items-center gap-2 px-2 py-1 hover:bg-dark-hover rounded transition"
        onclick={() => showTasksModal.set(true)}
      >
        <span class="animate-pulse">●</span>
        <span>{runningJobs.length} job(s) running</span>
      </button>
    {:else}
      <span>Ready</span>
    {/if}
  </div>
</div>
