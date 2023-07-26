<script lang="ts">
  import { afterUpdate } from "svelte";
  import type { DiabloItem } from "../lib/models";

  export let showModal: boolean;
  export let selectedItem: DiabloItem | undefined;

  let dialog: HTMLDialogElement;
  let contentDiv: HTMLDivElement;

  $: if (dialog && showModal) {
    dialog.showModal();
    afterUpdate(() => {
      contentDiv.scrollTop = 0;
    });
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<div>
  <dialog
    bind:this={dialog}
    on:close={() => (showModal = false)}
    on:click|self={() => dialog.close()}
    class="modal"
  >
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div on:click|stopPropagation class="modal-box" bind:this={contentDiv}>
      <h3 class="font-bold text-lg m-2">Item</h3>
      <p class="py-4 whitespace-pre-wrap">
        {selectedItem?.text_value}
      </p>
      <div class="modal-action">
        <!-- svelte-ignore a11y-autofocus -->
        <button autofocus class="btn" on:click={() => dialog.close()}
          >Close</button
        >
      </div>
    </div>
  </dialog>
</div>
