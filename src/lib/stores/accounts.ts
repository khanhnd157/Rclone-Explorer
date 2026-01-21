import { writable } from 'svelte/store';
import type { Remote } from '$lib/types';

export const accounts = writable<Remote[]>([]);
export const showAccountsModal = writable<boolean>(false);
export const showRegisterModal = writable<boolean>(false);
