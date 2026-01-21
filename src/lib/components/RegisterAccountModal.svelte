<script lang="ts">
  import { showRegisterModal, accounts } from '$lib/stores/accounts';
  import { configCreate, configList } from '$lib/api/rcClient';
  import type { ProviderInfo } from '$lib/types';

  const providers: ProviderInfo[] = [
    { id: 'drive', name: 'Google Drive', icon: '📁', description: 'Google Drive cloud storage', requiresOAuth: true },
    { id: 'onedrive', name: 'OneDrive', icon: '☁️', description: 'Microsoft OneDrive', requiresOAuth: true },
    { id: 'dropbox', name: 'Dropbox', icon: '📦', description: 'Dropbox cloud storage', requiresOAuth: true },
    { id: 's3', name: 'Amazon S3', icon: '🪣', description: 'Amazon S3 compatible storage', requiresOAuth: false },
    { id: 'sftp', name: 'SFTP', icon: '🔐', description: 'SSH File Transfer Protocol', requiresOAuth: false },
    { id: 'webdav', name: 'WebDAV', icon: '🌐', description: 'WebDAV server', requiresOAuth: false },
    { id: 'azure', name: 'Azure Blob', icon: '☁️', description: 'Microsoft Azure Blob Storage', requiresOAuth: false },
    { id: 'box', name: 'Box', icon: '📦', description: 'Box cloud storage', requiresOAuth: true },
  ];

  let accountName = $state('');
  let selectedProvider = $state<string | null>(null);
  let searchTerm = $state('');
  let step = $state<'select' | 'configure'>('select');

  const filteredProviders = $derived(() => {
    return providers.filter((p) =>
      p.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      p.description.toLowerCase().includes(searchTerm.toLowerCase())
    );
  });

  function selectProvider(providerId: string) {
    selectedProvider = providerId;
    step = 'configure';
  }

  async function handleCreate() {
    if (!accountName || !selectedProvider) return;

    try {
      await configCreate(accountName, selectedProvider, {});
      const remotes = await configList();
      accounts.set(remotes);
      closeModal();
    } catch (error) {
      console.error('Failed to create account:', error);
      alert('Failed to create account. Please try again.');
    }
  }

  function closeModal() {
    showRegisterModal.set(false);
    accountName = '';
    selectedProvider = null;
    step = 'select';
  }

  function goBack() {
    step = 'select';
    selectedProvider = null;
  }
</script>

{#if $showRegisterModal}
  <div class="fixed inset-0 bg-black bg-opacity-60 flex items-center justify-center z-50" onclick={closeModal}>
    <div class="bg-[#252526] border border-[#3e3e42] rounded-lg shadow-2xl w-[800px] max-h-[600px] flex flex-col" onclick={(e) => e.stopPropagation()}>
      <div class="border-b border-[#3e3e42] px-4 py-3 flex items-center gap-3">
        <span class="text-2xl">➕</span>
        <h2 class="text-base font-semibold text-[#e0e0e0]">
          {step === 'select' ? 'Register Account' : 'Configure Account'}
        </h2>
        <div class="flex-1"></div>
        <button class="text-[#858585] hover:text-[#e0e0e0] transition text-xl" onclick={closeModal}>×</button>
      </div>

      {#if step === 'select'}
        <div class="flex h-[500px]">
          <div class="w-80 border-r border-[#3e3e42] flex flex-col">
            <div class="p-3 border-b border-[#3e3e42]">
              <input
                type="text"
                placeholder="Search providers..."
                class="w-full px-2 py-1.5 text-xs bg-[#1e1e1e] border border-[#3e3e42] rounded text-[#cccccc] placeholder-[#858585]"
                bind:value={searchTerm}
              />
            </div>
            <div class="flex-1 overflow-auto">
              {#each filteredProviders() as provider (provider.id)}
                <button
                  class="w-full flex items-center gap-3 px-3 py-2.5 border-b border-[#3e3e42] hover:bg-[#2a2d2e] transition text-left"
                  onclick={() => selectProvider(provider.id)}
                >
                  <span class="text-2xl">{provider.icon}</span>
                  <span class="text-sm text-[#cccccc] font-medium">{provider.name}</span>
                </button>
              {/each}
            </div>
          </div>

          <div class="flex-1 p-6 flex items-center justify-center bg-[#1e1e1e]">
            <div class="text-center text-[#858585]">
              <span class="text-5xl mb-4 block">☁️</span>
              <p class="text-sm">Select a provider to continue</p>
            </div>
          </div>
        </div>
      {:else}
        <div class="flex-1 overflow-auto p-6 bg-[#1e1e1e]">
          <div class="max-w-md mx-auto space-y-4">
            <div>
              <label class="block text-xs font-medium text-[#858585] mb-1.5 uppercase tracking-wide">Account Name</label>
              <input
                type="text"
                placeholder="my-drive"
                class="w-full px-3 py-2 text-sm bg-[#252526] border border-[#3e3e42] rounded text-[#cccccc] placeholder-[#858585]"
                bind:value={accountName}
              />
              <p class="text-[10px] text-[#858585] mt-1">Unique name for this remote</p>
            </div>

            <div>
              <label class="block text-xs font-medium text-[#858585] mb-1.5 uppercase tracking-wide">Provider</label>
              <div class="px-3 py-2 bg-[#252526] border border-[#3e3e42] rounded text-sm text-[#cccccc]">
                {providers.find((p) => p.id === selectedProvider)?.name}
              </div>
            </div>

            <div class="bg-[#3a3a2d] border border-[#4a4a3d] rounded p-4">
              <p class="text-xs text-[#c7c77c]">
                {providers.find((p) => p.id === selectedProvider)?.requiresOAuth
                  ? '⚠️ OAuth authentication required. Browser will open for authorization.'
                  : 'ℹ️ Additional configuration may be required.'}
              </p>
            </div>
          </div>
        </div>

        <div class="border-t border-[#3e3e42] px-4 py-3 flex justify-between bg-[#252526]">
          <button
            class="px-4 py-1.5 text-sm text-[#cccccc] hover:bg-[#2a2d2e] rounded transition"
            onclick={goBack}
          >
            Back
          </button>
          <button
            class="px-5 py-1.5 text-sm bg-[#0e639c] hover:bg-[#1177bb] text-white rounded transition disabled:opacity-50"
            onclick={handleCreate}
            disabled={!accountName}
          >
            Continue
          </button>
        </div>
      {/if}
    </div>
  </div>
{/if}
