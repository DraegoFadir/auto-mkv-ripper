use anyhow_tauri::TAResult;
use tauri::{AppHandle, Manager};

use crate::{models::Media, tvdb::service::TVDBService};

pub mod service;
mod response;

#[tauri::command]
pub async fn search_tvdb(app: AppHandle, query: String) -> TAResult<Vec<Media>> {
    app.state::<TVDBService>().get_by_search(&query).await
}