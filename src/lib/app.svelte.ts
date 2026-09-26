import { goto } from "$app/navigation";
import type { MediaResponse } from "../types/MediaResponse";

export type MediaType = "movie" | "tv-show" | "anime"
export type DiscType = "dvd" | "bluray" | "4k"
export type Step = "scan" | "title-mapping" | "rip" | null

class AppState {
    mediaType: MediaType = $state<MediaType>("movie");
    discType: DiscType = $state<DiscType>("dvd");
    mediaSelected: MediaResponse[] = $state<MediaResponse[]>([])
    nextStep: Step = $state<Step>(null);

    next = () => {
        if(this.mediaSelected.length < 1) {
            return;
        }

        let route = "";
        if (this.nextStep !== "scan") {
            let route = this.nextStep;
        }

        goto(`/${this.mediaType}/${route}`)
    }

    nextDisabled = $derived.by(() => {
        if(this.mediaSelected.length < 1) {
            return true;
        }

        return false;
    });
}

export const app = new AppState();