<script lang="ts">
  import { register } from "@tauri-apps/api/globalShortcut";
  import {
    api_get_items,
    api_remove_all_items,
    api_screenshot_item,
  } from "./lib/api";
  import type { DiabloItem } from "./lib/models";
  import Modal from "./components/Modal.svelte";

  let items: Array<DiabloItem> = [];
  let showModal = false;
  let selectedItem: DiabloItem | null;

  const setup = async () => {
    try {
      await register("Control+H", screenshot_and_update);
    } catch {}

    items = await api_get_items();
    setTimeout(async () => {
      await setup();
    }, 500);
  };

  const screenshot_and_update = async () => {
    await api_screenshot_item();
    items = await api_get_items();
  };

  setup();
</script>

<main class="prose text-center m-0 w-screen h-screen">
  <div>
    <h1 class="w-screen">D4 Enhanced</h1>
    <div class="grid grid-cols-10 gap-2 w-screen p-4">
      {#each items as item}
        <button
          class="btn btn-lg h-28 text-xs break-words p-0"
          on:click={() => {
            selectedItem = item;
            showModal = true;
          }}
        >
          {item.text_value.slice(0, 15)}
        </button>
      {/each}
    </div>
  </div>

  <div class="flex m-4">
    <button
      class="btn btn-error align-bottom"
      on:click={async () => await api_remove_all_items()}>CLEAR ALL</button
    >
  </div>

  <Modal bind:showModal bind:selectedItem />
</main>
