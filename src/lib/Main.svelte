<script lang="ts">
    import Profile from "./Profile.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    let profile = 1;

    function handleClick(curent_profile) {
        profile = curent_profile;
    }
</script>

<div class="grid grid-cols-5 gap-4">
    {#each [1, 2, 3, 4, 5] as curent_profile}
        <div 
            class="border-2 border-primary p-2 transition ease-in-out shadow-xl {profile == curent_profile ? 'text-white bg-primary' : 'hover:bg-primary hover:text-white'}"
            on:click={() => handleClick(curent_profile)}
        >
            Profil {curent_profile}
        </div>
    {/each}
</div>
<Profile profile={profile} />

