<script lang="ts">
  import { jobs } from '$lib/stores/jobs';
  import { getJobs } from '$lib/api/rcClient';

  async function refreshJobs() {
    try {
      const jobList = await getJobs();
      jobs.set(jobList);
    } catch (error) {
      console.error('Failed to refresh jobs:', error);
    }
  }

  function formatProgress(progress: number): string {
    return `${(progress * 100).toFixed(1)}%`;
  }

  function getStatusColor(status: string): string {
    switch (status) {
      case 'running':
        return 'bg-blue-900 text-blue-300';
      case 'completed':
        return 'bg-green-900 text-green-300';
      case 'failed':
        return 'bg-red-900 text-red-300';
      case 'paused':
        return 'bg-yellow-900 text-yellow-300';
      default:
        return 'bg-gray-900 text-gray-300';
    }
  }
</script>

<div class="flex flex-col h-full p-6">
  <div class="flex items-center justify-between mb-6">
    <div>
      <h1 class="text-2xl font-semibold">Tasks</h1>
      <p class="text-sm text-dark-text-dim mt-1">
        {$jobs.length} task(s) · {$jobs.filter((j) => j.status === 'running').length} running
      </p>
    </div>
    <button
      class="bg-dark-hover hover:bg-opacity-80 px-4 py-2 rounded transition"
      onclick={refreshJobs}
    >
      Refresh
    </button>
  </div>

  <div class="flex-1 overflow-auto bg-dark-panel rounded border border-dark-border">
    {#if $jobs.length === 0}
      <div class="flex flex-col items-center justify-center h-full text-dark-text-dim">
        <span class="text-4xl mb-4">📋</span>
        <p class="text-lg">No tasks</p>
        <p class="text-sm mt-2">File operations will appear here</p>
      </div>
    {:else}
      <table class="w-full text-sm">
        <thead class="bg-dark-panel sticky top-0 border-b border-dark-border">
          <tr>
            <th class="text-left px-4 py-3 font-medium">Operation</th>
            <th class="text-left px-4 py-3 font-medium">Source</th>
            <th class="text-left px-4 py-3 font-medium">Destination</th>
            <th class="text-left px-4 py-3 font-medium">Status</th>
            <th class="text-right px-4 py-3 font-medium">Progress</th>
            <th class="text-right px-4 py-3 font-medium">Speed</th>
          </tr>
        </thead>
        <tbody>
          {#each $jobs as job (job.id)}
            <tr class="border-b border-dark-border hover:bg-dark-hover">
              <td class="px-4 py-3">{job.operation}</td>
              <td class="px-4 py-3 text-dark-text-dim truncate max-w-[200px]">{job.source}</td>
              <td class="px-4 py-3 text-dark-text-dim truncate max-w-[200px]">{job.destination}</td>
              <td class="px-4 py-3">
                <span class="px-2 py-1 rounded text-xs {getStatusColor(job.status)}">
                  {job.status}
                </span>
              </td>
              <td class="px-4 py-3 text-right">{formatProgress(job.progress)}</td>
              <td class="px-4 py-3 text-right text-dark-text-dim">{job.speed}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>
</div>
