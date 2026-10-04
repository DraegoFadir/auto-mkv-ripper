import { goto } from "$app/navigation";
import type { Status } from "../bindings/Status";
import type { Title } from "../bindings/Title";
import type { MediaResponse } from "../types/MediaResponse";

export type MediaType = "movie" | "tv-show" | "anime"
export type DiscType = "dvd" | "bluray" | "4k"
export type Step = "search" | "scan" | "title-mapping" | "rip" | "finish" | null

export type Progress = {
    current: number,
    max: number
}

export type TitleMap = {
    title: Title;
    media: MediaResponse;
    ripStatus?: Status;
    ripProgress?: Progress;
    sftpStatus?: Status;
    sftpProgress?: Progress;
}

export type Alert = {
    message: string,
    type: "success" | "error";
} | null;

type AppStateType = {
    mediaType: MediaType,
    discType: DiscType
    mediaSelected: MediaResponse[]
    titlesSelected: Title[]
    titlesMapped: TitleMap[]
    currentStep: Step,
    alert: Alert,
}

const defaults = (): AppStateType => ({
    mediaType: "movie",
    discType: "dvd",
    mediaSelected: [],
    titlesSelected: [],
    titlesMapped: [],
    currentStep: null,
    alert: null,
});

class AppState {
    alertTimer: ReturnType<typeof setTimeout> | undefined;
    state: AppStateType = $state<AppStateType>(defaults())
 
    go = (route: Step) => goto(`/${this.state.mediaType}/${route}`);
    next = () => {
        this.resetAlert();
        
        // Search -> Scan -> Title Mapping -> Rip -> Finish
        switch(this.state.currentStep) {
            case "search": {
                if (this.state.mediaSelected.length > 0) {
                    return this.go("scan");
                }
            }
            case "scan": {
                if (this.state.titlesSelected.length < 1) {
                    return;
                }

                if (this.state.mediaSelected.length > 1) {
                    this.state.titlesMapped = defaults().titlesMapped;
                    return this.go("title-mapping");
                }

                // Only 1 media selected, Just map to 1 title
                this.state.titlesMapped = [{
                    title: this.state.titlesSelected[0],
                    media: this.state.mediaSelected[0]
                }];
                return this.go("rip");
            }
            case "title-mapping": {
                if (this.state.titlesMapped.length > 0) {
                    return this.go("rip");
                }
            }
            case "rip": {
                const isRipping = this.state.titlesMapped.some((x) => x.ripStatus !== "done" && x.ripStatus !== "failed");
                const isUploading = this.state.titlesMapped.some((x) => x.sftpStatus !== "done" && x.sftpStatus !== "failed");

                if (isRipping || isUploading) {
                    return;
                }

                return this.reset();
            }
            default:
                return this.reset();
        }
    }

    reset = () => { this.state = defaults(); goto("/"); }
    
    resetAlert = () => { this.state.alert = defaults().alert }
    setAlert = (alert: Alert) => { 
        clearTimeout(this.alertTimer)
        this.state.alert = alert;
        this.alertTimer = setTimeout(() => {
            this.resetAlert();
        }, 5000);
    };
}

export const app = new AppState();