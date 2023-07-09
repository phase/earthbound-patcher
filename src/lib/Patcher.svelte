<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let originalRomLocation = "";
  let patchLocation = "";
  let destination = "";

  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

  async function chooseROM() {
    originalRomLocation = await invoke("choose_rom", {})
  }

  async function choosePatch() {
    patchLocation = await invoke("choose_patch", {})
  }

  async function chooseDestination() {
    destination = await invoke("choose_destination", {})
  }

  async function patch() {
    await invoke("patch_file", {})
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
