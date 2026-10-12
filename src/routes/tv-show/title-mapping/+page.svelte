<script lang="ts">
    import { app, type TitleMap } from "$lib/app.svelte";
    import { onDestroy, onMount } from "svelte";
    import MediaCardComponent from "../../../components/MediaCardComponent.svelte";
    import { rust } from "$lib/rust.svelte";
    import type { Media } from "../../../bindings/Media";
    import type { Season } from "../../../bindings/Season";
    import type { Episode } from "../../../bindings/Episode";

    app.state.currentStep = "title-mapping";

    let media = $derived(app.state.mediaSelected[0] as Extract<Media, { kind: "Series" }>);
    let seasons = $derived(media.seasons.filter((s) => media.episodes.some((e) => e.season_number === s.number)).sort((a, b) => a.number - b.number));
    let defaultSeason = $derived<Season>(seasons.find((s) => s.number === 1) ?? seasons[0]);
    let episodes = $derived(media.episodes?.filter((x) => x.season_number === defaultSeason.number));
    let defaultEpisode = $derived<Episode>(episodes[0]);

    let loading = $state(false);

    onMount(async () => {
        loading = true;
        try {
            await fetchExtendedData();
            updateDefaultMap()
        } catch(e) {
            console.log(e)
        } finally {
            loading = false;
        }
    });

    onMount(() => {
        app.validator = () => {
            if(app.state.titlesMapped.some((x) => x.error)) {
                let numInvalid = app.state.titlesMapped.filter((x) => x.error).length;
                return `${numInvalid} episodes not mapped correctly`;
            }
        }
    });
    onDestroy(() => { app.validator = undefined; })

    $effect(() => {
        const maps = app.state.titlesMapped;

        maps.forEach((map, _index) => {
            let error: string | undefined;

            if(!map.startEpisode || !map.episodes) {
                error = "Invalid episode mapping";
            } else if (map.startEpisode.number > episodes.length) {
                error = "Invalid episode selection";
            } else if (maps.some((x, index) => index !== _index && JSON.stringify(x.startEpisode) === JSON.stringify(map.startEpisode))) {
                error = "Duplicate episode detected";
            } else if (maps.some((x, index) => index !== _index && x.episodes?.some((e) => map.episodes?.includes(e)))) {
                error = "Duplicate episode detected";
            }

            map.error = error;
        });
    });

    async function fetchExtendedData() {
        app.state.mediaSelected = await rust<Media[]>("get_tvdb_extended", { id: media.id }) ?? [];
    }

    function updateDefaultMap() {
        app.resetTitleMapping();
        if(!media.seasons || !media.episodes) {
            return;
        }

        app.state.titlesSelected.forEach((title, index) => {
            let episodeNumber: number = defaultEpisode.number + index;
            
            if(index > 0) {
                let previousMap = app.state.titlesMapped[index - 1];

                if(previousMap?.episodes) {
                    episodeNumber = previousMap.episodes[previousMap.episodes.length - 1].number + 1;
                }
            }

            let startEpisode = episodes.find((e) => e.number === episodeNumber);
            let map: TitleMap = { id: crypto.randomUUID(), media, title, startEpisode: undefined };

            if(startEpisode) {
                map = { ...map, startEpisode, episodes: [startEpisode], season: defaultSeason }
            }

            app.state.titlesMapped.push(map);
        });
    }

    function updateMappedEpisodes(map: TitleMap) {
        const season = map.season;
        if(season && !map.startEpisode) {
            map.startEpisode = media.episodes.filter((x) => x.season_number === season.number)[0]
        }
        if(map.startEpisode) {
            map.episodes = [map.startEpisode]
        }
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
        <form role="group" onchange={updateDefaultMap}>
            <select name="Season" aria-label="Season" bind:value={defaultSeason}>
                {#each seasons as season(season.id)}
                    <option value={season}>Season {String(season.number).padStart(2, "0")}{season.name ? ` - ${season.name}`: ""}</option>
                {/each}
            </select>
            <select name="First Episode" aria-label="First Episode" bind:value={defaultEpisode}>
                {#each episodes as episode(episode.id) }
                    <option value={episode}>Episode {String(episode.number).padStart(2, "0")}{episode.name ? ` - ${episode.name}`: ""}</option>
                {/each}
            </select>
        </form>
    </main>
</article>
{#each app.state.titlesMapped as map(map.id)}
<MediaCardComponent media={map.media}>
    {#snippet titleInfo()}
        <div>
            <kbd>Title Index: {map.title.index}</kbd>
            <kbd>Duration: {map.title.duration}</kbd>
            <kbd>Chapters: {map.title.chapters}</kbd>
        </div>
    {/snippet}
    <form onchange={() => updateMappedEpisodes(map)}>
        <select name="Season" aria-label="Season" bind:value={map.season}>
            <option value={undefined}>Select an item</option>
            {#each seasons as season(season.id)}
                <option value={season}>Season {String(season.number).padStart(2, "0")}{season.name ? ` - ${season.name}`: ""}</option>
            {/each}
        </select>
        <select name="Episode" aria-label="Episode" bind:value={map.startEpisode} aria-invalid={map.error ? true : undefined} aria-describedby="invalid-episode">
            <option value={undefined}>Select an item</option>
            {#each media.episodes.filter((x) => x.season_number === map.season?.number) as episode(episode.id) }
                <option value={episode}>Episode {String(episode.number).padStart(2, "0")}{episode.name ? ` - ${episode.name}`: ""}</option>
            {/each}
        </select>
    </form>
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