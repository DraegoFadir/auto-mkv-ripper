use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use ts_rs::TS;

pub const STORE_FILE: &str = "settings.json";
pub const STORE_KEY: &str = "settings";

#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(default)]
pub struct Settings {
    pub tmdb_api_key: String
}

impl Settings {
    pub fn load(app: &AppHandle) -> Result<Self, String> {
        let store = app.store(STORE_FILE).map_err(|e| e.to_string())?;
        Ok(store
            .get(STORE_KEY)
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default()
        )
    }
}