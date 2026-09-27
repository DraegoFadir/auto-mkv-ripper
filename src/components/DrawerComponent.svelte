<script lang="ts">
    import type { Snippet } from "svelte";
    import { fly } from "svelte/transition";

    let {open = $bindable(false), title, children, footer}: {open?: boolean, title: string, children: Snippet, footer?: Snippet} = $props()

    let dialog: HTMLDialogElement;

    $effect(() => {
        if (open && !dialog.open) {
            dialog.showModal();
        }
    })
</script>

<dialog
    bind:this={dialog}
    class="drawer"
    onclose={() => (open = false)}
    onclick={(e) => e.target === dialog && (open = false)}
>
    {#if open}
        <article transition:fly={{ x: 600, duration: 500 }} onoutroend={() => {if (!open) dialog.close()}}>
            <header>
                <button aria-label="Close" rel="prev" onclick={() => (open = false)}></button>
                <h2><strong>{title}</strong></h2>
            </header>
            
            <main>
                {@render children()}
            </main>

            {#if footer}
                <footer>
                    {@render footer()}
                </footer>
            {/if}
        </article>
    {/if}
</dialog>

<style>
    .drawer {
        justify-content: flex-end;
        align-items: stretch;
        overflow: hidden;
    }

    .drawer article {
        display: flex;
        flex-direction: column;
        width: min(600px, 90vw);
        height: 100%;

        main {
            flex: 1;
            overflow-y: auto;
            min-height: 0;
            scrollbar-width: thin;
            scrollbar-color: var(--pico-muted-border-color) transparent;
        }
    }
</style>