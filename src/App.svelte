<script lang="ts">
  import Devices from "./lib/components/Devices.svelte";
  import SerialMonitor from "./lib/components/SerialMonitor.svelte";
  import Chart from "./lib/components/Chart.svelte";
  import Config from "./lib/components/Config.svelte";

  let tabs = ["Config", "Raw", "Chart"];
  let activeTabIndex = 0;

  const handleError = (event) => {
    console.log(event);
    debugger;
    // alert(event);
  };

  window.onunhandledrejection = (e) => {
    console.log(e);

    // alert(e.reason);
  };
</script>

<!-- <svelte:window on:error={handleError} on:rejectionhandled={handleError} /> -->

<main class="container">
  <div class="flex h-screen bg-gray-100">
    <!-- 1st column for icons -->
    <div class="w-15 bg-gray-800 p-0 text-white">
      <div class="mb-2 p-2 active">
        <span class="material-icons md-36 md-dark pointer-events-none">usb</span
        >
      </div>
      <div class="mb-2 p-2">
        <span class="material-icons md-36 md-dark pointer-events-none"
          >settings</span
        >
      </div>
      <!-- More icons here -->
    </div>

    <!-- 2nd column for content related to icon -->
    <div class="w-80 bg-white p-4 overflow-y-auto">
      <Devices />
    </div>

    <div class="flex-grow bg-gray-200 p-4">
      <div class="tabs">
        {#each tabs as tab, index}
          <a
            class="tab tab-bordered"
            class:tab-active={activeTabIndex == index}
            on:click={() => (activeTabIndex = index)}
          >
            {tab}
          </a>
        {/each}
      </div>

      {#if activeTabIndex == 0}
        <Config />
      {/if}

      {#if activeTabIndex == 1}
        <SerialMonitor />
      {/if}

      {#if activeTabIndex == 2}
        <Chart />
      {/if}
    </div>

    <!-- 3rd column for the editor -->
    <!-- <div class="flex-grow bg-gray-200 p-4">
      
    </div>
    <div class="flex-grow bg-gray-200 p-4">
      
    </div> -->
  </div>
</main>

<style global lang="postcss">
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>
