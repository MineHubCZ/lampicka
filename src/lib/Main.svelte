<script lang="ts">
    import Profile from "./Profile.svelte";

    export let profiles: [string];
    let profile: string = profiles[0];

    function handleClick(curent_profile: number) {
        profile = profiles[curent_profile - 1];
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

