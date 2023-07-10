<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { open, save } from '@tauri-apps/api/dialog';

  let originalRomLocation = "";
  let patchLocation = "";
  let destination = "";

  async function chooseROM() {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'SNES ROM',
        extensions: ['sfc']
      }]
    })
    if (selected !== null && !Array.isArray(selected)) {
      originalRomLocation = selected
    }
  }

  async function choosePatch() {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'ROM Patch',
        extensions: ['ebp', 'ips']
      }]
    })
    if (selected !== null && !Array.isArray(selected)) {
      patchLocation = selected
    }
  }

  async function chooseDestination() {
    const selected = await save({
      filters: [{
        name: 'SNES ROM',
        extensions: ['sfc']
      }]
    })
    if (selected !== null) {
      destination = selected
    }
  }

  async function patch() {
    // rom_file: String, patch_file: String, destination_file: String
    await invoke("patch_file", {romFile: originalRomLocation, patchFile: patchLocation, destinationFile: destination})
    originalRomLocation = ""
    patchLocation = ""
    destination = ""
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={chooseROM}>
    {#if originalRomLocation === ""}
      <button type="submit">Choose ROM</button>
    {:else}
      <button type="submit">Choosen ROM: {originalRomLocation}</button>
    {/if}
  </form>

  <form class="row" on:submit|preventDefault={choosePatch}>
    {#if patchLocation === ""}
      <button type="submit">Choose Patch</button>
    {:else}
      <button type="submit">Choosen Patch: {patchLocation}</button>
    {/if}
  </form>

  <form class="row" on:submit|preventDefault={chooseDestination}>
    {#if destination === ""}
      <button type="submit">Choose Destination</button>
    {:else}
      <button type="submit">Choosen Destination: {destination}</button>
    {/if}
  </form>

  {#if originalRomLocation !== "" && patchLocation !== "" && destination !== ""}
    <form class="row" on:submit|preventDefault={patch}>
      <button type="submit">Patch!</button>
    </form>
  {/if}
</div>
