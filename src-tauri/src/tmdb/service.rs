use anyhow_tauri::{IntoTAResult, TAResult};
use dotenvy_macro::dotenv;
use serde::de::DeserializeOwned;

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
            .query(&[("api_key", dotenv!("TMDB_API_KEY"))])
            .send()
            .await
            .and_then(|r| r.error_for_status())
            .into_ta_result()?;

        res.json::<T>().await.into_ta_result()
    }

    pub async fn get_by_search(&self, query: &str) -> TAResult<Vec<Media>> {
        let response = self.get::<TMDBResponseArray>("/search/movie", &[("query", query)]).await?;

        Ok(response.into())
    }

    pub async fn get_by_id(&self, id: u32) -> TAResult<Vec<Media>> {
        let response = self.get::<TMDBResponse>(&format!("/movie/{id}"), &[]).await?;

        Ok(vec![response.into()])
    }


}