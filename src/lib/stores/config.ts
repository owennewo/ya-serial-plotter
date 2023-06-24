import { writable } from 'svelte/store';

import { appDataDir } from '@tauri-apps/api/path';
import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';

import { exists } from '@tauri-apps/api/fs';
const appDataDirPath = await appDataDir();

// console.log("config.ts");

const configPath = `${appDataDirPath}config.json`;


let config;
export const autoScan = writable(false);

if (await exists(configPath)) {
    console.log("reading config:", configPath);
    config = JSON.parse(await readTextFile(configPath));
    console.log("config", config);
    autoScan.set(config.autoScan);

} else {
    console.log("config not found");
    config = {
        autoScan: false
    }
}


const saveConfig = async () => {
    console.log("writing file");
    await writeTextFile(configPath, JSON.stringify(config));
}

autoScan.subscribe(value => {
    console.log("autoScan.subscribe", value);
    config.autoScan = value;
    saveConfig();
});
