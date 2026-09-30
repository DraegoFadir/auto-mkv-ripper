<script lang="ts">
    import { open } from '@tauri-apps/plugin-dialog'
    import { Folder, File } from '@lucide/svelte';
    import {settings, saveSettings} from "../lib/settings.svelte";
    import { app } from '$lib/app.svelte';

    let { saving = $bindable(false) }: {saving: boolean} = $props();

    let formRef: HTMLFormElement;
    export function submit(): void {
        formRef.requestSubmit();
    }

    async function save(e: SubmitEvent) {
        e.preventDefault();
        saving = true;
        try {
            await saveSettings();
            app.setAlert({
                message: "Saved",
                type: 'success'
            });
        } catch(err) {
            app.setAlert({
                message: "Error saving settings",
                type: 'error'
            });
        } finally {
            saving = false;
        }
    }

    async function folder(setter: (path: string) => void) {
        const path = await open({ directory: true });
        if (path) {
            setter(path);
        }
    }

    async function file(setter: (path: string) => void) {
        const path = await open();
        if (path) {
            setter(path);
        }
    }
</script>

<form bind:this={formRef} onsubmit={save}>
    <fieldset>
        <legend><h3><strong>API Keys</strong></h3></legend>
        <label>
            TMDB Key
            <input type="password" name="tmdb_api_key" placeholder="TMDB API Key" bind:value={settings.tmdb_api_key} aria-label="TMDB API Key" />
        </label>
        <label>
            TVDB Key
            <input type="password" name="tmdb_api_key" placeholder="TMDB API Key" bind:value={settings.tvdb_api_key} aria-label="TVDB API Key" />
        </label>
        <label>
            TVDB Pin (Optional)
            <input type="password" name="tmdb_api_key" placeholder="TMDB API Key" bind:value={settings.tvdb_api_pin} aria-label="TVDB API Pin" />
        </label>
    </fieldset>

    <fieldset>
        <legend><h3><strong>MakeMKV</strong></h3></legend>
        <label>
            MakeMKV Path
            <div role="group">
                <input type="text" readonly placeholder="Select path to MakeMKV" bind:value={settings.makemkv_path} />
                <button type="button" onclick={() => file(p => settings.makemkv_path = p)}>
                    <File size={16} />
                </button>
            </div>
        </label>
        <label>
            Output Directory
            <div role="group">
                <input type="text" readonly placeholder="Select directory to output to" bind:value={settings.output_directory} />
                <button type="button" onclick={() => folder(p => settings.output_directory = p)}>
                    <Folder size={16} />
                </button>
            </div>
        </label>
    </fieldset>

    <fieldset>
        <legend><h3><strong>SFTP</strong></h3></legend>
        <label>
            Hostname
            <input type="text" placeholder="sftp.hostname.com" bind:value={settings.sftp_hostname}>
        </label>
        <label>
            Username
            <input type="text" placeholder="username" bind:value={settings.sftp_username}>
        </label>
        <label>
            Password
            <input type="password" placeholder="password" bind:value={settings.sftp_password}>
        </label>
    </fieldset>
    
    <fieldset>
        <legend><h3><strong>SFTP Path</strong></h3></legend>
        <label>
            Movie
            <input type="text" placeholder="/path/to/movies" bind:value={settings.sftp_movie_path}>
            <small>This is the path to your movie libary from the sftp root.</small>
        </label>
        <label>
            TV Show
            <input type="text" placeholder="/path/to/shows" bind:value={settings.sftp_tvshow_path}>
            <small>This is the path to your tv show libary from the sftp root.</small>
        </label>
        <label>
            Anime
            <input type="text" placeholder="/path/to/anime" bind:value={settings.sftp_anime_path}>
            <small>This is the path to your anime libary from the sftp root.</small>
        </label>
    </fieldset>
</form>

<style>
    form {
        flex: 1
    }
</style>