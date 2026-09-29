<script lang="ts">
    import { app, type DiscType, type Progress, type TitleMap } from "$lib/app.svelte";
    import { onMount } from "svelte";
    import MediaCardComponent from "../../../components/MediaCardComponent.svelte";
    import { listen } from "@tauri-apps/api/event";
    import { settings } from "$lib/settings.svelte";
    import type { Sftp } from "../../../bindings/Sftp";
    import { rust } from "$lib/rust.svelte";

    app.state.nextStep = "finish";

    onMount(beginRip)

    async function beginRip () {
        for (const title of app.state.titlesMapped) {
            title.ripProgress = { current: 0, max: 1};
            const unlisten = await listen<Progress>("rip-progress", (e) => {
                title.ripProgress = e.payload;
            });

            await rust("rip_disc", { titleIndex: title.title.index })
            unlisten();

            beginSftp(title);
        }
    }

    async function beginSftp(map: TitleMap) {
        const movie = map.media;
        const year = movie.release_date.getFullYear();

        const name = `${movie.title} (${year}) [tmdbid-${movie.id}]`;

        const quality_map: Record<DiscType, String> = {
            "dvd": "480p",
            "bluray": "1080p",
            "4k": "2160p"
        }

        const quality = quality_map[app.state.discType];

        const sftp: Sftp = {
            local_path: `${settings.output_directory}/${map.title.file_name}`,
            remote_path: `${settings.sftp_movie_path}/${name}`,
            file_path: `${name} - ${quality}.mkv`
        }

        // If you find this setting, cool
        // This is not going to be officially documented but was made to fit how i upload
        if(settings.sftp_movie_path.includes("~split")) {
            sftp.remote_path = sftp.remote_path.replace("~split", `/_${app.state.discType}`)
        }

        const unlisten = await listen<Progress>("sftp-progress", (e) => {
            map.sftpProgress = e.payload;
        });

        await rust("send_sftp", { sftp })
        unlisten();
    }

    function isRunning(progress?: Progress) {
        if(!progress){
            return false;
        }

        return progress.current > 0;
    }

    function isComplete(progress?: Progress) {
        if(!progress) {
            return false;
        }

        return progress.current === progress.max;
    }

</script>

<h1>Rip</h1>

{#snippet progress(p?: Progress)}
    {#if !p}
        <p class="danger">progress error</p>
    {:else if p.current > 0}
        <progress value={p.current} max={p.max}></progress>
    {:else}
        <progress></progress>
    {/if}
{/snippet}

{#each app.state.titlesMapped as title(title.media.id)}
    <MediaCardComponent media={title.media}>
        {#if isComplete(title.ripProgress)}
            <p class="success">Rip Finished</p>
        {:else if isRunning(title.ripProgress)}
            Ripping
            {@render progress(title.ripProgress)}
        {:else}
            <p>Rip Not Started</p>
        {/if}



        {#if isComplete(title.sftpProgress)}
            <p class="success">Upload Finished</p>
        {:else if isRunning(title.sftpProgress)}
            Uploading
            {@render progress(title.sftpProgress)}
        {:else}
            <p>Upload Not Started</p>
        {/if}
        
    </MediaCardComponent>
{/each}

