<script lang="ts">
    import type { Snippet } from "svelte";
    import type { HTMLAttributes } from "svelte/elements";
    import type { MediaResponse } from "../types/MediaResponse";

    let { media, alignChildrenEnd = false, children, ...rest } : { media: MediaResponse, alignChildrenEnd?: boolean, children?: Snippet } & HTMLAttributes<HTMLElement> = $props();
</script>


<article {...rest}>
    <div class="movie-details">
        <img src="{media.poster_path}" alt="{media.title}" />
        <p>{media.title} ({media.release_date.getFullYear()})</p>
        <div class:end={alignChildrenEnd}>
            {@render children?.()}
        </div>
    </div>
</article>

<style>

    article {
        box-shadow: none;
        background: transparent;
        border: 1px solid var(--pico-muted-border-color);
        background: var(--pico-card-sectioning-background-color);
    }

    article img {
        aspect-ratio: 2/3;
        height: 100%;
        max-height: 125px;
    }

    .movie-details {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .movie-details > :first-child {
        flex: 0 0 auto;
    }

    .movie-details > :not(:first-child) {
        flex: 1;
        min-width: 0;
    }

    .end {
        display: flex;
        justify-content: flex-end;
    }
</style>