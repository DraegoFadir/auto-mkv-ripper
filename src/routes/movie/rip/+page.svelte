<script lang="ts">
    import { app, type DiscType, type Progress, type TitleMap } from "$lib/app.svelte";
    import { onMount } from "svelte";
    import MediaCardComponent from "../../../components/MediaCardComponent.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { settings } from "$lib/settings.svelte";
    import type { Sftp } from "../../../bindings/Sftp";

    onMount(beginRip)

    async function beginRip () {
        for (const title of app.titlesMapped) {
            title.ripProgress = { current: 0, max: 1};
            const unlisten = await listen<Progress>("rip-progress", (e) => {
                title.ripProgress = e.payload;
            });

            await invoke("rip_disc", { titleIndex: title.title.index })
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

        const quality = quality_map[app.discType];

        const sftp: Sftp = {
            local_path: `${settings.output_directory}/${map.title.file_name}`,
            remote_path: `${settings.sftp_movie_path}/${name}`,
            file_path: `${name} - ${quality}.mkv`
        }

        // If you find this setting, cool
        // This is not going to be officially documented but was made to fit how i upload
        if(settings.sftp_movie_path.includes("~split")) {
            sftp.remote_path = sftp.remote_path.replace("~split", `/_${app.discType}`)
        }

        const unlisten = await listen<Progress>("sftp-progress", (e) => {
            map.sftpProgress = e.payload;
        });

        await invoke("send_sftp", { sftp })
        unlisten();
    }

</script>

<h1>Rip</h1>

{#each app.titlesMapped as title(title.media.id)}
    <MediaCardComponent media={title.media}>
        Ripping
        <progress value={title.ripProgress?.current ?? 0} max={title.ripProgress?.max || 1}></progress>
        Uploading
        <progress value={title.sftpProgress?.current ?? 0} max={title.sftpProgress?.max || 1}></progress>
    </MediaCardComponent>
{/each}