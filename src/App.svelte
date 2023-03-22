<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Loading from "./lib/Loading.svelte";
    import Error from "./lib/Error.svelte";
    import Main from "./lib/Main.svelte";
    import type { ComponentType } from "svelte";
    import { parse } from "/src/profiles"

    let component: ComponentType = Loading;

    let profiles;

    async function load() {
        //profiles = await invoke("connect");
        profiles = [
            "1;s;255;ffff00;r;255;10;",
            "1;s;255;ffffff;r;255;10;",
            "5;r;255;50;rw;255;50;",
            "5;r;255;50;rw;255;50;",
            "3;w;255;204466ff;rg;255;5",
        ]
        component =
            profiles
            ? Main
            : Error
        ;

        profiles = profiles.map((profile) => parse(profile));
    }

    setTimeout(() => load(), 1);
</script>

<main class="bg-white w-screen text-primary">
    <svelte:component this={component} profiles={profiles} />
</main>

