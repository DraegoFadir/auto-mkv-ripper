<script lang="ts">
    import { app } from "../lib/app.svelte";
    import MediaCardComponent from "../components/MediaCardComponent.svelte";
    import { rust } from "$lib/rust.svelte";
    import type { Media } from "../bindings/Media";

    app.state.currentStep = "search";

    let search: string = $state("");
    let loading: boolean = $state(false);
    let results: Media[] = $state([])

    let cmd: { byId: string, byQuery: string } | void = $derived.by(() => {
        switch(app.state.mediaType) {
            case "movie":
                return { byId: "get_tmdb_by_id", byQuery: "search_tmdb" }
            case "tv-show":
            case "anime":
                return { byId: "get_tvdb_by_id", byQuery: "search_tvdb" }
            default:
                return app.setAlert({
                    message: `Select a valid Media Type`,
                    type: 'error'
                });
        }
    });

    async function getMedia(event: SubmitEvent) {
        event.preventDefault();

        results = [];
        app.resetAlert();

        search = search.trim();
        if(!search || !cmd) {
            return app.setAlert({
                message: `Please enter a valid search term`,
                type: 'error'
            });
        }

        if(search.startsWith("id:")) {
            if(!parseInt(search)){
                return app.setAlert({
                    message: `Please enter a valid id when using the 'id:' prefix`,
                    type: 'error'
                });
            }
            results = await rust<Media[]>(cmd.byId, {id: parseInt(search.replace("id:", ""))}) ?? [];
        } else {
            results = await rust<Media[]>(cmd.byQuery, { query: search }) ?? [];
        }
    }
</script>

<form role="search" onsubmit={getMedia}>
    <select name="media-format" aria-label="Select Media Format" required bind:value={app.state.mediaType}>
        <option value="movie">Movie</option>
        <option value="tv-show">TV Show</option>
        <option value="anime">Anime</option>
    </select>
    <input type="search" name="search" placeholder="Search by title, or by id with (id:) prefix" autocomplete="off" aria-label="Search by title, or by id with (id:) prefix" bind:value={search} />
    <button type="submit" aria-label="{loading ? "Searching" : "Search"}" aria-busy="{loading}">Search</button>
</form>

<div class="flex-row">
    <article>
        <header>
            <h2>Search Results</h2>
        </header>
        <main>
            {#if results.length > 0}
                {#each results.filter((m) => !app.state.mediaSelected.includes(m)) as media(media.id) }
                <label class="media-select-label">
                    <MediaCardComponent media={media} alignChildrenEnd>
                        <input type="checkbox" name={`${media.id}`} bind:group={app.state.mediaSelected} value={media} />
                    </MediaCardComponent>
                </label>
                {/each}
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
            {app.state.mediaSelected.length} Selected
        </header>

        <main>
            {#if app.state.mediaSelected.length > 0}
                {#each app.state.mediaSelected as m (m.id)}
                    <label class="media-select-label">
                        <MediaCardComponent media={m} alignChildrenEnd>
                            <input type="checkbox" name={`${m.id}`} bind:group={app.state.mediaSelected} value={m} />
                        </MediaCardComponent>
                    </label>
                {/each}
            {:else}
                <hgroup>
                    <h3>No Media Selected</h3>
                    <p>Please search for media and add it from the select list</p>
                </hgroup>
            {/if}
        </main>

    </article>
    
</div>


<style>

    .flex-row {
        display: flex;
        gap: 1rem;
        max-height: calc(100vh - 435px);
    }

    .flex-row > article {
        flex: 1;
        min-width: 0;
        display: flex;
        flex-direction: column;
        min-height: 0;
    }

    .flex-row > article main {
        flex: 1;
        min-height: 0;
        overflow-y: auto;
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
</style>