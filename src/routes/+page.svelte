<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { MovieDetails } from "../bindings/MovieDetails";
    import type { MediaResponse } from "../types/MediaResponse";
    import { app } from "../lib/app.svelte";
    import MediaCardComponent from "../components/MediaCardComponent.svelte";

    app.nextStep = "scan";

    let searchId: string = $state("");
    let loading: boolean = $state(false);
    let result: MovieDetails | null = $state(null)

    let hintText: string = $derived.by(() => {
        switch(app.mediaType) {
            case 'movie': 
                return 'TMDB Movie ID';
            case 'tv-show': 
            case 'anime':
                return 'TVDB Series ID';
            default:
                return 'Media Type not Set'
        }
    })

    let media: MediaResponse | null = $derived.by(() => {
        if(!result)
            return null;

        switch(app.mediaType) {
            case 'movie':
                let movie: MediaResponse = {
                    id: result.id,
                    title: result.original_title,
                    overview: result.overview,
                    release_date: new Date(result.release_date),
                    poster_path: `https://image.tmdb.org/t/p/w342${result.poster_path}`
                }
                return movie;
            default:
                return null;
        }
    });

    let hasMedia: boolean = $derived(app.mediaSelected.findIndex((x) => x.id === media?.id) >= 0)

    async function getMovie(event: SubmitEvent) {
        event.preventDefault();

        if(!searchId)
            return;
    
        try {
            result = await invoke("get_tmdb", { movieId: parseInt(searchId) })
        } catch(e) {
            console.log(e)
        }
    }

    function selectMedia() {
        if(media && !hasMedia) {
            app.mediaSelected.push(media)
        }
    }
</script>

<form role="search" onsubmit={getMovie}>
    <select name="media-format" aria-label="Select Media Format" required bind:value={app.mediaType}>
        <option value="movie">Movie</option>
        <option value="tv-show">TV Show</option>
        <option value="anime">Anime</option>
    </select>
    <input type="search" name="search" placeholder="{hintText}" aria-label="{hintText}" bind:value={searchId} />
    <button type="submit" aria-label="{loading ? "Searching" : "Search"}" aria-busy="{loading}">Search</button>
</form>

<div class="flex-row">
    <article>
        <header>
            <h2>Search Results</h2>
        </header>
        <main>
            {#if media && !app.mediaSelected.some(m => m.id === media.id)}
                <label class="media-select-label">
                    <MediaCardComponent media={media}>
                        <input type="checkbox" name={`${media.id}`} bind:group={app.mediaSelected} value={media} />
                    </MediaCardComponent>
                </label>
            {:else}
                <hgroup>
                    <h3>No Results Found</h3>
                    <p>Please try again or check your already selected media</p>
                </hgroup>
            {/if}
        </main>
    </article>


    <article>
        <header>
            <h2>Selected Media</h2>
            {app.mediaSelected.length} Selected
        </header>

        {#if app.mediaSelected.length > 0}
            {#each app.mediaSelected as m (m.id)}
                <label class="media-select-label">
                    <MediaCardComponent media={m}>
                        <input type="checkbox" name={`${m.id}`} bind:group={app.mediaSelected} value={m} />
                    </MediaCardComponent>
                </label>
            {/each}
        {:else}
            <hgroup>
                <h3>No Media Selected</h3>
                <p>Please search for media and add it from the select list</p>
            </hgroup>
        {/if}

    </article>
    
</div>


<style>

    .flex-row {
        display: flex;
        gap: 1rem;
        align-items: flex-start;
    }

    .flex-row > article {
        flex: 1;
        min-width: 0;
    }

    article header {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    article h2 {
        margin: 0;
    }

    .media-select-label {
        width: 100%;
    }
    
    form {
        select {
            width: 25%;
        }
    }

    .media-footer {
        text-align: right;

        button {
            margin: 0;
        }
    }

</style>