import { goto } from "$app/navigation";
import type { Title } from "../bindings/Title";
import type { MediaResponse } from "../types/MediaResponse";

export type MediaType = "movie" | "tv-show" | "anime"
export type DiscType = "dvd" | "bluray" | "4k"
export type Step = "scan" | "title-mapping" | "rip" | null

class AppState {
    mediaType: MediaType = $state<MediaType>("movie");
    discType: DiscType = $state<DiscType>("dvd");
    mediaSelected: MediaResponse[] = $state<MediaResponse[]>([]);
    titlesSelected: Title[] = $state<Title[]>([]);
    nextStep: Step = $state<Step>(null);

    next = () => {
        if(this.mediaSelected.length < 1 && this.nextStep === "scan") {
            return;
        }

        if(this.titlesSelected.length < 1 && this.nextStep === "title-mapping") {
            return;
        }

        let route = "";
        if (this.nextStep && this.nextStep !== "scan") {
            route = this.nextStep;
        }

        goto(`/${this.mediaType}/${route}`)
    }

    reset = () => {
        // Probably need to clear everything back to defaults

        // go back to search page
        goto("/");
    }

    nextDisabled = $derived.by(() => {
        if(this.mediaSelected.length < 1 && this.nextStep == "scan") {
            return true;
        }

        if(this.titlesSelected.length < 1 && this.nextStep == "title-mapping") {
            return true;
        }

        return false;
    });
}

export const app = new AppState();