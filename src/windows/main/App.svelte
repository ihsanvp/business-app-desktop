<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    onMount(() => {
        setTimeout(() => invoke("window_ready"), 500);
    });
</script>

<main>
    <h1>Main Window</h1>
</main>
