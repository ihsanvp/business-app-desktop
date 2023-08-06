<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import ActivationInput from "../../lib/components/ActivationInput.svelte";
    import Spinner from "../../lib/components/Spinner.svelte";
    import { Toasts, toast } from "svoast";
    import { activateKey } from "../../lib/utils/activation";

    let loading = false;

    async function onSubmit(e: CustomEvent<string>) {
        try {
            loading = true;
            await activateKey(e.detail);
        } catch (err) {
            toast.error("Server Error. Please try again later.", {
                closable: true,
                duration: 5000,
            });
            loading = false;
        } finally {
            loading = false;
        }
    }

    onMount(async () => {
        setTimeout(() => invoke("window_ready"), 500);
    });
</script>

<main
    class="w-screen h-screen flex flex-col items-center justify-center px-10 py-32 gap-10"
>
    <Toasts position="bottom-center" />
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
