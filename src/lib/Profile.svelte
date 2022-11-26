<script lang="ts">
    import Test from "./Modes/Test.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    const modes = [
        ['Static', Test],
        ['Rainbow', Test],
        ['Rainbow Gradient', Test],
        ['Individual', Test],
        ['Individual Gradient', Test],
        ['Individual Gradient All', Test],
    ];
    export let profile;
    let config = "";
    async function send() {
        await invoke("send", { config })
    }
</script>

<div class="border-2 border-primary w-full h-screen mt-3 p-5 flex justify-center">
    <div>
        <input type="text" bind:value={config} class="border-2 border-primary">
        <button on:click={send}>Send</button>
    </div>
</div>
