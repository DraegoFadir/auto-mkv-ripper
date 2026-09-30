import { load } from "@tauri-apps/plugin-store";

import type { Settings } from "../bindings/Settings";

const defaults: Settings = {
    tmdb_api_key: "",
    tvdb_api_key: "",
    tvdb_api_pin: "",
    makemkv_path: "",
    output_directory: "",
    sftp_hostname: "",
    sftp_username: "",
    sftp_password: "",
    sftp_movie_path: "",
    sftp_tvshow_path: "",
    sftp_anime_path: ""
}

export const settings = $state<Settings>({ ...defaults });

const storePromise = load("settings.json", { defaults: {}, autoSave: true });

export async function loadSettings() {
    const store = await storePromise;
    Object.assign(settings, defaults, await store.get<Settings>("settings"));
}

export async function saveSettings() {
    const store = await storePromise;
    await store.set("settings", $state.snapshot(settings));
}