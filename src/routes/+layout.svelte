<script lang="ts">
    import "@picocss/pico/css/pico.violet.min.css"
    import {onMount} from "svelte";
    import { ChevronRight, RotateCcw, Save, Settings } from "@lucide/svelte";

    import DrawerComponent from "../components/DrawerComponent.svelte";
    import SettingsFormComponent from "../components/SettingsFormComponent.svelte";
    import { loadSettings } from "../lib/settings.svelte";
    import { app } from "../lib/app.svelte";

    interface FormHandle {
        submit: () => void;
    }

    // Trying to prevent back/forward navigation which could break processes like scan/rip/sftp
    window.addEventListener("keydown", (e) => {
        if (e.altKey && (e.key === "ArrowLeft" || e.key === "ArrowRight")) e.preventDefault();
    });
    window.addEventListener("mouseup", (e) => {
        if (e.button === 3 || e.button === 4) e.preventDefault();
    });

    onMount(loadSettings);

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

{#if !settingsOpen}
    <div class="alert-wrapper">
        <p class="alert {app.state.alert?.type}">
            {app.state.alert?.message}
        </p>
    </div>
{/if}

<footer class="container-fluid flex-between">
    <button onclick={() => app.reset()}><RotateCcw /> Reset</button>
    <button onclick={() => app.next()}>{app.state.currentStep === "rip" ? "Finish" : "Continue"} <ChevronRight /></button>
</footer>

<DrawerComponent bind:open={settingsOpen} title="Settings">
    <SettingsFormComponent bind:saving bind:this={settingsForm} />
    {#snippet footer()}
        <div class="settings-footer">
            <div>
                {#if app.state.alert}
                    <p class="{app.state.alert.type}">
                        {app.state.alert.message}
                    </p>
                {/if}
            </div>
            <button class="save-btn" type="button" onclick={() => settingsForm.submit()} disabled={saving} aria-busy={saving}>
                {#if !saving} Save <Save size={24} /> {/if}
            </button>
        </div>
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

    footer {
        text-align: end;
    }

    .alert-wrapper {
        margin-left: 3rem;
        width: fit-content;
        max-width: 40%;
    }

    .settings-footer {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 1rem;
    }

    .settings-footer > * {
        flex: 1;
        min-width: 0;
        width: 100%;
    }

    .flex-between {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .success,
    .error {
        text-align: left;
        margin: 0;
    }

    .alert {
        text-align: left;
        margin: 0;
        box-shadow: 0 0.25rem 0.75rem rgba(0, 0, 0, 0.12);
        border-radius: var(--pico-border-radius);
        padding: var(--pico-form-element-spacing-vertical) var(--pico-form-element-spacing-horizontal);  
        color: var(--pico-color);
    }

    .alert.error {
        border: var(--pico-border-width) solid var(--pico-form-element-invalid-border-color);
        background-color: color-mix(in srgb, var(--pico-form-element-invalid-border-color) 35%, var(--pico-card-background-color));
    }

    .alert.success {  
        border: var(--pico-border-width) solid var(--pico-form-element-valid-border-color);
        background-color: color-mix(in srgb, var(--pico-form-element-valid-border-color) 35%, var(--pico-card-background-color));  
    }

    :global(body) {
        display: flex;
        flex-direction: column;
        height: 100vh;
        gap: .5rem;
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

    :global(.success) {
        color: var(--pico-ins-color);
    }

    :global(.error) {
        color: var(--pico-del-color);
    }

</style>