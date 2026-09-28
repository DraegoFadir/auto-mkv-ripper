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

{#if media}
    <MediaCardComponent media={media}>
        <footer class="media-footer">
            <button type="button" class="secondary" onclick={selectMedia} disabled={hasMedia}>{hasMedia ? 'Selected' : 'Select'}</button>
        </footer>
    </MediaCardComponent>
{/if}

<style>

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