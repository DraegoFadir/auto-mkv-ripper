<script lang="ts">
    import { app, type DiscType, type Progress, type TitleMap } from "$lib/app.svelte";
    import MediaCardComponent from "../../../components/MediaCardComponent.svelte";
    import { listen } from "@tauri-apps/api/event";
    import { settings } from "$lib/settings.svelte";
    import type { Sftp } from "../../../bindings/Sftp";
    import { rust } from "$lib/rust.svelte";
    import type { TitleStatus } from "../../../bindings/TitleStatus";

    app.state.currentStep = "rip";

    // We only need to start once, disable but start button.
    // TODO: Implement a restart feature for any failed attempts.
    let started = $state(false)
    let uploads: Promise<void>[] = [];

    $effect(() => {
        const unlistenRipStatus = listen<TitleStatus>("rip-status", (e) => {
            let title = app.state.titlesMapped.find((x) => x.title.index === e.payload.title_index);

            if(title){
                title.ripStatus = e.payload.status;
            }
        });

        const unlistenSftpStatus = listen<TitleStatus>("sftp-status", (e) => {
            let title = app.state.titlesMapped.find((x) => x.title.index === e.payload.title_index);

            if(title){
                title.sftpStatus = e.payload.status;
            }
        });

        return () => { 
            unlistenRipStatus.then((fn) => fn()); 
            unlistenSftpStatus.then((fn) => fn()); 
        };
    })

    async function beginRip () {
        if(started)
            return;

        started = true;
        uploads = [];

        for (const title of app.state.titlesMapped) {
            let ok: boolean | null = title.ripStatus === "done";

            // if title was already extracted, move to sftp
            // TODO: Add file check
            if(title.ripStatus !== "done") {
                title.ripProgress = { current: 0, max: 1};
                const unlisten = await listen<Progress>("rip-progress", (e) => {
                    title.ripProgress = e.payload;
                });

                ok = await rust<boolean>("rip_disc", { titleIndex: title.title.index })
                unlisten();
            }

            if(ok){
                uploads.push(beginSftp(title));
            }
        }

        // regardless of rip/sftp pass fail, turn off the started
        await Promise.allSettled(uploads);
        started = false;
    }

    async function beginSftp(map: TitleMap) {
        // sftp is completed, skip it
        // TODO: Add SFTP Check
        if(map.sftpStatus === "done") {
            return;
        }

        const movie = map.media;
        const year = movie.release_date.getFullYear();

        const name = `${movie.title} (${year}) [tmdbid-${movie.id}]`;

        const quality_map: Record<DiscType, String> = {
            "dvd": "480p",
            "bluray": "1080p",
            "4k": "2160p"
        }

        const quality = quality_map[app.state.discType];

        // TODO: Add settings access to rust
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

        await rust("send_sftp", { sftp, titleIndex: map.title.index })
        unlisten();
    }

</script>

<h1>Rip</h1>
<input type="button" value="Start Rip" onclick={beginRip} disabled={started} />
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
        {#if title.ripStatus === "started"}
            Ripping
            {@render progress(title.ripProgress)}
        {:else if title.ripStatus === "done"}
            <p class="success">Rip Finished</p>
        {:else if title.ripStatus === "failed"}
            <p class="error">Rip Failed</p>
        {:else}
            <p>Rip Not Started</p>
        {/if}

        {#if title.sftpStatus === "started"}
            Uploading
            {@render progress(title.sftpProgress)}
        {:else if title.sftpStatus === "done"}
            <p class="success">Upload Finished</p>
        {:else if title.sftpStatus === "failed"}
            <p class="error">Upload Failed</p>
        {:else}
            <p>Upload Not Started</p>
        {/if}

        <input type="button" value="Stop" class="contrast" disabled />
    </MediaCardComponent>
{/each}

