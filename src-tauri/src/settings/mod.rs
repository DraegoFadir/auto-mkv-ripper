use anyhow_tauri::IntoTAResult;
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
    tmdb_api_key: String,
    tvdb_api_key: String,
    tvdb_api_pin: String,
    makemkv_path: String,
    output_directory: String,
    sftp_hostname: String,
    sftp_username: String,
    sftp_password: String,
    sftp_movie_path: String,
    sftp_tvshow_path: String,
    sftp_anime_path: String
}

// Not used right now but will be used to store sensitive data more securely than a raw text json
pub struct PrivateSettings {
    tmdb_api_key: String,
    tvdb_api_key: String,
    tvdb_api_pin: String,
    sftp_password: String,
}

impl Settings {
    fn require_non_empty(value: &str, label: &str) -> anyhow_tauri::TAResult<String> {
        if value.trim().is_empty() {
            anyhow_tauri::bail!("{label} is not set. Add it in Settings.");
        }

        Ok(value.to_string())
    }

    fn require_non_empty_windows(value: &str, label: &str) -> anyhow_tauri::TAResult<String> {
        if cfg!(target_os = "windows") {
            Self::require_non_empty(value, label)
        } else {
            Ok(value.to_string())
        }
    }

    pub fn load(app: &AppHandle) -> anyhow_tauri::TAResult<Self> {
        let store = app.store(STORE_FILE).into_ta_result()?;
        Ok(store
            .get(STORE_KEY)
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default()
        )
    }

    pub fn tmdb_api_key(&self) -> anyhow_tauri::TAResult<String> {
        Self::require_non_empty(&self.tmdb_api_key, "TMDB API Key")
    }
    pub fn tvdb_api_key(&self) -> anyhow_tauri::TAResult<String> {
        Self::require_non_empty(&self.tvdb_api_key, "TVDB API Key")
    }
    pub fn makemkv_path(&self) -> anyhow_tauri::TAResult<String> {
        Self::require_non_empty_windows(&self.makemkv_path, "MakeMKV Path")
    }
    pub fn output_directory(&self) -> anyhow_tauri::TAResult<String> {
        Self::require_non_empty(&self.output_directory, "Output Directory")
    }
    pub fn sftp_hostname(&self) -> anyhow_tauri::TAResult<String> {
        Self::require_non_empty(&self.sftp_hostname, "SFTP Hostname")
    }
    pub fn sftp_username(&self) -> anyhow_tauri::TAResult<String> {
        Self::require_non_empty(&self.sftp_username, "SFTP Username")
    }
    pub fn sftp_password(&self) -> anyhow_tauri::TAResult<String> {
        Self::require_non_empty(&self.sftp_password, "SFTP Password")
    }
    pub fn sftp_movie_path(&self) -> anyhow_tauri::TAResult<String> {
        Self::require_non_empty(&self.sftp_movie_path, "SFTP Movie Path")
    }
    pub fn sftp_tvshow_path(&self) -> anyhow_tauri::TAResult<String> {
        Self::require_non_empty(&self.sftp_tvshow_path, "SFTP TV Show Path")
    }
    pub fn sftp_anime_path(&self) -> anyhow_tauri::TAResult<String> {
        Self::require_non_empty(&self.sftp_anime_path, "SFTP Anime Path")
    }    
}