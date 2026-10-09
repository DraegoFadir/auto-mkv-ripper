<script lang="ts">
    import { app } from "$lib/app.svelte";
    import { onMount } from "svelte";
    import MediaCardComponent from "../../../components/MediaCardComponent.svelte";
    import { rust } from "$lib/rust.svelte";
    import type { Media } from "../../../bindings/Media";
    import type { TVDBData } from "../../../bindings/TVDBData";

    app.state.currentStep = "title-mapping";

    let defaultSeason = $state(1);
    let defaultEpisode = $state(1);
    let loading = $state(false);
    let media = $derived(app.state.mediaSelected[0] as Extract<Media, { kind: "Series" }>);

    onMount(async () => {
        loading = true;
        try {
            await fetchExtendedData();
        } catch(e) {
            console.log(e)
        } finally {
            loading = false;
        }
    });

    $effect(() => {
        updateDefaultMap();
    })

    async function fetchExtendedData() {
        app.state.mediaSelected = await rust<Media[]>("get_tvdb_extended", { id: media.id }) ?? [];
    }

    function updateDefaultMap() {
        if(!media.seasons || !media.episodes) {
            return;
        }

        app.state.titlesMapped = app.state.titlesSelected.map((title) => {
            let map = { media, title, episode: { season: defaultSeason, episode: defaultEpisode + title.index } };
            return map;
        });
    }

</script>

<h1>Episode Mapping</h1>
{#if loading} <progress></progress> {/if}
<article>
    <header>
        <hgroup>
            <h2>Defaults</h2>
            <p>Set defaults, then adjust rows for specials or out-of-order episodes. Numbering follows Jellyfin, not the disc.</p>
        </hgroup>
    </header>
    <main class="row">
        <form role="group">
            <select name="Season" aria-label="Season" bind:value={defaultSeason} onchange={updateDefaultMap}>
                {#each media.seasons as season(season.id)}
                    <option value={season.number}>Season {String(season.number).padStart(2, "0")} - {season.name}</option>
                {/each}
            </select>
            <select name="First Episode" aria-label="First Episode" bind:value={defaultEpisode} onchange={updateDefaultMap}>
                {#each media.episodes?.filter((x) => x.season_number === defaultSeason) as episode(episode.id) }
                    <option value={episode.number}>Episode {String(episode.number).padStart(2, "0")} - {episode.name}</option>
                {/each}
            </select>
        </form>
    </main>
</article>
{#each app.state.titlesMapped as map(map.title.index)}
<MediaCardComponent media={app.state.mediaSelected[0]} >
    <select name="Season" aria-label="Season" bind:value={map.episode!.season}>
        {#each media.seasons as season(season.id)}
            <option value={season.number}>Season {String(season.number).padStart(2, "0")} - {season.name}</option>
        {/each}
    </select>
    <select name="Episode" aria-label="Episode" bind:value={map.episode!.episode}>
        {#each media.episodes?.filter((x) => x.season_number === map.episode!.season) as episode(episode.id) }
            <option value={episode.number}>Episode {String(episode.number).padStart(2, "0")} - {episode.name}</option>
        {/each}
    </select>
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