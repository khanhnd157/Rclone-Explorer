<script lang="ts">
  import { activePanelSide } from '$lib/stores/tabs';
  import FilePanel from './FilePanel.svelte';
  import { onMount } from 'svelte';

  let dividerX = 50;
  let isDragging = false;

  function startDrag(e: MouseEvent) {
    isDragging = true;
    document.addEventListener('mousemove', onDrag);
    document.addEventListener('mouseup', stopDrag);
  }

  function onDrag(e: MouseEvent) {
    if (!isDragging) return;
    const container = document.getElementById('dual-panel-container');
    if (container) {
      const rect = container.getBoundingClientRect();
      dividerX = ((e.clientX - rect.left) / rect.width) * 100;
      dividerX = Math.max(20, Math.min(80, dividerX));
    }
  }

  function stopDrag() {
    isDragging = false;
    document.removeEventListener('mousemove', onDrag);
    document.removeEventListener('mouseup', stopDrag);
  }

  function setActivePanel(panel: 'left' | 'right') {
    activePanelSide.set(panel);
  }

  function handleCompare() {
    console.log('Compare panels');
  }

  function handleCopyToRight() {
    console.log('Copy to right');
  }

  function handleCopyToLeft() {
    console.log('Copy to left');
  }

  function handleSynchronize() {
    console.log('Synchronize');
  }
</script>

<div id="dual-panel-container" class="flex h-full overflow-hidden">
  <div style="width: {dividerX}%" class="h-full">
    <FilePanel
      side="left"
      isActive={$activePanelSide === 'left'}
      onActivate={() => setActivePanel('left')}
    />
  </div>

    <div class="panel-divider-container">
      <div class="drag-handle top" onmousedown={startDrag}></div>
      
      <div class="divider-buttons">
        <button class="divider-btn" onclick={handleCompare} title="Compare">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M2 5h8l-2-2 1-1 4 4-4 4-1-1 2-2H2V5z"/>
            <path d="M14 11H6l2 2-1 1-4-4 4-4 1 1-2 2h8v2z"/>
          </svg>
        </button>

        <button class="divider-btn" onclick={handleSynchronize} title="Synchronize">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M13.8 5.5c-.3-1.2-1-2.2-2-3-.9-.7-2-1.1-3.2-1.1-1.7 0-3.2.9-4.1 2.2l1.4 1C6.5 3.8 7.5 3.2 8.6 3.2c1.5 0 2.8 1 3.2 2.3h-2.3l2.8 2.8 2.8-2.8h-1.3z"/>
            <path d="M2.2 10.5c.3 1.2 1 2.2 2 3 .9.7 2 1.1 3.2 1.1 1.7 0 3.2-.9 4.1-2.2l-1.4-1c-.6.8-1.6 1.4-2.7 1.4-1.5 0-2.8-1-3.2-2.3h2.3L3.7 7.7.9 10.5h1.3z"/>
          </svg>
        </button>

        <button class="divider-btn" onclick={handleCopyToRight} title="Copy to Right">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <line x1="3" y1="8" x2="13" y2="8"/>
            <polyline points="10 5 13 8 10 11"/>
          </svg>
        </button>

        <button class="divider-btn" onclick={handleCopyToLeft} title="Copy to Left">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <line x1="13" y1="8" x2="3" y2="8"/>
            <polyline points="6 5 3 8 6 11"/>
          </svg>
        </button>
      </div>

      <div class="drag-handle bottom" onmousedown={startDrag}></div>
    </div>

  <div style="width: {100 - dividerX}%" class="h-full">
    <FilePanel
      side="right"
      isActive={$activePanelSide === 'right'}
      onActivate={() => setActivePanel('right')}
    />
  </div>
</div>
