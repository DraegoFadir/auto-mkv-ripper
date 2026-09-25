<script lang="ts">
    import "@picocss/pico/css/pico.indigo.min.css"
    import {onMount} from "svelte";
    import { Settings } from "@lucide/svelte";

    import DrawerComponent from "../components/DrawerComponent.svelte";
    import SettingsFormComponent from "../components/SettingsFormComponent.svelte";
    import { loadSettings } from "../lib/settings.svelte";
    import { app } from "../lib/app.svelte";

    onMount(loadSettings);

    let { children } = $props();
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

    {@render children()}

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

<DrawerComponent bind:open={settingsOpen} title="Settings">
    <SettingsFormComponent />
</DrawerComponent>

<style>
    :global(body) {
        display: flex;
        flex-direction: column;
        height: 100vh;
    }

    :global(body > main) {
        flex: 1;
        min-height: 0;
        overflow: auto;
    }

    :global(body > header) {
        background: var(--pico-card-background-color);
        border-bottom: 1px solid var(--pico-muted-border-color);
    }

    :global(body > footer) {
        background: var(--pico-card-background-color);
        border-top: 1px solid var(--pico-muted-border-color);
    }
</style>