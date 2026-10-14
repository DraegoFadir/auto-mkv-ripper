<script lang="ts">
    import { app, type TitleMap } from "$lib/app.svelte";
    import { onDestroy, onMount } from "svelte"
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

            if(!map.episodes?.length) {
                error = "Invalid episode mapping";
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
            let map: TitleMap = { id: crypto.randomUUID(), media, title };

            if(startEpisode) {
                map = { ...map, episodes: [startEpisode], season: defaultSeason }
            }

            app.state.titlesMapped.push(map);
        });
    }

    function updateSeason(map: TitleMap) {
        const season = map.season;
        if(!season) {
            return 
        }

        const startEpisode = media.episodes.filter((x) => x.season_number === season.number)[0];
        if(startEpisode) {
            map.episodes = [startEpisode]
        }
    }

    function offsetStartEpisodes(index: number) {
        // No need to update anything if it's the last title in the mapping
        // I know this can go above, but it's just easier to read when unrelated checks are in their own if
        if (index === app.state.titlesMapped.length - 1) {
            return;
        }
        
        app.state.titlesMapped.map((title, i) => {
            // No need to update anything before the updated title
            if (i <= index) {
                return title;
            }

            let previousMap = app.state.titlesMapped[i - 1];
            if(previousMap.episodes?.length) {
                let lastEpisode = previousMap.episodes[previousMap.episodes.length - 1];

                const startEpisode = episodes.find((e) => e.number === lastEpisode.number + 1);
                if(!startEpisode) { return title; }             
                if(!title.episodes?.length) { return title; }

                
                for(let i = 0; i < title.episodes?.length; i++) {
                    if(i === 0) {
                        title.episodes[i] = startEpisode;
                        continue;
                    }
                    const episode = title.episodes[i];
                    const next = episodes.find((e) => e.number === episode.number + 1);
                    if(next) {
                        title.episodes[i] = next;
                    }
                }
            }

            return title;
        })
    }

</script>

<h2>Episode Mapping</h2>
{#if loading} <progress></progress> {/if}
<article>
    <header>
        <hgroup>
            <h2>Defaults</h2>
            <p>Set defaults, then adjust rows for specials or out-of-order episodes. Numbering follows Jellyfin, not the disc.</p>
        </hgroup>
    </header>
    <main>
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
{#each app.state.titlesMapped as map, i (map.id)}
<MediaCardComponent media={map.media}>
    {#snippet header()}
        <hgroup>
            <h3>{ map.media.kind === "Movie" ? map.media.title : map.media.name }</h3>
            <p>{map.title.file_name}</p>
        </hgroup>
    {/snippet}
    {#snippet body()}
        <div class="grid">
            <kbd>Title Index: {map.title.index}</kbd>
            <kbd>Duration: {map.title.duration}</kbd>
            <kbd>Chapters: {map.title.chapters}</kbd>
        </div>
    {/snippet}
    <form>
        <select name="Season" aria-label="Season" bind:value={map.season} onchange={() => updateSeason(map)}>
            <option value={undefined}>Select an item</option>
            {#each seasons as season(season.id)}
                <option value={season}>Season {String(season.number).padStart(2, "0")}{season.name ? ` - ${season.name}`: ""}</option>
            {/each}
        </select>
        <details class="dropdown">
            <!-- svelte-ignore a11y_role_supports_aria_props_implicit -->
            <summary aria-invalid={map.error ? true : undefined} aria-describedby="invalid-episode">Select episodes in title</summary>
            <ul>
                {#each media.episodes.filter((x) => x.season_number === map.season?.number) as episode(episode.id)}
                    <li>
                        <label>
                            <input type="checkbox" bind:group={map.episodes} value={episode} onchange={() => offsetStartEpisodes(i)} />
                            Episode {String(episode.number).padStart(2, "0")}{episode.name ? ` - ${episode.name}`: ""}
                        </label>
                    </li>
                {/each}
            </ul>
        </details>
        {#if map.error}<small id="invalid-episode">{map.error}</small>{/if}
    </form>

    {#snippet footer()}
        <div class="badges">
            {#each map.episodes as episode(episode.id)}
            <kbd>Episode {String(episode.number).padStart(2, "0")}</kbd>
            {/each}
        </div>
    {/snippet}
</MediaCardComponent>
{/each}

<style>
    .row {
        display: flex;
        gap: 1rem;
        align-items: center;
        justify-content: space-between;
    }

    .dropdown ul {
        max-height: 16rem;  /* about 5–6 options */
        overflow-y: auto;   /* scroll inside the list instead of the page */
    }

    details.dropdown:has(summary[aria-invalid="true"]) + small {
        color: var(--pico-del-color);
    }
    details.dropdown:has(summary[aria-invalid="false"]) + small {
        color: var(--pico-ins-color);
    }

    .badges {
        display: flex;
        gap: .25rem;
        flex-wrap: wrap;
        align-items: flex-start;
        align-content: flex-start;
        align-self: center;
    }
</style>