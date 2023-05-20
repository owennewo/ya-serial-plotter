<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import DeviceTile from "./DeviceTile.svelte";

  let name = "";
  let scanMessage = "";
  let devices = [];

  async function scan() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    devices = await invoke("list_ports");
    console.log(scanMessage);
  }
</script>

<button class="btn" on:click={scan}>Scan</button>
{#each devices as device}
  <DeviceTile {device} />
{/each}

<style>
</style>
