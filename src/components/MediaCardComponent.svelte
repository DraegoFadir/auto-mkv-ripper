<script lang="ts">
    import type { Snippet } from "svelte";
    import type { HTMLAttributes } from "svelte/elements";
    import type { Media } from "../bindings/Media";

    let { media, alignChildrenEnd = false, children, titleInfo, ...rest } : { media: Media, alignChildrenEnd?: boolean, children?: Snippet, titleInfo?: Snippet } & HTMLAttributes<HTMLElement> = $props();
</script>


<article {...rest}>
    <div class="movie-details">
        {#if media.kind === "Movie"}
            <img src="https://image.tmdb.org/t/p/w342/{media.poster_path}" alt="{media.title}" />
            <div>
                <p>{media.title} ({media.release_date?.substring(0, 4)})</p>
                {@render titleInfo?.()}
            </div>
        {/if}

        {#if media.kind === "Series"}
            <img src="{media.image_url}" alt="{media.name}" />
            <div class="info">
                <p>{media.name} ({media.year})</p>
                {@render titleInfo?.()}
            </div>
        {/if}
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

    .info {
        display: flex;
        flex-direction: column;
        gap: .25rem;
    }
</style>