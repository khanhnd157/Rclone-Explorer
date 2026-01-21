<script lang="ts">
  import type { PanelState, FileItem } from '$lib/types';
  import { listDir, listRemotes } from '$lib/api/rcClient';
  import { accounts } from '$lib/stores/accounts';
  import { onMount } from 'svelte';
  import FileList from './FileList.svelte';
  import TabBar from './TabBar.svelte';
  import { 
    leftTabs, rightTabs, 
    activeLeftTab, activeRightTab,
    activeLeftTabId, activeRightTabId 
  } from '$lib/stores/tabs';

  interface Props {
    side: 'left' | 'right';
    isActive: boolean;
    onActivate: () => void;
  }

  let { side, isActive, onActivate }: Props = $props();

  const tabsStore = side === 'left' ? leftTabs : rightTabs;
  const activeTabStore = side === 'left' ? activeLeftTab : activeRightTab;
  const activeTabIdStore = side === 'left' ? activeLeftTabId : activeRightTabId;

  let panel = $derived($activeTabStore?.panel);
  let activeTabId = $derived($activeTabIdStore);

  function onUpdate(updater: (panel: PanelState) => PanelState) {
    if (activeTabId) {
      tabsStore.updateTab(activeTabId, updater);
    }
  }

  let searchTerm = $state('');
  let viewMode = $state<'list' | 'grid'>('list');

  async function loadFiles() {
    if (!panel) return;
    try {
      const files = await listDir(panel.remote, panel.path);
      onUpdate((p) => ({ ...p, files }));
    } catch (error) {
      console.error('Failed to load files:', error);
    }
  }

  async function navigateTo(path: string) {
    if (!panel) return;
    onUpdate((p) => ({
      ...p,
      path,
      history: [...p.history.slice(0, p.historyIndex + 1), path],
      historyIndex: p.historyIndex + 1
    }));
    await loadFiles();
  }

  async function goBack() {
    if (!panel || panel.historyIndex <= 0) return;
    const newIndex = panel.historyIndex - 1;
    onUpdate((p) => ({
      ...p,
      path: p.history[newIndex],
      historyIndex: newIndex
    }));
    await loadFiles();
  }

  async function goForward() {
    if (!panel || panel.historyIndex >= panel.history.length - 1) return;
    const newIndex = panel.historyIndex + 1;
    onUpdate((p) => ({
      ...p,
      path: p.history[newIndex],
      historyIndex: newIndex
    }));
    await loadFiles();
  }

  async function goUp() {
    if (!panel) return;
    const parts = panel.path.split('/').filter(Boolean);
    if (parts.length > 0) {
      parts.pop();
      await navigateTo('/' + parts.join('/'));
    }
  }

  function toggleSelection(filePath: string) {
    onUpdate((p) => {
      const newSelected = new Set(p.selectedFiles);
      if (newSelected.has(filePath)) {
        newSelected.delete(filePath);
      } else {
        newSelected.add(filePath);
      }
      return { ...p, selectedFiles: newSelected };
    });
  }

  function handleFileDoubleClick(file: FileItem) {
    if (file.is_dir) {
      navigateTo(file.path);
    }
  }

  let prevRemote = $state<string | undefined>(undefined);
  let prevPath = $state<string | undefined>(undefined);

  function selectAccount(accountName: string) {
    onUpdate((p) => ({ ...p, remote: accountName, path: '/' }));
    if (activeTabId) {
      tabsStore.renameTab(activeTabId, accountName);
    }
  }

  $effect(() => {
    if (panel && panel.remote && (panel.remote !== prevRemote || panel.path !== prevPath)) {
      prevRemote = panel.remote;
      prevPath = panel.path;
      loadFiles();
    }
  });
</script>

