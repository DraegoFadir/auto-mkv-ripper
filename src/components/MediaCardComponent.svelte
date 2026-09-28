<script lang="ts">
    import type { Snippet } from "svelte";
    import type { HTMLAttributes } from "svelte/elements";
    import type { MediaResponse } from "../types/MediaResponse";

    let { media, children, ...rest } : { media: MediaResponse, children?: Snippet } & HTMLAttributes<HTMLElement> = $props();
</script>


<article {...rest}>
    <div class="movie-details">
        <img src="{media.poster_path}" alt="{media.title}" />
        {media.title} ({media.release_date.getFullYear()})
        <div>
            {@render children?.()}
        </div>
    </div>
</article>

<style>
    article img {
        aspect-ratio: 2/3;
        height: 100%;
        max-height: 125px;
    }

    .movie-details {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }
</style>