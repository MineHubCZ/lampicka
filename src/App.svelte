<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Loading from "./lib/Loading.svelte";
    import Main from "./lib/Main.svelte";
    import type { ComponentType } from "svelte";
    import { parse } from "/src/profiles"

    let component: ComponentType = Loading;

    let profiles;

    let retry = null;
    async function load() {
        profiles = await invoke("connect");

        if (!profiles) {
            retry = load;
            return;
        }
        component = Main;

        profiles = profiles.map((profile) => parse(profile));

    }

    setTimeout(() => load(), 1);
</script>

<main class="bg-white w-screen text-primary">
    <svelte:component this={component} profiles={profiles}  retry={retry}/>
</main>

