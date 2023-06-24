<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import DeviceTile from "./DeviceTile.svelte";
  import { onMount } from "svelte";
  import { autoScan } from "../stores/config";

  let scanMessage = "";
  let devices = [];
  let interval = undefined;
  let loading = false;
  let reloadInterval = 2000;

  // let users = liveQuery(() => db.user.toArray());

  async function scan() {
    loading = true;
    dummy();
    // invoke("list_ports").then((result: []) => {
    //   devices = [...result];
    //   setTimeout(() => {
    //     loading = false;
    //   }, 500);
    // });
    console.log(scanMessage);
  }

  // $: if ($autoScan) {
  //   interval = setInterval(scan, reloadInterval);
  // } else {
  //   interval && clearInterval(interval);
  // }

  const dummy = () => {
    invoke("dummy", {
      numStreams: 10,
      samplesPerSecond: 100,
      periodInMillis: 5000,
    });
  };

  onMount(() => {
    console.log("mounted");
    // addUser("foo5", "bar5");
    // userData.sync()
  });
</script>

<span>Scan</span>

<div class="btn-group">
  <input
    type="radio"
    bind:group={$autoScan}
    name="scanMode"
    data-title="off"
    class="btn"
    value={false}
  />
  <input
    type="radio"
    bind:group={$autoScan}
    name="scanMode"
    data-title="auto"
    class="btn"
    value={true}
  />
</div>
{#if loading}
  <span class="material-icons md-36 md-dark pointer-events-none">sensors</span>
{/if}
{#each devices as device}
  <DeviceTile {device} />
{/each}

<button on:click={dummy}>dummy</button>

<p>localStorage</p>

<!-- {#if $users}
  {#each $users as user}
    <p>{user.key}:{user.key}</p>
  {/each}
{/if} -->

<style>
</style>
