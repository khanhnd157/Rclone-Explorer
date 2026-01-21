<script lang="ts">
  import type { FileItem } from '$lib/types';

  interface Props {
    files: FileItem[];
    selectedFiles: Set<string>;
    searchTerm: string;
    onToggleSelection: (path: string) => void;
    onFileDoubleClick: (file: FileItem) => void;
  }

  let { files, selectedFiles, searchTerm, onToggleSelection, onFileDoubleClick }: Props = $props();

  let sortBy = $state<'name' | 'size' | 'modified'>('name');
  let sortOrder = $state<'asc' | 'desc'>('asc');

  const filteredFiles = $derived(() => {
    let result = files.filter((f) =>
      f.name.toLowerCase().includes(searchTerm.toLowerCase())
    );

    result.sort((a, b) => {
      let comparison = 0;
      if (sortBy === 'name') {
        comparison = a.name.localeCompare(b.name);
      } else if (sortBy === 'size') {
        comparison = a.size - b.size;
      } else if (sortBy === 'modified') {
        comparison = new Date(a.modified).getTime() - new Date(b.modified).getTime();
      }
      return sortOrder === 'asc' ? comparison : -comparison;
    });

    return result;
  });

  function toggleSort(column: 'name' | 'size' | 'modified') {
    if (sortBy === column) {
      sortOrder = sortOrder === 'asc' ? 'desc' : 'asc';
    } else {
      sortBy = column;
      sortOrder = 'asc';
    }
  }

  function formatSize(bytes: number): string {
    if (bytes === 0) return '-';
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return (bytes / Math.pow(1024, i)).toFixed(2) + ' ' + units[i];
  }

  function formatDate(dateStr: string): string {
    try {
      return new Date(dateStr).toLocaleString();
    } catch {
      return dateStr;
    }
  }

  function isDrive(path: string): boolean {
    return /^[A-Z]:\\$/i.test(path);
  }

  const isShowingDrives = $derived(files.length > 0 && files.every(f => isDrive(f.path)));
</script>

<div class="flex-1 overflow-auto">
  <table class="w-full text-sm">
    <thead class="bg-dark-panel sticky top-0 border-b border-dark-border">
      <tr>
        <th class="text-left px-4 py-2 font-medium cursor-pointer hover:bg-dark-hover" onclick={() => toggleSort('name')}>
          Name {sortBy === 'name' ? (sortOrder === 'asc' ? '▲' : '▼') : ''}
        </th>
        <th class="text-right px-4 py-2 font-medium cursor-pointer hover:bg-dark-hover w-32" onclick={() => toggleSort('size')}>
          Size {sortBy === 'size' ? (sortOrder === 'asc' ? '▲' : '▼') : ''}
        </th>
        {#if !isShowingDrives}
          <th class="text-left px-4 py-2 font-medium cursor-pointer hover:bg-dark-hover w-48" onclick={() => toggleSort('modified')}>
            Modified {sortBy === 'modified' ? (sortOrder === 'asc' ? '▲' : '▼') : ''}
          </th>
        {/if}
      </tr>
    </thead>
    <tbody>
      {#each filteredFiles() as file (file.path)}
        <tr
          class="border-b border-dark-border hover:bg-dark-hover cursor-pointer {selectedFiles.has(file.path) ? 'bg-dark-accent bg-opacity-20' : ''}"
          onclick={() => onToggleSelection(file.path)}
          ondblclick={() => onFileDoubleClick(file)}
        >
          <td class="px-4 py-2 flex items-center gap-2">
            {#if isDrive(file.path)}
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                <rect x="3" y="9" width="18" height="10" rx="1" fill="#e0e0e0" stroke="#757575" stroke-width="0.5"/>
                <rect x="3" y="9" width="18" height="3" rx="1" fill="#f5f5f5"/>
                <rect x="4" y="13" width="16" height="5" fill="#4a4a4a"/>
                <circle cx="7" cy="15.5" r="1.5" fill="#4ade80">
                  <animate attributeName="opacity" values="1;0.3;1" dur="2s" repeatCount="indefinite"/>
                </circle>
              </svg>
            {:else}
              <span class="text-lg">{file.is_dir ? '📁' : '📄'}</span>
            {/if}
            <span>{file.name}</span>
          </td>
          <td class="px-4 py-2 text-right text-dark-text-dim">
            {formatSize(file.size)}
          </td>
          {#if !isShowingDrives}
            <td class="px-4 py-2 text-dark-text-dim">
              {formatDate(file.modified)}
            </td>
          {/if}
        </tr>
      {/each}
    </tbody>
  </table>
</div>
