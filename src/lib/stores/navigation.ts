import { writable } from 'svelte/store';

export type NavTab = 'explorer' | 'accounts' | 'tasks' | 'options' | 'help';

export const activeNavTab = writable<NavTab>('explorer');
