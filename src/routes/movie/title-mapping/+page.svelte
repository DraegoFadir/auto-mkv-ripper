<script lang="ts">
    import { app } from "$lib/app.svelte";
    import type { Title } from "../../../bindings/Title";
    import MediaCardComponent from "../../../components/MediaCardComponent.svelte";

    app.state.currentStep = "title-mapping";

    function mapTitle(title: Title, id: number) {
        let movie = getMovieData(id);
        if(!movie) {
            return;
        }

        let existingMediaIndex = app.state.titlesMapped.findIndex((x) => x.media.id === id);
        if(existingMediaIndex >= 0) {

            // Movie already belongs to this index, return
            if(app.state.titlesMapped[existingMediaIndex].title.index === title.index) {
                return;
            }

            app.state.titlesMapped.splice(existingMediaIndex, 1);
        }

        let existingIndex = app.state.titlesMapped.findIndex((x) => x.title.index === title.index);
        if(existingIndex >= 0) {

            // If same index, same movie, return
            if(app.state.titlesMapped[existingIndex].media.id === movie.id) {
                return;
            }

            // If same index, different movie, swap & return
            app.state.titlesMapped[existingIndex].media = movie;
            return;
        }

        app.state.titlesMapped.push({
            title,
            media: movie
        })
    }

    function getMappedTitle(index: number) {
        return app.state.titlesMapped.find((x) => x.title.index === index);
    }

    function getMappedMedia(id: number) {
        return app.state.titlesMapped.find((x) => x.media.id === id);
    }

    function getMovieData(id: number) {
        return app.state.mediaSelected.find((x) => x.id === id);
    }

</script>

<h1>Title Mapping</h1>
<div class="row">
    <article class="column-half">
        <header>Selected Movies</header>
        {#each app.state.mediaSelected as movie(movie.id)}
            {#if !getMappedMedia(movie.id)}
                <MediaCardComponent media={movie} draggable ondragstart={(e) => e.dataTransfer?.setData('text/plain', String(movie.id))} />
            {/if}
        {/each}
    </article>

    <div class="col grow">
        {#each app.state.titlesSelected as title(title.index)}
            <article ondrop={(e) => {
                e.preventDefault();
                const id = Number(e.dataTransfer?.getData('text/plain'));
                mapTitle(title, id)
            }} ondragover={(e) => e.preventDefault()}>
                <header>Title: {title.index}</header>
                {#if getMappedTitle(title.index)}
                    <MediaCardComponent media={getMappedTitle(title.index)!.media} draggable ondragstart={(e) => e.dataTransfer?.setData('text/plain', String(getMappedTitle(title.index)!.media.id))} />
                {/if}
            </article>
        {/each}
    </div>
</div>

<style>
    .row {
        display: flex;
        gap: 1rem;
        align-items: flex-start;
    }

    .col {
        display: flex;
        flex-direction: column;
    }

    .grow {
        flex: 1;
    }

    .column-half {
        width: 50%
    }
</style>