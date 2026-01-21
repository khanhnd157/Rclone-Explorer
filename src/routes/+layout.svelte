<script lang="ts">
  import '../app.css';
  import TopNavTabs from '$lib/components/TopNavTabs.svelte';
  import StatusBar from '$lib/components/StatusBar.svelte';
  import KeyboardHelper from '$lib/components/KeyboardHelper.svelte';
  import RegisterAccountModal from '$lib/components/RegisterAccountModal.svelte';
  import ExplorerPage from '$lib/components/pages/ExplorerPage.svelte';
  import AccountsPage from '$lib/components/pages/AccountsPage.svelte';
  import TasksPage from '$lib/components/pages/TasksPage.svelte';
  import OptionsPage from '$lib/components/pages/OptionsPage.svelte';
  import HelpPage from '$lib/components/pages/HelpPage.svelte';
  import { onMount } from 'svelte';
  import { listRemotes } from '$lib/api/rcClient';
  import { accounts } from '$lib/stores/accounts';
  import { initKeyboardShortcuts } from '$lib/stores/keyboard';
  import { activeNavTab } from '$lib/stores/navigation';

  onMount(async () => {
    try {
      const remotes = await listRemotes();
      accounts.set(remotes);
    } catch (error) {
      console.error('Failed to load remotes:', error);
    }

    initKeyboardShortcuts();
  });
</script>

<div class="flex flex-col h-screen bg-[#1e1e1e]">
  <TopNavTabs />
  <main class="flex-1 overflow-hidden">
    {#if $activeNavTab === 'explorer'}
      <ExplorerPage />
    {:else if $activeNavTab === 'accounts'}
      <AccountsPage />
    {:else if $activeNavTab === 'tasks'}
      <TasksPage />
    {:else if $activeNavTab === 'options'}
      <OptionsPage />
    {:else if $activeNavTab === 'help'}
      <HelpPage />
    {/if}
  </main>
  <StatusBar />
</div>

<RegisterAccountModal />
<KeyboardHelper />
