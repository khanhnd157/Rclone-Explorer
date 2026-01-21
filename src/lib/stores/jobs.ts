import { writable } from 'svelte/store';
import type { Job } from '$lib/types';

export const jobs = writable<Job[]>([]);
export const showTasksModal = writable<boolean>(false);
