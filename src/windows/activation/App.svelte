<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { appWindow } from "@tauri-apps/api/window";
    import { Toasts, toast } from "svoast";
    import ActivationInput from "../../lib/components/ActivationInput.svelte";
    import Spinner from "../../lib/components/Spinner.svelte";
    import Icon from "@iconify/svelte";

    let loading = false;

    async function onSubmit(e: CustomEvent<string>) {
        try {
            loading = true;
            await invoke("activate_key", { key: e.detail });
        } catch (err) {
            toast.error(String(err), {
                closable: true,
                duration: 5000,
            });
        } finally {
            loading = false;
        }

        // try {
        //     loading = true;
        //     await activateKey(e.detail);
        // } catch (err) {
        //     toast.error("Server Error. Please try again later.", {
        //         closable: true,
        //         duration: 5000,
        //     });
        //     loading = false;
        // } finally {
        //     loading = false;
        // }
    }

    onMount(async () => {
        setTimeout(() => invoke("window_ready"), 500);
    });
</script>

<main class="w-screen h-screen flex flex-col">
    <div
        data-tauri-drag-region
        class="w-full h-20 flex items-center justify-end"
    >
        <div class="mr-5">
            <button
                class="hover:bg-zinc-900 p-2 rounded"
                on:click={appWindow.close}
            >
                <Icon icon="mdi:close" />
            </button>
        </div>
    </div>
    <Toasts position="bottom-center" />
    {#if loading}
        <div
            class="fixed inset-0 z-[999] backdrop-blur-sm flex items-center justify-center"
        >
            <Spinner class="w-10 h-10 text-white" />
        </div>
    {/if}
    <div
        class="flex-1 flex flex-col items-center justify-center pb-20 px-10 gap-10"
    >
        <div class="text-center text-5xl font-semibold">Business App</div>
        <div class="text-center opacity-60 px-32">
            Lorem ipsum, dolor sit amet consectetur adipisicing elit. Eius ut
            placeat facilis officiis, minima repellendus. Amet vero quas
            molestias autem.
        </div>
        <ActivationInput groups={4} fieldLength={4} on:submit={onSubmit} />
    </div>
</main>
