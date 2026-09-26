<script lang="ts">    
    import { invoke } from "@tauri-apps/api/core";

    import { app } from "$lib/app.svelte";

    let loading: boolean = $state(false);

    async function startScan() {
        loading = true;
        try {
            let result = await invoke("scan_disc", { })
        } catch(e) {
            console.log("Error", e)
        }
        loading = false;
    }
</script>

<h2>Select Disc Type</h2>
<div role="group">
    <button class={ app.discType === "dvd" ? "primary" : "outline" } onclick={() => app.discType = "dvd"} disabled={loading}>DVD</button>
    <button class={ app.discType === "bluray" ? "primary" : "outline" } onclick={() => app.discType = "bluray"} disabled={loading}>BluRay</button>
    <button class={ app.discType === "4k" ? "primary" : "outline" } onclick={() => app.discType = "4k"} disabled={loading}>4k UltraHD</button>
</div>

{#if !loading}
    <input type="button" class="secondary" value="Start Scan" onclick={startScan} />
{/if}