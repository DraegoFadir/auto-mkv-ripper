<script lang="ts">    
    import { invoke } from "@tauri-apps/api/core";

    import { app } from "$lib/app.svelte";
    import type { Title } from "../../bindings/Title";
    import prettyBytes from "pretty-bytes";

    app.nextStep = "title-mapping";

    let loading: boolean = $state(false);
    let titles: Title[] = $state([])

    async function startScan() {
        loading = true;
        try {
            let result: Title[] = await invoke("scan_disc", { })
            console.log(result);
            titles = result;
        } catch(e) {
            console.log("Error", e)
        }
        loading = false;
    }
</script>



{#if !loading}
    <h2>Select Disc Type</h2>
    <div role="group">
        <button class={ app.discType === "dvd" ? "primary" : "outline" } onclick={() => app.discType = "dvd"} disabled={loading}>DVD</button>
        <button class={ app.discType === "bluray" ? "primary" : "outline" } onclick={() => app.discType = "bluray"} disabled={loading}>BluRay</button>
        <button class={ app.discType === "4k" ? "primary" : "outline" } onclick={() => app.discType = "4k"} disabled={loading}>4k UltraHD</button>
    </div>
    <input type="button" class="secondary" value="Start Scan" onclick={startScan} />

    {#if titles}
    <h2>Select Titles to Map</h2>
    <fieldset>
        {#each titles as title}
            <label>
                <article>
                    <input type="checkbox" name="title_{title.index}" bind:group={app.titlesSelected} value={title.index}  />
                    <table>
                        <thead>
                            <tr>
                                <th scope="col">Index</th>
                                <th scope="col">Duration</th>
                                <th scope="col">Chapters</th>
                                <th scope="col">Size (GB)</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <th scope="row">{title.index}</th>
                                <td>{title.duration}</td>
                                <td>{title.chapters}</td>
                                <td>{prettyBytes(title.size_bytes ?? 0, { binary: true })}</td>
                            </tr>
                        </tbody>
                    </table>
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
        width: 100%;
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

    article table {
        margin-bottom: 0;
    }
</style>