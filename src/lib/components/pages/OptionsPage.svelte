<script lang="ts">
  import { checkRcloneVersion, installRclone, updateRclone } from '$lib/api/rcClient';
  import type { RcloneInfo } from '$lib/types';
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';

  let rcloneInfo = $state<RcloneInfo | null>(null);
  let darkMode = $state(true);
  let autoRefresh = $state(true);
  let refreshInterval = $state(30);
  let isInstalling = $state(false);
  let installMessage = $state('');
  let downloadProgress = $state({ downloaded: 0, total: 0, percentage: 0 });

  onMount(() => {
    const unlisten = listen('rclone-download-progress', (event: any) => {
      downloadProgress = event.payload;
    });
    
    return () => {
      unlisten.then(fn => fn());
    };
  });

  async function loadRcloneInfo() {
    try {
      rcloneInfo = await checkRcloneVersion();
    } catch (err) {
      console.error('Failed to check rclone:', err);
    }
  }

  async function handleInstallRclone() {
    isInstalling = true;
    installMessage = 'Downloading rclone...';
    downloadProgress = { downloaded: 0, total: 0, percentage: 0 };
    try {
      const result = await installRclone();
      installMessage = result;
      await loadRcloneInfo();
      setTimeout(() => { 
        installMessage = ''; 
        downloadProgress = { downloaded: 0, total: 0, percentage: 0 };
      }, 3000);
    } catch (err) {
      installMessage = `Error: ${err}`;
      downloadProgress = { downloaded: 0, total: 0, percentage: 0 };
    } finally {
      isInstalling = false;
    }
  }

  async function handleUpdateRclone() {
    isInstalling = true;
    installMessage = 'Updating rclone...';
    downloadProgress = { downloaded: 0, total: 0, percentage: 0 };
    try {
      const result = await updateRclone();
      installMessage = result;
      await loadRcloneInfo();
      setTimeout(() => { 
        installMessage = ''; 
        downloadProgress = { downloaded: 0, total: 0, percentage: 0 };
      }, 3000);
    } catch (err) {
      installMessage = `Error: ${err}`;
      downloadProgress = { downloaded: 0, total: 0, percentage: 0 };
    } finally {
      isInstalling = false;
    }
  }

  function saveSettings() {
    console.log('Settings saved:', { darkMode, autoRefresh, refreshInterval });
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
  }

  loadRcloneInfo();
</script>

<div class="flex flex-col h-full p-6">
  <h1 class="text-2xl font-semibold mb-6">Options</h1>

  <div class="max-w-2xl space-y-6">
    <section class="bg-dark-panel rounded border border-dark-border p-6">
      <h2 class="text-lg font-medium mb-4">Rclone</h2>
      <div class="space-y-4">
        {#if rcloneInfo}
          <div class="bg-dark-bg border border-dark-border rounded p-4">
            <div class="flex items-center justify-between mb-3">
              <div>
                <div class="text-sm font-medium">Status</div>
                <div class="text-xs text-dark-text-dim mt-1">
                  {#if rcloneInfo.installed}
                    <span class="text-green-500">✓ Installed</span>
                    {#if rcloneInfo.version}
                      <span class="ml-2">Version: {rcloneInfo.version}</span>
                    {/if}
                  {:else}
                    <span class="text-yellow-500">Not installed</span>
                  {/if}
                </div>
              </div>
              <div>
                {#if rcloneInfo.installed}
                  <button
                    class="bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded transition disabled:opacity-50"
                    onclick={handleUpdateRclone}
                    disabled={isInstalling}
                  >
                    {isInstalling ? 'Updating...' : 'Update Rclone'}
                  </button>
                {:else}
                  <button
                    class="bg-green-600 hover:bg-green-700 px-4 py-2 rounded transition disabled:opacity-50"
                    onclick={handleInstallRclone}
                    disabled={isInstalling}
                  >
                    {isInstalling ? 'Installing...' : 'Install Rclone'}
                  </button>
                {/if}
              </div>
            </div>
            <div class="text-xs text-dark-text-dim">
              Path: {rcloneInfo.path}
            </div>
            {#if isInstalling && downloadProgress.percentage > 0}
              <div class="mt-3">
                <div class="flex justify-between text-xs text-dark-text-dim mb-1">
                  <span>{downloadProgress.percentage.toFixed(1)}%</span>
                  <span>{formatBytes(downloadProgress.downloaded)} / {formatBytes(downloadProgress.total)}</span>
                </div>
                <div class="w-full bg-dark-bg rounded-full h-2 overflow-hidden">
                  <div 
                    class="bg-blue-600 h-full transition-all duration-300"
                    style="width: {downloadProgress.percentage}%"
                  ></div>
                </div>
              </div>
            {/if}
            {#if installMessage}
              <div class="mt-3 text-sm {installMessage.includes('Error') ? 'text-red-500' : 'text-green-500'}">
                {installMessage}
              </div>
            {/if}
          </div>
        {:else}
          <div class="text-center py-4 text-dark-text-dim">
            Loading rclone info...
          </div>
        {/if}
      </div>
    </section>

    <section class="bg-dark-panel rounded border border-dark-border p-6">
      <h2 class="text-lg font-medium mb-4">Appearance</h2>
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div>
            <label class="block text-sm font-medium">Dark Mode</label>
            <p class="text-xs text-dark-text-dim">Use dark theme</p>
          </div>
          <input
            type="checkbox"
            class="w-4 h-4"
            bind:checked={darkMode}
          />
        </div>
      </div>
    </section>

    <section class="bg-dark-panel rounded border border-dark-border p-6">
      <h2 class="text-lg font-medium mb-4">File List</h2>
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div>
            <label class="block text-sm font-medium">Auto Refresh</label>
            <p class="text-xs text-dark-text-dim">Automatically refresh file lists</p>
          </div>
          <input
            type="checkbox"
            class="w-4 h-4"
            bind:checked={autoRefresh}
          />
        </div>
        {#if autoRefresh}
          <div>
            <label class="block text-sm font-medium mb-2">Refresh Interval (seconds)</label>
            <input
              type="number"
              min="10"
              max="300"
              class="w-full bg-dark-bg border border-dark-border rounded px-3 py-2"
              bind:value={refreshInterval}
            />
          </div>
        {/if}
      </div>
    </section>

    <div class="flex justify-end">
      <button
        class="bg-dark-accent hover:bg-opacity-80 px-6 py-2 rounded transition"
        onclick={saveSettings}
      >
        Save Settings
      </button>
    </div>
  </div>
</div>
