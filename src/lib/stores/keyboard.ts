import { 
  leftTabs, rightTabs, 
  activeLeftTab, activeRightTab, 
  activeLeftTabId, activeRightTabId,
  activePanelSide 
} from './tabs';
import { get } from 'svelte/store';
import { copyItems, moveItems, deleteItems } from '$lib/api/rcClient';

export function initKeyboardShortcuts() {
  document.addEventListener('keydown', async (e) => {
    if (e.ctrlKey && e.key === 't') {
      e.preventDefault();
      const side = get(activePanelSide);
      if (side === 'left') {
        leftTabs.addTab();
      } else {
        rightTabs.addTab();
      }
    }

    if (e.ctrlKey && e.key === 'w') {
      e.preventDefault();
      const side = get(activePanelSide);
      const currentTabId = side === 'left' ? get(activeLeftTabId) : get(activeRightTabId);
      const allTabs = side === 'left' ? get(leftTabs) : get(rightTabs);
      const tabsStore = side === 'left' ? leftTabs : rightTabs;
      
      if (allTabs.length > 1) {
        tabsStore.closeTab(currentTabId);
      }
    }

    if (e.key === 'Tab' && !e.ctrlKey && !e.shiftKey) {
      e.preventDefault();
      const currentSide = get(activePanelSide);
      activePanelSide.set(currentSide === 'left' ? 'right' : 'left');
    }

    if (e.key === 'F5') {
      e.preventDefault();
      await handleCopy();
    }

    if (e.key === 'F6') {
      e.preventDefault();
      await handleMove();
    }

    if (e.key === 'F2') {
      e.preventDefault();
      console.log('Rename not implemented yet');
    }

    if (e.key === 'Delete') {
      e.preventDefault();
      await handleDelete();
    }
  });
}

async function handleCopy() {
  const side = get(activePanelSide);
  const sourceTab = side === 'left' ? get(activeLeftTab) : get(activeRightTab);
  const targetTab = side === 'left' ? get(activeRightTab) : get(activeLeftTab);
  
  if (!sourceTab?.panel || !targetTab?.panel) return;

  const sourcePanel = sourceTab.panel;
  const targetPanel = targetTab.panel;

  const selectedPaths = Array.from(sourcePanel.selectedFiles);
  if (selectedPaths.length === 0) {
    console.log('No files selected');
    return;
  }

  try {
    await copyItems(
      sourcePanel.remote,
      selectedPaths,
      targetPanel.remote,
      targetPanel.path,
      { overwrite: false, skip_existing: true }
    );
    console.log('Copy started');
  } catch (error) {
    console.error('Copy failed:', error);
  }
}

async function handleMove() {
  const side = get(activePanelSide);
  const sourceTab = side === 'left' ? get(activeLeftTab) : get(activeRightTab);
  const targetTab = side === 'left' ? get(activeRightTab) : get(activeLeftTab);
  
  if (!sourceTab?.panel || !targetTab?.panel) return;

  const sourcePanel = sourceTab.panel;
  const targetPanel = targetTab.panel;

  const selectedPaths = Array.from(sourcePanel.selectedFiles);
  if (selectedPaths.length === 0) {
    console.log('No files selected');
    return;
  }

  try {
    await moveItems(sourcePanel.remote, selectedPaths, targetPanel.remote, targetPanel.path);
    console.log('Move started');
  } catch (error) {
    console.error('Move failed:', error);
  }
}

async function handleDelete() {
  const side = get(activePanelSide);
  const tab = side === 'left' ? get(activeLeftTab) : get(activeRightTab);
  
  if (!tab?.panel) return;

  const activePanel = tab.panel;
  const selectedPaths = Array.from(activePanel.selectedFiles);

  if (selectedPaths.length === 0) {
    console.log('No files selected');
    return;
  }

  if (confirm(`Delete ${selectedPaths.length} item(s)?`)) {
    try {
      await deleteItems(activePanel.remote, selectedPaths);
      console.log('Delete completed');
    } catch (error) {
      console.error('Delete failed:', error);
    }
  }
}
