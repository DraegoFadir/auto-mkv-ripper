<script lang="ts">
    import {settings, saveSettings} from "../lib/settings.svelte";

    let status = $state("");

    async function save(e: SubmitEvent) {
        e.preventDefault();
        try {
            await saveSettings();
            status = "Saved";
        } catch(err) {
            status = `Error: ${err}`;
        }
    }
</script>

<form onsubmit={save}>
    <fieldset>
        <legend>API Keys</legend>
        <label>
            API Key (TMDB)
            <input type="password" name="tmdb_api_key" placeholder="TMDB API Key" bind:value={settings.tmdb_api_key} aria-label="TMDB API Key" />
        </label>
    </fieldset>

    {#if status}<small>{status}</small>{/if}
    <input type="submit" value="Save" />
</form>