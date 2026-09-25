<script lang="ts">
    import Drawer from "./components/Drawer.svelte";
    import { Settings } from "@lucide/svelte";
    import SettingsForm from "./components/SettingsForm.svelte";
    import {onMount} from "svelte";
    import { loadSettings } from "./lib/settings.svelte";
    // import type { MediaResponse } from "./types/MediaResponse";
    import SearchComponent from "./components/SearchComponent.svelte";
    import { app } from "./lib/app.svelte";

    onMount(loadSettings);

    let settingsOpen: boolean = $state(false);

    // let media: MediaResponse | null = $state(null);

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

    <SearchComponent />

</main>

<footer class="container-fluid">
        <nav class="container">

            {#if app.mediaSelected.length > 0}
                <ul>
                    <li><strong>Media Selected: ({app.mediaSelected.length})</strong></li>
                </ul>
            {:else}
                <ul>
                    <li><strong>Please select one media to continue</strong></li>
                </ul>
            {/if}

            <ul>
                <li><button disabled={app.mediaSelected.length == 0}>Next</button></li>
            </ul>
        </nav>
</footer>

<Drawer bind:open={settingsOpen} title="Settings">
    <SettingsForm />
</Drawer>

<style>
</style>