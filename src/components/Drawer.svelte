<script lang="ts">
    import type { Snippet } from "svelte";

    let {open = $bindable(false), title, children}: {open?: boolean, title: string, children: Snippet} = $props()

    let dialog: HTMLDialogElement;

    $effect(() => {
        if (open && !dialog.open) 
            dialog.showModal();

        if (!open && dialog.open)
            dialog.close();
    })
</script>

<dialog
    bind:this={dialog}
    class="drawer"
    onclose={() => (open = false)}
    onclick={(e) => e.target === dialog && (open = false)}
>
    <article>
        <header>
            <button aria-label="Close" class="close" onclick={() => (open = false)}></button>
            <strong>{title}</strong>
        </header>
        {@render children()}
    </article>
</dialog>

<style>
    .drawer {
        justify-content: flex-end;
        align-items: stretch;
    }

    .drawer article {
        margin: 0;
        width: min(400px, 90vw);
        height: 100%;
        max-height: none;
        border-radius: 0;
        overflow-y: auto;
        animation: slide-in 2.0s ease-out;
    }

    @keyframes slide-in {
        from { margin-right: -400; }
        to { margin-right: 0; }
    }
</style>