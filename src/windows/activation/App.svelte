<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import ActivationInput from "../../lib/components/ActivationInput.svelte";

    function activate(e: CustomEvent<string>) {
        const key = e.detail;

        console.log(key);
    }

    onMount(() => {
        setTimeout(() => invoke("app_window_ready"), 500);
    });
</script>

<main
    class="w-screen h-screen flex flex-col items-center justify-center px-10 py-32 gap-10"
>
    <div class="text-center text-5xl font-semibold">Business App</div>
    <div class="text-center opacity-60 px-32">
        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Eius ut
        placeat facilis officiis, minima repellendus. Amet vero quas molestias
        autem.
    </div>
    <ActivationInput groups={4} fieldLength={4} on:submit={activate} />
</main>
