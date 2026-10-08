use anyhow_tauri::{IntoTAResult, TAResult};
use serde::de::DeserializeOwned;
use tauri::http::response;

use crate::{models::Media, tmdb::response::{TMDBResponse, TMDBResponseArray}};

pub struct TMDBService {
    client: reqwest::Client,
    base_url: String,
}

impl TMDBService {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client, base_url: "https://api.themoviedb.org/3".into() }
    }
    
    async fn get<T: DeserializeOwned>(&self, path: &str, params: &[(&str, &str)]) -> TAResult<T> {
        let client = &self.client;
        let url = format!("{base}{path}", base = self.base_url);
        let res: reqwest::Response = client.get(url)
            .query(params)
            .send()
            .await
            .and_then(|r| r.error_for_status())
            .into_ta_result()?;

        res.json::<T>().await.into_ta_result()
    }

    pub async fn get_by_search(&self, query: &str, api_key: &str) -> TAResult<Vec<Media>> {
        let response = self.get::<TMDBResponseArray>("/search/movie", &[("api_key", api_key),("query", query)]).await?;

        Ok(response.into())
    }

    pub async fn get_by_id(&self, id: u32, api_key: &str) -> TAResult<Vec<Media>> {
        let response = self.get::<TMDBResponse>(&format!("/movie/{id}"), &[("api_key", api_key)]).await?;

        Ok(vec![response.into()])
    }


}