<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    onMount(() => {
        setTimeout(() => invoke("app_window_ready"), 500);
    });
</script>

<main>
    <h1>Activation Window</h1>
</main>
