use tauri::AppHandle;
use reqwest;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::settings::Settings;

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MovieDetails {
    original_title: String,
    poster_path: String
}

#[tauri::command]
pub async fn get_tmdb(app: AppHandle, movie_id: i32) -> Result<MovieDetails, String> {
    let settings: Settings = Settings::load(&app)?;

    if(settings.tmdb_api_key.is_empty()) {
        return Err("TMDB API Key is not set. Add it in Settings.".into());
    }

    let url: String = format!("https://api.themoviedb.org/3/movie/{movie_id}");
    let api_key: &str = settings.tmdb_api_key.as_str();

    let client: reqwest::Client = reqwest::Client::new();
    let res: reqwest::Response = client.get(url)
        .query(&[("api_key", api_key)])
        .send()
        .await
        .map_err(|e: reqwest::Error| e.to_string())?;

    let body: MovieDetails = res.json::<MovieDetails>().await.map_err(|e: reqwest::Error| e.to_string())?;
    println!("{:?}", body.original_title);
    Ok(body)
}