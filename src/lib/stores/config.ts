import { writable } from 'svelte/store';

const autoScan = writable(true);

export { autoScan }