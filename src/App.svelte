<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Loading from "./lib/Loading.svelte";
    import Error from "./lib/Error.svelte";
    import Main from "./lib/Main.svelte";
    import Wireframe from "./lib/Wireframes/Default.svelte";

    let component = Loading;

    async function load() {
        let profiles = await invoke("connect");
        component = 
            profiles == null 
            ? Error
            : Main
        ;
    }

//    setTimeout(() => load(), 1);
</script>

<main class="bg-white w-screen h-screen p-5 text-primary flex flex-col">
    <!--svelte:component this={component} /-->
    <Wireframe></Wireframe>
</main>

