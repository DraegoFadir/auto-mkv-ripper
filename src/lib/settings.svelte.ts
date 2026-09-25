import { load } from "@tauri-apps/plugin-store";

import type { Settings } from "../bindings/Settings";

const defaults: Settings = { tmdb_api_key: "" }

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