{#if panel}
  <div
    class="flex flex-col h-full border border-dark-border {isActive ? 'border-dark-accent' : ''}"
    onclick={onActivate}
  >
    <TabBar {side} />
    
    {#if !panel.remote}
      <div class="flex-1 flex items-center justify-center bg-[#1e1e1e]">
        <div class="w-full max-w-2xl p-6">
          <h2 class="text-lg font-semibold text-[#e0e0e0] mb-4">Select Location</h2>
          <div class="grid gap-2">
            <button
              class="flex items-center gap-3 p-3 bg-[#252526] border border-[#3e3e42] rounded hover:border-[#0e639c] hover:bg-[#2a2d2e] transition-all text-left"
              onclick={() => selectAccount('This PC')}
            >
              <div class="w-10 h-10 flex items-center justify-center">
                <svg width="36" height="36" viewBox="0 0 24 24" fill="none">
                  <rect x="3" y="9" width="18" height="10" rx="1" fill="#e0e0e0" stroke="#757575" stroke-width="0.5"/>
                  <rect x="3" y="9" width="18" height="3" rx="1" fill="#f5f5f5"/>
                  <rect x="4" y="13" width="16" height="5" fill="#4a4a4a"/>
                  <circle cx="7" cy="15.5" r="1.5" fill="#4ade80">
                    <animate attributeName="opacity" values="1;0.3;1" dur="2s" repeatCount="indefinite"/>
                  </circle>
                </svg>
              </div>
              <div class="flex-1">
                <div class="text-[#e0e0e0] font-medium">This PC</div>
                <div class="text-xs text-[#858585]">Local Computer</div>
              </div>
            </button>

            {#if $accounts.length > 0}
              <div class="my-2 text-xs text-[#858585] uppercase font-semibold">Cloud Storage</div>
              {#each $accounts as account (account.name)}
                <button
                  class="flex items-center gap-3 p-3 bg-[#252526] border border-[#3e3e42] rounded hover:border-[#0e639c] hover:bg-[#2a2d2e] transition-all text-left"
                  onclick={() => selectAccount(account.name)}
                >
                  <div class="w-10 h-10 bg-[#0e639c] rounded flex items-center justify-center text-lg font-bold">
                    {account.provider.charAt(0).toUpperCase()}
                  </div>
                  <div class="flex-1">
                    <div class="text-[#e0e0e0] font-medium">{account.name}</div>
                    <div class="text-xs text-[#858585]">{account.provider}</div>
                  </div>
                  <div class="text-xs px-2 py-1 rounded {account.status === 'Connected' ? 'bg-[#2d5a2d] text-[#7cc77c]' : 'bg-[#5a2d2d] text-[#c77c7c]'}">
                    {account.status}
                  </div>
                </button>
              {/each}
            {:else}
              <div class="text-center py-4 text-[#858585] text-sm">
                <p>No cloud accounts configured</p>
                <p class="mt-1">Go to Accounts tab to add connections</p>
              </div>
            {/if}
          </div>
        </div>
      </div>
    {:else}
      <div class="bg-dark-panel border-b border-dark-border px-2 py-1.5 flex items-center gap-2">
        <button
          class="px-2 py-1 hover:bg-dark-hover rounded disabled:opacity-50 disabled:cursor-not-allowed"
          onclick={goBack}
          disabled={panel.historyIndex === 0}
        >
          ←
        </button>
        <button
          class="px-2 py-1 hover:bg-dark-hover rounded disabled:opacity-50 disabled:cursor-not-allowed"
          onclick={goForward}
          disabled={panel.historyIndex >= panel.history.length - 1}
        >
          →
        </button>
        <button class="px-2 py-1 hover:bg-dark-hover rounded" onclick={goUp}>
          ↑
        </button>
        <button class="px-2 py-1 hover:bg-dark-hover rounded" onclick={loadFiles}>
          ⟳
        </button>
        <div class="flex-1 flex items-center">
          <span class="text-sm text-dark-text-dim">{panel.path}</span>
        </div>
        <input
          type="text"
          placeholder="Search..."
          class="bg-dark-bg border border-dark-border rounded px-2 py-1 text-sm w-40"
          bind:value={searchTerm}
        />
      </div>

      <FileList
        files={panel.files}
        selectedFiles={panel.selectedFiles}
        searchTerm={searchTerm}
        onToggleSelection={toggleSelection}
        onFileDoubleClick={handleFileDoubleClick}
      />
    {/if}
  </div>
{/if}
