<script lang="ts">
    import { app, type Progress } from "$lib/app.svelte";
    import { onMount } from "svelte";
    import MediaCardComponent from "../../../components/MediaCardComponent.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    onMount(beginRip)

    async function beginRip () {
        for (const title of app.titlesMapped) {
            title.ripProgress = { current: 0, max: 1};
            const unlisten = await listen<Progress>("rip-progress", (e) => {
                console.log("Listening: ", e.payload);
                title.ripProgress = e.payload;
            });

            await invoke("rip_disc", { titleIndex: title.index })
            unlisten();
        }
    }

</script>

<h1>Rip</h1>

{#each app.titlesMapped as title(title.media.id)}
    <MediaCardComponent media={title.media}>
        Ripping
        <progress value={title.ripProgress?.current ?? 0} max={title.ripProgress?.max || 1}></progress>
    </MediaCardComponent>
{/each}