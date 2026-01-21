<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';

  const appWindow = getCurrentWindow();
  let isMaximized = $state(false);

  $effect(() => {
    appWindow.isMaximized().then(maximized => {
      isMaximized = maximized;
    });

    const unlisten = appWindow.onResized(async () => {
      isMaximized = await appWindow.isMaximized();
    });

    return () => {
      unlisten.then(fn => fn());
    };
  });

  async function minimize() {
    await appWindow.minimize();
  }

  async function toggleMaximize() {
    await appWindow.toggleMaximize();
  }

  async function close() {
    await appWindow.close();
  }
</script>

<div class="flex items-center h-full">
  <div class="flex-1" data-tauri-drag-region></div>
  
  <div class="flex items-center h-full">
    <button
      class="w-11 h-full flex items-center justify-center text-[#a0a0a0] hover:bg-[#404040] hover:text-[#e0e0e0] transition-all"
      onclick={minimize}
      type="button"
      title="Minimize"
    >
      <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
        <line x1="0" y1="9" x2="10" y2="9" stroke="currentColor" stroke-width="1" />
      </svg>
    </button>

    <button
      class="w-11 h-full flex items-center justify-center text-[#a0a0a0] hover:bg-[#404040] hover:text-[#e0e0e0] transition-all"
      onclick={toggleMaximize}
      type="button"
      title={isMaximized ? "Restore" : "Maximize"}
    >
      {#if isMaximized}
        <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
          <rect x="2" y="0.5" width="7.5" height="7.5" stroke="currentColor" stroke-width="1" fill="none" />
          <path d="M 0.5 2 L 0.5 9.5 L 8 9.5" stroke="currentColor" stroke-width="1" fill="none" />
        </svg>
      {:else}
        <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
          <rect x="0.5" y="0.5" width="9" height="9" stroke="currentColor" stroke-width="1" fill="none" />
        </svg>
      {/if}
    </button>

    <button
      class="w-11 h-full flex items-center justify-center text-[#a0a0a0] hover:bg-[#cc6666] hover:text-white transition-all"
      onclick={close}
      type="button"
      title="Close"
    >
      <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
        <line x1="0.5" y1="0.5" x2="9.5" y2="9.5" stroke="currentColor" stroke-width="1" />
        <line x1="9.5" y1="0.5" x2="0.5" y2="9.5" stroke="currentColor" stroke-width="1" />
      </svg>
    </button>
  </div>
</div>
