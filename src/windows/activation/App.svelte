<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import * as os from "@tauri-apps/api/os";
    import ActivationInput from "../../lib/components/ActivationInput.svelte";
    import Spinner from "../../lib/components/Spinner.svelte";

    let loading = false;

    async function onSubmit(e: CustomEvent<string>) {
        try {
            loading = true;
            await activate(e.detail);
        } catch (err) {
            // TODO: display error
            console.log(err);
            loading = false;
        } finally {
            loading = false;
        }
    }

    async function activate(key: string) {
        if (!key) return;

        const res = await fetch(
            `http://localhost:8000/activation/status/${key}`
        );

        if (res.status != 200) {
            // TODO: show error message in ui
            console.log("invalid key");
            return;
        }

        // TODO: single device activation logic !IMPORTANT
        await invoke("save_activation", { key });

        await invoke("activation_complete");
    }

    onMount(() => {
        setTimeout(() => invoke("window_ready"), 500);
    });
</script>

<main
    class="w-screen h-screen flex flex-col items-center justify-center px-10 py-32 gap-10"
>
    {#if loading}
        <div
            class="fixed inset-0 z-[999] backdrop-blur-sm flex items-center justify-center"
        >
            <Spinner class="w-10 h-10 text-white" />
        </div>
    {/if}
    <div class="text-center text-5xl font-semibold">Business App</div>
    <div class="text-center opacity-60 px-32">
        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Eius ut
        placeat facilis officiis, minima repellendus. Amet vero quas molestias
        autem.
    </div>
    <ActivationInput groups={4} fieldLength={4} on:submit={onSubmit} />
</main>
