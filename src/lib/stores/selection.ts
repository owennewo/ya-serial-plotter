import { writable } from 'svelte/store';

const selectedSerialPort = writable(null);

export { selectedSerialPort }