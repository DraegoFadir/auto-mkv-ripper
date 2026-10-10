import { goto } from "$app/navigation";
import type { Media } from "../bindings/Media";
import type { Status } from "../bindings/Status";
import type { Title } from "../bindings/Title";

export type MediaType = "movie" | "tv-show" | "anime"
export type DiscType = "dvd" | "bluray" | "4k"
export type Step = "search" | "scan" | "title-mapping" | "rip" | "finish" | null

export type Progress = {
    current: number,
    max: number
}

export type Episode = {
    season: number | undefined,
    episode: number | undefined
}

export type TitleMap = {
    id: string;
    title: Title;
    media: Media;
    episode?: Episode;
    ripStatus?: Status;
    ripProgress?: Progress;
    sftpStatus?: Status;
    sftpProgress?: Progress;
    error?: string;
}

export type Alert = {
    message: string,
    type: "success" | "error";
} | null;

type AppStateType = {
    mediaType: MediaType,
    discType: DiscType
    mediaSelected: Media[]
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
    validator: (() => string | undefined) | undefined;
    next = () => {
        this.resetAlert();

        const error = this.validator?.();
        if(error) {
            this.setAlert({ message: error, type: "error" });
            return;
        }
        
        // Search -> Scan -> Title Mapping -> Rip -> Finish
        switch(this.state.currentStep) {
            case "search": {
                if (this.state.mediaSelected.length > 0) {
                    return goto("/scan");
                }
            }
            case "scan": {
                if (this.state.titlesSelected.length < 1) {
                    return;
                }

                if(this.state.mediaType === "movie") {
                    if (this.state.titlesSelected.length === 1 && this.state.mediaSelected.length === 1) {
                        this.state.titlesMapped = defaults().titlesMapped;

                        // Only 1 media selected, Just map to 1 title
                        this.state.titlesMapped = [{
                            id: crypto.randomUUID(),
                            title: this.state.titlesSelected[0],
                            media: this.state.mediaSelected[0],
                        }];
                        return goto("/rip");
                    }
                }
                
                return this.go("title-mapping");
            }
            case "title-mapping": {
                return goto("/rip");
            }
            case "rip": {
                const isRipping = this.state.titlesMapped.some((x) => x.ripStatus !== "done" && x.ripStatus !== "failed");
                const isUploading = this.state.titlesMapped.some((x) => x.ripStatus === "done" && x.sftpStatus !== "done" && x.sftpStatus !== "failed");

                if (isRipping || isUploading) {
                    return;
                }

                return this.reset();
            }
            default:
                return this.reset();
        }
    }

    reset = async () => { 
        await goto("/"); 
        this.state = defaults(); 
    }
    
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