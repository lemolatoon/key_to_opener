<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import Greet from "./lib/Greet.svelte";

  import { register } from "@tauri-apps/api/globalShortcut";
  import { onMount } from "svelte";
  onMount(async () => {
    await register("Alt+Ctrl+Shift+F1", async () => {
      await invoke("hello");
      console.log("Alt+Ctrl+Shift+F1 was pressed");
    });

    await register("Alt+Ctrl+Shift+F6", async () => {
      console.log("Alt+Ctrl+Shift+F6 was pressed");
      await invoke("command1");
    });
  });
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>Click on the Tauri, Vite, and Svelte logos to learn more.</p>

  <div class="row">
    <Greet />
  </div>
</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
