pub mod service;
mod response;

use anyhow_tauri::TAResult;
use tauri::AppHandle;
use tauri::Manager;

use crate::tmdb::service::TMDBService;
use crate::models::Media;

#[tauri::command]
pub async fn search_tmdb(app: AppHandle, query: String) -> TAResult<Vec<Media>> {
    app.state::<TMDBService>().get_by_search(&query).await
}

#[tauri::command]
pub async fn get_tmdb_by_id(app: AppHandle, tmdb_id: u32) -> TAResult<Vec<Media>> {
    app.state::<TMDBService>().get_by_id(tmdb_id).await
}