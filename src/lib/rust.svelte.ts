import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
import { app } from "./app.svelte";

export async function rust<T> (cmd: string, args?: InvokeArgs): Promise<T | null> {
    try {
        const result: T = await invoke(cmd, args);
        return result;
    } catch(e) {
        const message = typeof e === "string" ? e : "An unknown error has occurred";
        app.setAlert({
            message,
            type: 'error'
        });
        console.error(e);
    }

    return null;
}