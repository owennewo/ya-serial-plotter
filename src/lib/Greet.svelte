<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("list_ports")
    console.log(greetMsg)
  }
</script>

<div class="flex h-screen bg-gray-100">
  <!-- 1st column for icons -->
  <div class="w-15 bg-gray-800 p-0 text-white">
    <div class="mb-2 p-2 active">
      <span class="material-icons md-36 md-dark pointer-events-none">usb</span>
    </div>
    <div class="mb-2 p-2">
      <span class="material-icons md-36 md-dark pointer-events-none">settings</span>
    </div>
    <!-- More icons here -->
  </div>

  <!-- 2nd column for content related to icon -->
  <div class="w-80 bg-white p-4 overflow-y-auto">
    <!-- Content goes here, shown when an icon is clicked -->
  </div>

  <!-- 3rd column for the editor -->
  <div class="flex-grow bg-gray-200 p-4">
    <button class="btn" on:click={greet}>Greet</button>
    <p>{greetMsg}</p>
  </div>
</div>

<style>
div.active {
  border-left: blue 2px solid;
}

.toolicon {
  cursor: pointer;
  /* padding-left: 10px; */
  border-radius: 5px;
  /* margin: 0; */
}
.material-icons.md-36 { font-size: 36px; }
.material-icons.md-dark { color: rgba(255, 255, 255, 0.3); }
.active .material-icons.md-dark { color: rgba(255, 255, 255, 1); }

</style>
