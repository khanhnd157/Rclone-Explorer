<script lang="ts">
  import { 
    leftTabs, rightTabs,
    activeLeftTabId, activeRightTabId
  } from '$lib/stores/tabs';

  interface Props {
    side: 'left' | 'right';
  }

  let { side }: Props = $props();

  const tabsStore = side === 'left' ? leftTabs : rightTabs;
  const activeTabIdStore = side === 'left' ? activeLeftTabId : activeRightTabId;
  
  function selectTab(tabId: string) {
    activeTabIdStore.set(tabId);
  }

  function closeTab(tabId: string, event: Event) {
    event.stopPropagation();
    const currentTabs = side === 'left' ? $leftTabs : $rightTabs;
    const currentActiveId = side === 'left' ? $activeLeftTabId : $activeRightTabId;
    
    tabsStore.closeTab(tabId);
    if (currentActiveId === tabId && currentTabs.length > 0) {
      activeTabIdStore.set(currentTabs[0].id);
    }
  }

  let tabs = $derived(side === 'left' ? $leftTabs : $rightTabs);
  let activeTabId = $derived(side === 'left' ? $activeLeftTabId : $activeRightTabId);
</script>

<div class="bg-dark-panel border-b border-dark-border h-10 flex items-center px-2 gap-1">
  {#each tabs as tab (tab.id)}
    <button
      class="flex items-center gap-2 px-4 py-1.5 rounded transition {activeTabId === tab.id ? 'bg-dark-bg' : 'hover:bg-dark-hover'}"
      onclick={() => selectTab(tab.id)}
    >
      <span class="text-sm">{tab.name}</span>
      {#if tabs.length > 1}
        <span
          class="text-xs hover:text-red-400 transition"
          onclick={(e) => closeTab(tab.id, e)}
        >
          ✕
        </span>
      {/if}
    </button>
  {/each}

  <button
    class="px-3 py-1 hover:bg-dark-hover rounded transition text-sm"
    onclick={() => tabsStore.addTab()}
  >
    +
  </button>
</div>
