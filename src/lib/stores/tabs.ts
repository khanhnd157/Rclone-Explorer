import { writable, derived } from 'svelte/store';
import type { PanelState } from '$lib/types';

export interface PanelTab {
  id: string;
  name: string;
  panel: PanelState;
}

function createPanelState(): PanelState {
  return {
    remote: '',
    path: '/',
    files: [],
    selectedFiles: new Set(),
    history: ['/'],
    historyIndex: 0
  };
}

function createPanelTabsStore(side: 'left' | 'right') {
  const { subscribe, set, update } = writable<PanelTab[]>([
    {
      id: `${side}-tab-1`,
      name: 'Tab 1',
      panel: createPanelState()
    }
  ]);

  return {
    subscribe,
    addTab: () => {
      update((tabs) => {
        const newId = `${side}-tab-${Date.now()}`;
        return [
          ...tabs,
          {
            id: newId,
            name: `Tab ${tabs.length + 1}`,
            panel: createPanelState()
          }
        ];
      });
    },
    closeTab: (tabId: string) => {
      update((tabs) => {
        const filtered = tabs.filter((t) => t.id !== tabId);
        return filtered.length > 0 ? filtered : [tabs[0]];
      });
    },
    updateTab: (tabId: string, updater: (panel: PanelState) => PanelState) => {
      update((tabs) => tabs.map((t) => (t.id === tabId ? { ...t, panel: updater(t.panel) } : t)));
    },
    renameTab: (tabId: string, name: string) => {
      update((tabs) => tabs.map((t) => (t.id === tabId ? { ...t, name } : t)));
    }
  };
}

export const leftTabs = createPanelTabsStore('left');
export const rightTabs = createPanelTabsStore('right');

export const activeLeftTabId = writable<string>('left-tab-1');
export const activeRightTabId = writable<string>('right-tab-1');

export const activeLeftTab = derived([leftTabs, activeLeftTabId], ([$tabs, $activeTabId]) =>
  $tabs.find((t) => t.id === $activeTabId)
);

export const activeRightTab = derived([rightTabs, activeRightTabId], ([$tabs, $activeTabId]) =>
  $tabs.find((t) => t.id === $activeTabId)
);

export const activePanelSide = writable<'left' | 'right'>('left');
