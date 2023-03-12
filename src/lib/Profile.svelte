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

    export let profile: Profile;

    let top = profile.mode1;
    let bottom = profile.mode2;

    let topColor = "#ffffff";

    const modes = {
        "top": [
            ["Statický", Static],
            ["Duhový", Rainbow],
            ["Duhový přechod", RainbowGrad],
            ["Vlna", Wave],
            ["Duhova vlna", RainbowWave],
            ["Breathing", Breathing],
            ["Duhovy breathing", RainbowBreathing],
        ],
        "bottom": [
            ["Statický", StaticB],
            ["Duhový", RainbowB],
            ["Duhový přechod", RainbowGradB],
            ["Vlna", WaveB],
            ["Duhova vlna", RainbowWaveB],
            ["Breathing", BreathingB],
            ["Duhovy breathing", RainbowBreathingB],
        ]
    };

    let config = "";
    async function send() {
        let result = await invoke("write", { setting: config })
    }

    function hasSpeed(mode) {
        return ![StaticB, Static].includes(mode);
    }
</script>
<input type="color" id="color1" class="hidden-picker" bind:value={topColor}>
<div class="border-2 border-primary flex p-5 h-full gap-16 justify-center items-center">
    <div class="grid grid-cols-2 gap-4">
        <div>
            Horní zóna
            <div class="grid gap-4">
                {#each modes["top"] as cmode, index}
                    <div 
                        class="button {top == cmode[1] ? 'selected' : 'unselected'}"
                        on:click={() => top = cmode[1]}                
                    >
                        {cmode[0]}
                    </div>
                {/each}    
            </div>
            <div class="flex flex-col">
                jas
                <input type="range" class="slide">
            </div>
            {#if hasSpeed(top)}
                <div class="flex flex-col">
                    rychlost
                    <input type="range" class="slide">
                </div>
            {/if}
        </div>
        <div>
            Spodní zóna
            <div class="grid gap-4">
                {#each modes["bottom"] as cmode, index}
                    <div 
                         class="button {bottom == cmode[1] ? 'selected' : 'unselected'}"
                         on:click={() => bottom = cmode[1]}                
                    >
                        {cmode[0]}
                    </div>
                {/each}    
            </div>
            <div class="flex flex-col">
                jas
                <input type="range" class="slide">
            </div>
            {#if hasSpeed(bottom)}
                <div class="flex flex-col">
                    rychlost
                    <input type="range" class="slide">
                </div>
            {/if}
        </div>
        <div class="gap-4">
        <div class="grid grid-cols-2 gap-4">
            </div>
        </div>
    </div>
    <div class="w-3/6 xl:w-2/6"><Lamp top={top} bottom={bottom} topColor={topColor}></Lamp></div>
</div>
