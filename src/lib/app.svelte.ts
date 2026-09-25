import { MediaResponse } from "../types/MediaResponse";

export type MediaType = "movie" | "tv-show" | "anime"

class AppState {
    mediaType: MediaType = $state<MediaType>("movie");
    mediaSelected: MediaResponse[] = $state<MediaResponse[]>([])
}

export const app = new AppState();