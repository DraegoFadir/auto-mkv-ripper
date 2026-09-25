<script lang="ts">
    import type { MovieDetails } from "./bindings/MovieDetails";
    import { invoke } from "@tauri-apps/api/core";
    import Drawer from "./components/Drawer.svelte";
    import { Settings } from "@lucide/svelte";
    import SettingsForm from "./components/SettingsForm.svelte";
    import {onMount} from "svelte";
    import { loadSettings } from "./lib/settings.svelte";

    onMount(loadSettings);

    let settingsOpen: boolean = $state(false);

    let movieId: string | null = $state(null);
    let loading: boolean = $state(false);
    let result: MovieDetails | null = $state(null);

    async function getMovie(event: SubmitEvent) {
        event.preventDefault();

        if(!movieId)
        return;
    
        loading = true;
        try {
            result = await invoke("get_tmdb", { movieId: parseInt(movieId) })
        } catch(e) {
            console.log(e)
        }
        loading = false;
    }

    function openSettings(event: any) {
        event.preventDefault();

        settingsOpen = !settingsOpen;
    }
</script>

<header class="container-fluid">
    <nav>
        <ul>
            <li><strong>Auto MKV Ripper</strong></li>
        </ul>
        <ul>
            <li>
                <a href="#" aria-label="Settings" onclick={openSettings}>
                    <Settings size={20} />
                </a>
            </li>
        </ul>
    </nav>
</header>

<main class="container">
    <form role="search" onsubmit={getMovie}>
        <input type="search" name="movie_id" placeholder="TMDB Movie ID" aria-label="TMDB Movie ID" bind:value={movieId} />
        <button type="submit" aria-label="{loading ? "Searching" : "Search"}" aria-busy="{loading}">Search</button>
    </form>
    {#if result}
        <article>
            <header>{result.original_title}</header>
            <img src="https://image.tmdb.org/t/p/w500/{result.poster_path}" alt="{result.original_title}" />
            <footer>
                <button type="button" class="outline">Select Movie</button>
            </footer>
        </article>
    {/if}
</main>

<Drawer bind:open={settingsOpen} title="Settings">
    <SettingsForm />
</Drawer>

<style>

    article img {
        height: 100%;
        max-height: 250px;
    }

    article footer {
        text-align: right;

        button {
            margin: 0;
        }
    }
</style>