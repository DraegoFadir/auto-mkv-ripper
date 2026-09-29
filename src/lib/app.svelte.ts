import { goto } from "$app/navigation";
import type { Title } from "../bindings/Title";
import type { MediaResponse } from "../types/MediaResponse";

export type MediaType = "movie" | "tv-show" | "anime"
export type DiscType = "dvd" | "bluray" | "4k"
export type Step = "scan" | "title-mapping" | "rip" | "finish" | null

export type Progress = {
    current: number,
    max: number
}

export type TitleMap = {
    title: Title;
    media: MediaResponse;
    ripProgress?: Progress;
    sftpProgress?: Progress;
}

type AppStateType = {
    mediaType: MediaType,
    discType: DiscType
    mediaSelected: MediaResponse[]
    titlesSelected: Title[]
    titlesMapped: TitleMap[]
    nextStep: Step
}

const defaults = (): AppStateType => ({
    mediaType: "movie",
    discType: "dvd",
    mediaSelected: [],
    titlesSelected: [],
    titlesMapped: [],
    nextStep: null
}); 

class AppState {
    state: AppStateType = $state<AppStateType>(defaults());

    getNextString = () => {
        switch(this.state.nextStep) {
            case "scan":
                return "Continue to Disc Scan"
            case "title-mapping":
                return "Continue to Title Mapping"
            case "rip":
                return "Continue to Rip & Upload"
            case "finish":
                return "Finish"
            default:
                return "Continue"
        }
    }
 
    next = () => {
        if(this.state.mediaSelected.length < 1 && this.state.nextStep === "scan") {
            return;
        }

        if(this.state.titlesSelected.length < 1 && this.state.nextStep === "title-mapping") {
            return;
        }

        if(this.state.titlesSelected.length === 1 && this.state.nextStep === "title-mapping") {
            this.state.titlesMapped.push({
                title: this.state.titlesSelected[0],
                media: this.state.mediaSelected[0]
            });
            this.state.nextStep = "rip";
        }

        let route = "";
        if (this.state.nextStep && this.state.nextStep !== "scan") {
            route = this.state.nextStep;
        }
        if(this.state.nextStep && this.state.nextStep === "finish") {
            return this.reset();
        }

        goto(`/${this.state.mediaType}/${route}`)
    }

    reset = () => { this.state = defaults(); goto("/"); }

    nextDisabled = $derived.by(() => {
        
        if(this.state.mediaSelected.length < 1 && this.state.nextStep == "scan") {
            return true;
        }

        if(this.state.titlesSelected.length < 1 && this.state.nextStep == "title-mapping") {
            return true;
        }

        if(this.state.titlesMapped.find(x => x.ripProgress?.current !== x.ripProgress?.max)) {
            return true;
        }

        if(this.state.titlesMapped.find(x => x.sftpProgress?.current !== x.sftpProgress?.max)) {
            return true;
        }

        return false;
    });
}

export const app = new AppState();