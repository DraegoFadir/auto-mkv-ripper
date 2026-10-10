<script lang="ts">
    import { app, type TitleMap } from "$lib/app.svelte";
    import { onDestroy, onMount } from "svelte";
    import MediaCardComponent from "../../../components/MediaCardComponent.svelte";
    import { rust } from "$lib/rust.svelte";
    import type { Media } from "../../../bindings/Media";
    import type { TVDBData } from "../../../bindings/TVDBData";

    app.state.currentStep = "title-mapping";

    let defaultSeason = $state(1);
    let defaultEpisode = $state(1);
    let loading = $state(false);
    let media = $derived(app.state.mediaSelected[0] as Extract<Media, { kind: "Series" }>);
    let episodes = $derived(media.episodes?.filter((x) => x.season_number === defaultSeason));

    let mapping = $derived.by(mapValidation);

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

    onMount(() => {
        app.validator = () => {
            if(mapping.some((x) => x.error)) {
                let numInvalid = mapping.filter((x) => x.error).length;
                return `${numInvalid} episodes not mapped correctly`;
            }
        }
    });
    onDestroy(() => { app.validator = undefined; })

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
            let episodeNumber: number | undefined = defaultEpisode + title.index;
            let map: TitleMap = { media, title, episode: { season: defaultSeason, episode: episodeNumber }};
            return map;
        });
    }

    function mapValidation(): TitleMap[] {
        return app.state.titlesMapped.map((map, _index, mapped) => {
            if (!map.episode || !map.episode.episode) {
                const error = "Invalid episode mapping";
                return { ...map, error };
            }
            
            if (map.episode.episode > episodes.length) {
                const error = "Invalid episode selection";
                return { ...map, error };
            }

            if (mapped.some((x, index) => index !== _index && JSON.stringify(x.episode) === JSON.stringify(map.episode))) {
                console.log(map.episode)
                console.log(mapped.some((x) => JSON.stringify(x.episode) === JSON.stringify(map.episode)))
                const error = "Duplicate episode detected"
                return { ...map, error };
            }

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
                {#each episodes as episode(episode.id) }
                    <option value={episode.number}>Episode {String(episode.number).padStart(2, "0")} - {episode.name}</option>
                {/each}
            </select>
        </form>
    </main>
</article>
{#each mapping as map(map.title.index)}
<MediaCardComponent media={map.media}>
    {#snippet titleInfo()}
        <div>
            <kbd>Title Index: {map.title.index}</kbd>
            <kbd>Duration: {map.title.duration}</kbd>
            <kbd>Chapters: {map.title.chapters}</kbd>
        </div>
    {/snippet}
    <select name="Season" aria-label="Season" bind:value={map.episode!.season}>
        {#each media.seasons as season(season.id)}
            <option value={season.number}>Season {String(season.number).padStart(2, "0")} - {season.name}</option>
        {/each}
    </select>
    <select name="Episode" aria-label="Episode" bind:value={map.episode!.episode} aria-invalid={map.error ? true : undefined} aria-describedby="invalid-episode">
        {#each episodes as episode(episode.id) }
            <option value={episode.number}>Episode {String(episode.number).padStart(2, "0")} - {episode.name}</option>
        {/each}
    </select>
    {#if map.error}<small id="invalid-episode">{map.error}</small>{/if}

</MediaCardComponent>
{/each}

<style>
    .row {
        display: flex;
        gap: 1rem;
        align-items: flex-start;
    }
</style>