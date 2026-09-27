<script lang="ts">
    import { open } from '@tauri-apps/plugin-dialog'
    import { Folder, File } from '@lucide/svelte';
    import {settings, saveSettings} from "../lib/settings.svelte";

    let { saving = $bindable(false) }: {saving: boolean} = $props();
    let status = $state("");

    let formRef: HTMLFormElement;
    export function submit(): void {
        formRef.requestSubmit();
    }

    async function save(e: SubmitEvent) {
        e.preventDefault();
        saving = true;
        try {
            await saveSettings();
            status = "Saved";
        } catch(err) {
            status = `Error: ${err}`;
        } finally {
            saving = false;
        }
    }

    async function folder(setter: (path: string) => void) {
        const path = await open({ directory: true });
        if (path) {
            console.log(path)
            setter(path);
        }
    }

    async function file(setter: (path: string) => void) {
        const path = await open();
        if (path) {
            console.log(path)
            setter(path);
        }
    }
</script>

<form bind:this={formRef} onsubmit={save}>
    <fieldset>
        <legend>API Keys</legend>
        <label>
            API Key (TMDB)
            <input type="password" name="tmdb_api_key" placeholder="TMDB API Key" bind:value={settings.tmdb_api_key} aria-label="TMDB API Key" />
        </label>
    </fieldset>

    <fieldset>
        <legend>MakeMKV</legend>
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
</form>

<style>
    form {
        flex: 1
    }
</style>