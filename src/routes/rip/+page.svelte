<script lang="ts">
    import { app, type DiscType, type Progress, type TitleMap } from "$lib/app.svelte";
    import { listen } from "@tauri-apps/api/event";
    import { settings } from "$lib/settings.svelte";
    import { rust } from "$lib/rust.svelte";
    import type { TitleStatus } from "../../bindings/TitleStatus";
    import type { Sftp } from "../../bindings/Sftp";
    import MediaCardComponent from "../../components/MediaCardComponent.svelte";
    import type { TMDBData } from "../../bindings/TMDBData";
    import type { TVDBData } from "../../bindings/TVDBData";

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

                const minLength = app.state.mediaType === "movie" ? "3600" : "1320"
                ok = await rust<boolean>("rip_disc", { titleIndex: title.title.index, minLength })
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

        const media = map.media;


        const quality_map: Record<DiscType, string> = {
            "dvd": "480p",
            "bluray": "1080p",
            "4k": "2160p"
        }

        const quality = quality_map[app.state.discType];

        // TODO: Add settings access to rust
        let sftp: Sftp | null = null;
        
        const local_path = `${settings.output_directory}/${map.title.file_name}`;

        switch(media.kind) {
            case "Movie": {
                const movie = media as TMDBData;
                const year = movie.release_date?.substring(0, 4) ?? "0000";
                const name = `${movie.title} (${year}) [tmdbid-${movie.id}]`;
                sftp = {
                    local_path,
                    remote_path: `${name}`,
                    file_path: `${name} - ${quality}.mkv`,
                    media_type: media.kind,
                    split_path: `/_${app.state.discType}`
                }
                break;
            }
            case "Series": {
                const series = media as TVDBData;
                const year = series.year;
                const name = `${series.name} (${year}) [tvdbid-${series.id}]`;
                const seasonStr = String(map.episode?.season).padStart(2, "0");
                const episodeStr = String(map.episode?.episode).padStart(2, "0");
                sftp = {
                    local_path,
                    remote_path: `${name}/Season ${seasonStr}`,
                    file_path: `${series.name} - S${seasonStr}E${episodeStr} - ${quality}.mkv`,
                    media_type: media.kind
                }
                break;
            }
            default:
                app.setAlert({ message: "Invalid media kind", type: "error" });
        }

        if(sftp) {
            const unlisten = await listen<Progress>("sftp-progress", (e) => {
                map.sftpProgress = e.payload;
            });

            await rust("send_sftp", { sftp, titleIndex: map.title.index })
            unlisten();
        }
    }

</script>

<hgroup>
    <h1>Rip & Upload</h1>
    <p>Check your mapping, then press start. Ripping, uploading, and Jellyfin naming are handled for you</p>
</hgroup>
<input type="button" value="Start Rip & Upload" onclick={beginRip} disabled={started} />
{#snippet progress(p?: Progress)}
    {#if !p}
        <p class="danger">progress error</p>
    {:else if p.current > 0}
        <progress value={p.current} max={p.max}></progress>
    {:else}
        <progress></progress>
    {/if}
{/snippet}

{#each app.state.titlesMapped as title(title.id)}
    <MediaCardComponent media={title.media}>

        {#snippet titleInfo()}
            <div>
                <kbd>Season {String(title.episode?.season).padStart(2, "0")}</kbd>
                <kbd>Episode {String(title.episode?.episode).padStart(2, "0")}</kbd>
            </div>
        {/snippet}

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

