<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { register } from "@tauri-apps/api/globalShortcut";

  window.onload = async () => {
    await register("Control+H", (shortcut) => {
      invoke("process_item");
    });
  };
</script>

<main class="container">
  <h1>D4 Enhanced</h1>
</main>
