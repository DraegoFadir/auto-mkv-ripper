<script lang="ts">    
    import { app } from "$lib/app.svelte";
    import type { Title } from "../../bindings/Title";
    import prettyBytes from "pretty-bytes";
    import { rust } from "$lib/rust.svelte";

    app.state.currentStep = "scan";

    let loading: boolean = $state(false);
    let titles: Title[] = $state([])

    const atLimit = $derived(app.state.titlesSelected.length >= app.state.mediaSelected.length && app.state.mediaType === "movie");

    async function startScan() {
        loading = true;

        const minLength = app.state.mediaType === "movie" ? "3600" : "1320"

        const result = await rust<Title[]>("scan_disc", { minLength });

        if(result) {
            titles = result;
        }
        
        loading = false;
    }
</script>



{#if !loading}
    <h2>Select Disc Type</h2>
    <div role="group">
        <button class={ app.state.discType === "dvd" ? "primary" : "outline" } onclick={() => app.state.discType = "dvd"} disabled={loading}>DVD</button>
        <button class={ app.state.discType === "bluray" ? "primary" : "outline" } onclick={() => app.state.discType = "bluray"} disabled={loading}>BluRay</button>
        <button class={ app.state.discType === "4k" ? "primary" : "outline" } onclick={() => app.state.discType = "4k"} disabled={loading}>4k UltraHD</button>
    </div>
    <input type="button" class="secondary" value="Start Scan" onclick={startScan} />

    {#if titles.length > 0}
        <hgroup>
            <h2>Select Titles to Map</h2>
            {#if app.state.mediaType === "movie"}
                <p>{app.state.titlesSelected.length} of {app.state.mediaSelected.length} titles selected</p>
            {:else}
                <p>{app.state.titlesSelected.length} titles selected</p>
            {/if}
        </hgroup>
        <fieldset>
            {#each titles as title}
                <label>
                    <article>
                        <input 
                            type="checkbox" 
                            name="title_{title.index}" 
                            bind:group={app.state.titlesSelected} 
                            value={title}   
                            disabled={atLimit && !app.state.titlesSelected.includes(title)} />
                        <div class="title-info grid">
                            <kbd>Index: {title.index}</kbd>
                            <kbd>Duration: {title.duration}</kbd>
                            <kbd>Chapters: {title.chapters}</kbd>
                            <kbd>Size: {prettyBytes(title.size_bytes ?? 0, { binary: true })}</kbd>
                        </div>
                    </article>
                </label>
            {/each}
        </fieldset>
    {/if}
{:else}
    <h2>Scanning Disc</h2>
    <progress></progress>
{/if}

<style>
    label {
        width: 100% !important;
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    article {
        display: flex;
        align-items: center;
        flex: 1;
        margin: 0;
        gap: 5rem;
    }

    .title-info {
        flex: 1;
        justify-items: end;
    }
</style>