<script lang="ts">
    import { app } from "$lib/app.svelte";
    import MediaCardComponent from "../../../components/MediaCardComponent.svelte";

    app.state.currentStep = "title-mapping";

    let defaultSeason = $state(1);
    let defaultEpisode = $state(1);

    $effect(() => {
        updateDefaultMap();
    })

    function updateDefaultMap() {
        app.state.titlesMapped = app.state.titlesSelected.map((title) => {
            let map = { media: app.state.mediaSelected[0], title, episode: { season: defaultSeason, episode: defaultEpisode + title.index } };
            return map;
        });
    }

</script>

<h1>Episode Mapping</h1>
<article>
    <header><h2>Defaults</h2></header>
    <main class="row">
        <form role="group">
            <select name="Season" aria-label="Season" bind:value={defaultSeason} onchange={updateDefaultMap}>
                <option value={0}>Specials</option>
                <option value={1}>Season 1 - Blood and Sand</option>
                <option value={2}>Season 2 - Vengeance</option>
                <option value={3}>Season 3 - War of the Damned</option>
            </select>
            <select name="First Episode" aria-label="First Episode" bind:value={defaultEpisode} onchange={updateDefaultMap}>
                <option disabled selected>Select a Season</option>
                <option value={1}>Episode 1 - The Red Serpent</option>
                <option value={2}>Episode 2 - Sacramentum Gladiatorum</option>
                <option value={3}>Episode 3 - Legends</option>
                <option value={4}>Episode 4 - The Thing in the Pit</option>
            </select>
        </form>
    </main>
</article>
{#each app.state.titlesMapped as map(map.title.index)}
<MediaCardComponent media={app.state.mediaSelected[0]} >
    <form role="group">
        <select name="Season" aria-label="Season" bind:value={map.episode!.season}>
            <option value={0}>Specials</option>
            <option value={1}>Season 1 - Blood and Sand</option>
            <option value={2}>Season 2 - Vengeance</option>
            <option value={3}>Season 3 - War of the Damned</option>
        </select>
        <select name="Episode" aria-label="Episode" bind:value={map.episode!.episode}>
            <option disabled selected>Select a Season</option>
            <option value={1}>Episode 1 - The Red Serpent</option>
            <option value={2}>Episode 2 - Sacramentum Gladiatorum</option>
            <option value={3}>Episode 3 - Legends</option>
            <option value={4}>Episode 4 - The Thing in the Pit</option>
        </select>
    </form>
</MediaCardComponent>
{/each}

<style>
    .row {
        display: flex;
        gap: 1rem;
        align-items: flex-start;
    }

    .col {
        display: flex;
        flex-direction: column;
    }

    .grow {
        flex: 1;
    }

    .column-half {
        width: 50%
    }
</style>