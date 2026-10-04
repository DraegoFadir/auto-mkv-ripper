use anyhow_tauri::{TAResult, IntoTAResult};
use tauri::AppHandle;
use reqwest;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::settings::Settings;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MovieDetails {
    id: i32,
    original_title: String,
    poster_path: String,
    overview: String,
    release_date: String,
}

#[tauri::command]
pub async fn get_tmdb(app: AppHandle, movie_id: i32) -> TAResult<MovieDetails> {
    let settings: Settings = Settings::load(&app)?;
 
    let url: String = format!("https://api.themoviedb.org/3/movie/{movie_id}");
    let api_key: String = settings.tmdb_api_key()?;

    let client: reqwest::Client = reqwest::Client::new();
    let res: reqwest::Response = client.get(url)
        .query(&[("api_key", api_key)])
        .send()
        .await.into_ta_result()?;

    let body: MovieDetails = res.json::<MovieDetails>().await.into_ta_result()?;

    Ok(body)
}