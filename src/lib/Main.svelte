<script lang="ts">
    import Profile from "./Profile.svelte";
    import type { Profile as ProfileInterface } from "src/profiles";

    export let profiles: [ProfileInterface];
    let profile = 1;
</script>
<div class="h-screen fixed w-screen top-0 left-0">
    <div class="absolute top-0 left-0 w-full h-full p-5">
        <div class="grid grid-cols-5 gap-5 mb-5">
            {#each [1, 2, 3, 4, 5] as curent_profile}
                <div 
                   class="button shadow-xl 
                    {profile == curent_profile ? 'selected' : 'unselected'}
                    "
                    on:click={() => profile = curent_profile}
                >
                    Profil {curent_profile}
                </div>
            {/each}
        </div>
        <Profile top={profiles[profile - 1].mode1} bottom={profiles[profile - 1].mode2} id={profiles[profile - 1].id}/>
    </div>
</div>
