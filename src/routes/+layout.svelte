<script lang="ts">
    import "@picocss/pico/css/pico.indigo.min.css"
    import {onMount} from "svelte";
    import { Save, Settings } from "@lucide/svelte";

    import DrawerComponent from "../components/DrawerComponent.svelte";
    import SettingsFormComponent from "../components/SettingsFormComponent.svelte";
    import { loadSettings, settings } from "../lib/settings.svelte";
    import { app } from "../lib/app.svelte";

    interface FormHandle {
        submit: () => void;
    }

    onMount(loadSettings);

    $effect(() => {
        const mediaSelected = app.mediaSelected;
        if(mediaSelected.length < 1) {
            app.reset();
        }
    });

    let { children } = $props();
    let settingsOpen: boolean = $state(false);
    let settingsForm: FormHandle;
    let saving: boolean = $state(false);

    function openSettings(event: any) {
        event.preventDefault();

        settingsOpen = !settingsOpen;
    }
</script>

<header class="container-fluid">
    <nav>
        <ul>
            <li><h1>Auto MKV Ripper</h1></li>
        </ul>
        <ul>
            <li>
                <a href="/" aria-label="Settings" onclick={openSettings}>
                    <Settings size={50} />
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

            {#if app.nextStep == "scan" && app.mediaSelected.length > 0}
                <ul>
                    <li><strong>Media Selected: ({app.mediaSelected.length})</strong></li>
                </ul>
            {:else if app.nextStep == "scan"}
                <ul>
                    <li><strong>Please select media to continue</strong></li>
                </ul>
            {/if}

            {#if app.nextStep == "title-mapping" && app.titlesSelected.length > 0}
                <ul>
                    <li><strong>Titles Selected: ({app.titlesSelected.length})</strong></li>
                </ul>
            {:else if app.nextStep == "title-mapping"}
                <ul>
                    <li><strong>Please select title to continue</strong></li>
                </ul>
            {/if}

            <ul>
                <li><button disabled={app.nextDisabled} onclick={() => app.next()}>Next</button></li>
            </ul>
        </nav>
</footer>

<DrawerComponent bind:open={settingsOpen} title="Settings">
    <SettingsFormComponent bind:saving bind:this={settingsForm} />
    {#snippet footer()}
        <button class="save-btn" type="button" onclick={() => settingsForm.submit()} disabled={saving} aria-busy={saving}>
            {#if !saving} Save <Save size={24} /> {/if}
        </button>
    {/snippet}
</DrawerComponent>

<style>
    h1 {
        margin-bottom: 0;
    }

    .save-btn {
        margin: 0;
        width: 100%;
    }

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