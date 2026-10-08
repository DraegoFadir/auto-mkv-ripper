pub mod service;
mod response;

use anyhow_tauri::TAResult;
use tauri::AppHandle;
use tauri::Manager;

use crate::tmdb::service::TMDBService;
use crate::{models::Media, settings::Settings };

#[tauri::command]
pub async fn search_tmdb(app: AppHandle, query: String) -> TAResult<Vec<Media>> {
    let settings: Settings = Settings::load(&app)?;
    let api_key: String = settings.tmdb_api_key()?;

    app.state::<TMDBService>().get_by_search(&query, &api_key).await
}

#[tauri::command]
pub async fn get_tmdb_by_id(app: AppHandle, tmdb_id: u32) -> TAResult<Vec<Media>> {
    let settings: Settings = Settings::load(&app)?;
    let api_key: String = settings.tmdb_api_key()?;

    app.state::<TMDBService>().get_by_id(tmdb_id, &api_key).await
}