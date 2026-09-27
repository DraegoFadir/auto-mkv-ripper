import { goto } from "$app/navigation";
import type { Title } from "../bindings/Title";
import type { MediaResponse } from "../types/MediaResponse";

export type MediaType = "movie" | "tv-show" | "anime"
export type DiscType = "dvd" | "bluray" | "4k"
export type Step = "scan" | "title-mapping" | "rip" | null

export type Progress = {
    current: number,
    max: number
}

export type TitleMap = {
    index: number;
    media: MediaResponse;
    ripProgress?: Progress;
    sftpProgress?: Progress;
}

class AppState {
    mediaType: MediaType = $state<MediaType>("movie");
    discType: DiscType = $state<DiscType>("dvd");
    mediaSelected: MediaResponse[] = $state<MediaResponse[]>([]);
    titlesSelected: Title[] = $state<Title[]>([]);
    titlesMapped: TitleMap[] = $state<TitleMap[]>([]);
    nextStep: Step = $state<Step>(null);

    next = () => {
        if(this.mediaSelected.length < 1 && this.nextStep === "scan") {
            return;
        }

        console.log(this.titlesSelected);

        if(this.titlesSelected.length < 1 && this.nextStep === "title-mapping") {
            return;
        }

        if(this.titlesSelected.length === 1 && this.nextStep === "title-mapping") {
            this.titlesMapped.push({
                index: this.titlesSelected[0].index,
                media: this.mediaSelected[0]
            });
            this.nextStep = "rip";
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