<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type {Profile} from "src/profiles";
    import Lamp from "./Lamp.svelte";
    import Breathing from "./ModesTop/Breathing.svelte";
    import Rainbow from "./ModesTop/Rainbow.svelte";
    import RainbowBreathing from "./ModesTop/RainbowBreathing.svelte";
    import RainbowGrad from "./ModesTop/RainbowGrad.svelte";
    import RainbowWave from "./ModesTop/RainbowWave.svelte";
    import Static from "./ModesTop/Static.svelte";
    import Wave from "./ModesTop/Wave.svelte";

    import BreathingB from "./ModesBottom/Breathing.svelte";
    import RainbowB from "./ModesBottom/Rainbow.svelte";
    import RainbowBreathingB from "./ModesBottom/RainbowBreathing.svelte";
    import RainbowGradB from "./ModesBottom/RainbowGrad.svelte";
    import RainbowWaveB from "./ModesBottom/RainbowWave.svelte";
    import StaticB from "./ModesBottom/Static.svelte";
    import WaveB from "./ModesBottom/Wave.svelte";

    import { hasSpeed, viceVersa } from "/src/profiles";

    export let id;
    export let top;
    export let bottom;

    const modes = {
        "top": {
            "s":  ["Statický", Static],
            "r":  ["Duhový", Rainbow],
            "rg": ["Duhový přechod", RainbowGrad],
            "w":  ["Vlna", Wave],
            "rw": ["Duhova vlna", RainbowWave],
            "b":  ["Breathing", Breathing],
            "rb": ["Duhovy breathing", RainbowBreathing],
        },
        "bottom": {
            "s":  ["Statický", StaticB],
            "r":  ["Duhový", RainbowB],
            "rg": ["Duhový přechod", RainbowGradB],
            "w":  ["Vlna", WaveB],
            "rw": ["Duhova vlna", RainbowWaveB],
            "b":  ["Breathing", BreathingB],
            "rb": ["Duhovy breathing", RainbowBreathingB],
        },
    };

    async function upload() {
        let result = await invoke("write", { setting: viceVersa(id, top, bottom) })
    }
</script>
<input type="color" id="color1" class="hidden-picker" bind:value={top.color}>
<input type="color" id="color2" class="hidden-picker" bind:value={bottom.color}>
<div class="border-2 border-primary flex p-5 h-full gap-16 justify-center items-center">
    <div class="grid grid-cols-2 gap-4">
        <div>
            Horní zóna
            <div class="grid gap-4">
                {#each Object.entries(modes["top"]) as [index, cmode]}
                    <div 
                        class="button {top.mode == index ? 'selected' : 'unselected'}"
                        on:click={() => top.mode = index}                
                    >
                        {cmode[0]}
                    </div>
                {/each}    
            </div>
            <div class="flex flex-col">
                jas
                <input type="range" class="slide">
            </div>
            {#if hasSpeed(top.mode)}
                <div class="flex flex-col">
                    rychlost
                    <input type="range" class="slide" bind:value={top.speed} min="1" max="255">
                </div>
            {/if}
        </div>
        <div>
            Spodní zóna
            <div class="grid gap-4">
                {#each Object.entries(modes["bottom"]) as [index, cmode]}
                    <div 
                         class="button {bottom.mode == index ? 'selected' : 'unselected'}"
                         on:click={() => bottom.mode = index}                
                    >
                        {cmode[0]}
                    </div>
                {/each}    
            </div>
            <div class="flex flex-col">
                jas
                <input type="range" class="slide">
            </div>
            {#if hasSpeed(bottom.mode)}
                <div class="flex flex-col">
                    rychlost
                    <input type="range" class="slide" bind:value={bottom.speed} min="1" max="255">
                </div>
            {/if}
        </div>
        <div class="gap-4">
        <div class="grid grid-cols-2 gap-4">
            </div>
        </div>
    </div>
    <div class="w-3/6 xl:w-2/6"><Lamp top={modes["top"][top.mode][1]} bottom={modes["bottom"][bottom.mode][1]} topColor={top.color} bottomColor={bottom.color} topSpeed={top.speed} bottomSpeed={bottom.speed}></Lamp></div>
    <div class="bg-primary fixed bottom-0 right-0 m-8 text-white p-2" on:click={() => upload()}>Nahrat</div>
</div>
