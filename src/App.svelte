<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Loading from "./lib/Loading.svelte";
    import Error from "./lib/Error.svelte";
    import Main from "./lib/Main.svelte";

    let component = Loading;

    async function load() {
        let profiles = await invoke("connect");
        component = 
            profiles
            ? Main
            : Error
        ;

        alert(profiles);
    }

    setTimeout(() => load(), 1);
</script>

<main class="bg-white w-screen h-screen p-5 text-primary flex flex-col">
    <svelte:component this={component} />
</main>

