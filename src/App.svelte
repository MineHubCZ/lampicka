<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Loading from "./lib/Loading.svelte";
    import Error from "./lib/Error.svelte";
    import Main from "./lib/Main.svelte";

    let component = Loading;

    let profiles;

    async function load() {
        profiles = await invoke("connect");
        component = 
            profiles
            ? Main
            : Error
        ;
    }

    setTimeout(() => load(), 1);
</script>

<main class="bg-white w-screen h-screen p-5 text-primary flex flex-col">
    {#if component instanceof Main}
        <Main profiles={profiles} />
    {:else}
        <svelte:component this={component} />
    {/if}
</main>

