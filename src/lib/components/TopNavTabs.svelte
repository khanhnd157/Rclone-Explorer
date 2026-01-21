<script lang="ts">
  import { activeNavTab, type NavTab } from '$lib/stores/navigation';
  import WindowControls from './WindowControls.svelte';

  interface Tab {
    id: NavTab;
    label: string;
  }

  const tabs: Tab[] = [
    { id: 'explorer', label: 'Explorer' },
    { id: 'accounts', label: 'Accounts' },
    { id: 'tasks', label: 'Tasks' },
    { id: 'options', label: 'Options' },
    { id: 'help', label: 'Help' }
  ];

  function selectTab(tabId: NavTab) {
    activeNavTab.set(tabId);
  }

  function getIcon(tabId: NavTab) {
    const icons: Record<NavTab, string> = {
      explorer: '<svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 5h14M3 10h14M3 15h14"/><path d="M7 3v14M13 3v14"/></svg>',
      accounts: '<svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="10" cy="7" r="3"/><path d="M5 17c0-2.5 2.5-4 5-4s5 1.5 5 4"/></svg>',
      tasks: '<svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="10" cy="10" r="7"/><path d="M10 6v4l3 2"/></svg>',
      options: '<svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="10" cy="10" r="2"/><path d="M10 3v1.5M10 15.5V17M6.34 6.34l-1.06-1.06M14.72 14.72l1.06 1.06M3 10h1.5M15.5 10H17M6.34 13.66l-1.06 1.06M14.72 5.28l1.06-1.06"/></svg>',
      help: '<svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="10" cy="10" r="7"/><path d="M8 8c0-1.1.9-2 2-2s2 .9 2 2-.9 2-2 2"/><circle cx="10" cy="14" r="0.5" fill="currentColor"/></svg>'
    };
    return icons[tabId];
  }
</script>

<div class="select-none">
  <div class="h-8 bg-[#323233] flex items-center justify-between border-t border-[#3e3e42]" data-tauri-drag-region>
    <div class="flex items-center gap-2 px-3">
      <div class="w-5 h-5 bg-[#0e639c] rounded flex items-center justify-center font-bold text-[9px]">
        R
      </div>
      <span class="font-semibold text-xs text-[#e0e0e0]">Rclone Explorer</span>
    </div>
    <WindowControls />
  </div>

  <nav class="bg-[#2b2b2b] border-b border-[#3e3e42]">
    <div class="flex items-center h-12 px-3">
      <div class="flex gap-0.5">
        {#each tabs as tab (tab.id)}
          <button
            class="flex flex-col items-center justify-center px-8 py-1.5 transition-all relative group
                   {$activeNavTab === tab.id 
                     ? 'bg-[#1e1e1e] text-[#0e639c]' 
                     : 'text-[#a0a0a0] hover:bg-[#333333] hover:text-[#e0e0e0]'}"
            onclick={() => selectTab(tab.id)}
          >
            {#if $activeNavTab === tab.id}
              <div class="absolute top-0 left-0 right-0 h-0.5 bg-[#0e639c]"></div>
            {/if}
            <div class="mb-0.5">
              {@html getIcon(tab.id)}
            </div>
            <span class="text-[10px] font-medium tracking-wide uppercase">{tab.label}</span>
          </button>
        {/each}
      </div>
    </div>
  </nav>
</div>
