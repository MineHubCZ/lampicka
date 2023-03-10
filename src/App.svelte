<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Loading from "./lib/Loading.svelte";
    import Error from "./lib/Error.svelte";
    import Main from "./lib/Main.svelte";
    import type {ComponentType} from "svelte";

    let component: ComponentType = Loading;

    let profiles;

    async function load() {
//        profiles = await invoke("connect");
        profiles = [
            "1;s;255;ffffff;r;255;10;",
            "1;s;255;ffffff;r;255;10;",
            "5;r;255;50;r;255;50;",
            "3;i;255;4466ff,00ffff,00ffff,4455f0,555555;rg;255;5;",
            "3;iga;255;20,4466ff,00ffff,00ffff,4455f0,555555,4466ff,00ffff,00ffff;"
        ]
        component =
            profiles
            ? Main
            : Error
        ;
    }

    setTimeout(() => load(), 1);
</script>

<main class="bg-white w-screen text-primary">
    <svelte:component this={component} profiles={profiles} />
</main>

