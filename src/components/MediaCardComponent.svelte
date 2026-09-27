<script lang="ts">
    import type { Snippet } from "svelte";
    import type { HTMLAttributes } from "svelte/elements";
    import type { MediaResponse } from "../types/MediaResponse";

    let { media, children, ...rest } : { media: MediaResponse, children?: Snippet } & HTMLAttributes<HTMLElement> = $props();
</script>


<article {...rest}>
    <header>{media.title} ({media.release_date})</header>
    <div class="movie-details">
        <img src="{media.poster_path}" alt="{media.title}" />
        <p>{media.overview}</p>
    </div>
    {@render children?.()}
</article>

<style>
    article img {
        aspect-ratio: 2/3;
        height: 100%;
        max-height: 200px;
    }

    .movie-details {
        display: flex;
        gap: 1.25rem;
    }
</style>