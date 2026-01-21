<script lang="ts">
  import { accounts } from '$lib/stores/accounts';
  import { configList, configDelete, configReconnect } from '$lib/api/rcClient';
  import type { Remote } from '$lib/types';

  let selectedAccount = $state<Remote | null>($accounts[0] || null);
  let searchTerm = $state('');

  const filteredAccounts = $derived(() => {
    return $accounts.filter((acc) =>
      acc.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      acc.provider.toLowerCase().includes(searchTerm.toLowerCase())
    );
  });

  async function refreshAccounts() {
    try {
      const remotes = await configList();
      accounts.set(remotes);
    } catch (error) {
      console.error('Failed to refresh accounts:', error);
    }
  }

  async function handleDelete(accountName: string) {
    if (confirm(`Delete account "${accountName}"?`)) {
      try {
        await configDelete(accountName);
        await refreshAccounts();
        selectedAccount = null;
      } catch (error) {
        console.error('Failed to delete account:', error);
      }
    }
  }

  async function handleReconnect(accountName: string) {
    try {
      await configReconnect(accountName);
      await refreshAccounts();
    } catch (error) {
      console.error('Failed to reconnect account:', error);
    }
  }

  function selectAccount(account: Remote) {
    selectedAccount = account;
  }
</script>

<div class="flex h-full">
  <div class="w-80 bg-[#252526] border-r border-[#3e3e42] flex flex-col">
    <div class="p-3 border-b border-[#3e3e42]">
      <input
        type="text"
        placeholder="Filter accounts..."
        class="w-full px-2 py-1.5 text-xs bg-[#1e1e1e] border border-[#3e3e42] rounded text-[#cccccc] placeholder-[#858585]"
        bind:value={searchTerm}
      />
    </div>

    <div class="flex-1 overflow-auto">
      {#each filteredAccounts() as account (account.name)}
        <button
          class="w-full text-left px-3 py-2.5 border-b border-[#3e3e42] hover:bg-[#2a2d2e] transition flex items-center gap-3
                 {selectedAccount?.name === account.name ? 'bg-[#1e1e1e] border-l-2 border-l-[#0e639c]' : ''}"
          onclick={() => selectAccount(account)}
        >
          <span class="text-2xl">☁️</span>
          <div class="flex-1 min-w-0">
            <div class="text-sm text-[#cccccc] font-medium truncate">{account.name}</div>
            <div class="text-xs text-[#858585] truncate">{account.provider}</div>
          </div>
          <span class="px-1.5 py-0.5 rounded text-[9px] font-medium uppercase tracking-wide
                       {account.status === 'Connected' ? 'bg-[#2d5a2d] text-[#7cc77c]' : 'bg-[#5a2d2d] text-[#c77c7c]'}">
            {account.status === 'Connected' ? 'OK' : 'ERR'}
          </span>
        </button>
      {/each}
    </div>
  </div>

  <div class="flex-1 bg-[#1e1e1e] overflow-auto">
    {#if selectedAccount}
      <div class="max-w-3xl mx-auto p-6">
        <div class="mb-6 pb-4 border-b border-[#3e3e42] flex items-start justify-between">
          <div class="flex items-center gap-3">
            <span class="text-4xl">☁️</span>
            <div>
              <h2 class="text-xl font-semibold text-[#e0e0e0]">{selectedAccount.name}</h2>
              <p class="text-sm text-[#858585]">{selectedAccount.provider}</p>
            </div>
          </div>
          <div class="flex gap-2">
            <button
              class="px-3 py-1.5 text-xs bg-[#0e639c] hover:bg-[#1177bb] text-white rounded transition"
              onclick={() => handleReconnect(selectedAccount!.name)}
            >
              Reconnect
            </button>
            <button
              class="px-3 py-1.5 text-xs text-[#cc6666] hover:bg-[#2a2d2e] rounded transition"
              onclick={() => handleDelete(selectedAccount!.name)}
            >
              Delete
            </button>
          </div>
        </div>

        <div class="space-y-6">
          <section>
            <h3 class="text-sm font-semibold text-[#cccccc] mb-3 uppercase tracking-wide">General</h3>
            <div class="space-y-3">
              <div class="flex items-center">
                <label class="w-32 text-sm text-[#858585]">Account Name</label>
                <input
                  type="text"
                  value={selectedAccount.name}
                  class="flex-1 px-3 py-1.5 text-sm bg-[#252526] border border-[#3e3e42] rounded text-[#cccccc]"
                  readonly
                />
              </div>
              <div class="flex items-center">
                <label class="w-32 text-sm text-[#858585]">Provider</label>
                <input
                  type="text"
                  value={selectedAccount.provider}
                  class="flex-1 px-3 py-1.5 text-sm bg-[#252526] border border-[#3e3e42] rounded text-[#cccccc]"
                  readonly
                />
              </div>
              <div class="flex items-center">
                <label class="w-32 text-sm text-[#858585]">Status</label>
                <span class="px-2 py-1 rounded text-xs font-medium
                             {selectedAccount.status === 'Connected' ? 'bg-[#2d5a2d] text-[#7cc77c]' : 'bg-[#5a2d2d] text-[#c77c7c]'}">
                  {selectedAccount.status}
                </span>
              </div>
            </div>
          </section>

          <section>
            <h3 class="text-sm font-semibold text-[#cccccc] mb-3 uppercase tracking-wide">Authentication</h3>
            <div class="space-y-3">
              <div class="flex items-start">
                <label class="w-32 text-sm text-[#858585] pt-1.5">Type</label>
                <div class="flex-1">
                  <p class="text-sm text-[#cccccc]">OAuth 2.0</p>
                  <p class="text-xs text-[#858585] mt-1">Token refreshed automatically</p>
                </div>
              </div>
            </div>
          </section>

          <section>
            <h3 class="text-sm font-semibold text-[#cccccc] mb-3 uppercase tracking-wide">Advanced</h3>
            <div class="space-y-3">
              <div class="flex items-center">
                <label class="w-32 text-sm text-[#858585]">Chunk Size</label>
                <input
                  type="text"
                  value="8M"
                  class="flex-1 px-3 py-1.5 text-sm bg-[#252526] border border-[#3e3e42] rounded text-[#cccccc]"
                />
              </div>
              <div class="flex items-center">
                <label class="w-32 text-sm text-[#858585]">Timeout</label>
                <input
                  type="text"
                  value="60s"
                  class="flex-1 px-3 py-1.5 text-sm bg-[#252526] border border-[#3e3e42] rounded text-[#cccccc]"
                />
              </div>
            </div>
          </section>

          <div class="flex justify-end gap-2 pt-4 border-t border-[#3e3e42]">
            <button class="px-4 py-2 text-sm text-[#cccccc] hover:bg-[#2a2d2e] rounded transition">
              Cancel
            </button>
            <button class="px-4 py-2 text-sm bg-[#0e639c] hover:bg-[#1177bb] text-white rounded transition">
              Save Changes
            </button>
          </div>
        </div>
      </div>
    {:else}
      <div class="flex items-center justify-center h-full text-[#858585]">
        <div class="text-center">
          <span class="text-5xl mb-4 block">👤</span>
          <p class="text-sm">Select an account to view details</p>
        </div>
      </div>
    {/if}
  </div>
</div>
