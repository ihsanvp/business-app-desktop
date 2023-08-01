<script lang="ts">
    import { createEventDispatcher } from "svelte";

    const INPUT_SELECTOR = "activation-input";
    const BTN_SELECTOR = "activation-btn";

    const dispatch = createEventDispatcher();

    export let groups: number;
    export let fieldLength: number;

    let values: string[] = Array(groups).fill("");
    let elements: HTMLElement[] = [];

    function getElement(i: number) {
        if (!elements.length) {
            const fields = document.querySelectorAll("." + INPUT_SELECTOR);
            const btn = document.querySelector("." + BTN_SELECTOR);
            const el = Array.from(fields);

            el.push(btn);
            elements = el as HTMLElement[];
        }

        return elements[i];
    }

    function focusElement(index: number) {
        if (index >= 0 && index < elements.length) {
            getElement(index).focus();
        }
    }

    function handleInput(input: string, index: number) {
        values[index] = input.toUpperCase();
        getElement(index).classList.remove("error");

        if (input.length > fieldLength - 1) {
            focusElement(index + 1);
        }

        if (!input) {
            focusElement(index - 1);
        }
    }

    function onSubmit() {
        let error = false;

        values.forEach((value, i) => {
            if (value.length != fieldLength) {
                getElement(i).classList.add("error");
                error = true;
            }
        });

        if (!error) {
            dispatch("submit", values.join("-"));
        }
    }

    function onPaste(e: ClipboardEvent) {
        const key = e.clipboardData.getData("text/plain");
        const v = key.split("-");

        if (v.length == groups) {
            values = v;
            values.forEach((_, i) => getElement(i).classList.remove("error"));
        } else {
            // TODO: show error message in ui
            console.log("Cannot paste not correct format");
        }
    }
</script>

<div class="flex items-center justify-center gap-5 mt-10">
    {#each Array(groups) as _, i}
        <input
            class={`${INPUT_SELECTOR} border dark:border-dark-border border-black bg-transparent text-center tracking-[0.3em] rounded px-4 py-3 text-xl w-1/6 placeholder:text-dark-border placeholder:tracking-widest placeholder:text-center`}
            type="text"
            value={values[i]}
            maxlength={fieldLength}
            on:input={(e) => handleInput(e.currentTarget.value, i)}
            on:paste={onPaste}
        />
        {#if i < groups - 1}
            <span class="text-2xl font-light text-center"> - </span>
        {/if}
    {/each}
</div>
<div>
    <button
        class={`${BTN_SELECTOR} rounded bg-white hover:bg-gray-300 text-dark-border font-medium px-32 py-2`}
        on:click={onSubmit}>Activate</button
    >
</div>

<style>
    .error {
        @apply border-red-600;
    }
</style>
