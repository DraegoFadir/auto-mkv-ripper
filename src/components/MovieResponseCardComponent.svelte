<script lang="ts">
    import { MediaResponse } from "../types/MediaResponse";


    let media: MediaResponse = $props();

</script>


<article>
    <header>{media.title} ({media.release_date})</header>
    <div class="movie_details">
        <img src="{media.poster_path}" alt="{media.title}" />
        <p>{media.overview}</p>
    </div>
    <footer>
        <button type="button" class="outline">Select Movie</button>
    </footer>
</article>