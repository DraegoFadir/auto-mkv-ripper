use anyhow_tauri::{IntoTAResult, TAResult};
use dotenvy_macro::dotenv;
use reqwest::{Client, Response};
use serde::{Deserialize, de::DeserializeOwned};
use tokio::sync::Mutex;

use crate::{models::Media, tvdb::response::{TVDBResponse, TVDBResponseArray}};

#[derive(Deserialize)]
struct ApiResponse<T> {
    data: T,
}
#[derive(Deserialize)]
struct LoginResponse {
    token: String
}

pub struct TVDBService {
    client: Client,
    base_url: String,
    token: tokio::sync::Mutex<Option<String>>
}

impl TVDBService {
    pub fn new(client: Client) -> Self {
        Self { client, token: Mutex::new(None), base_url: "https://api4.thetvdb.com/v4".into() }
    }

    async fn get_token(&self) -> TAResult<String> {
        let mut guard = self.token.lock().await;

        if let Some(v) = guard.as_ref() {
            Ok(v.clone())
        } else {
            let client = &self.client;
            let url = format!("{base}/login", base = self.base_url);
            let res: Response = client.post(url)
                .json(&serde_json::json!({"apiKey": dotenv!("TVDB_API_KEY")}))
                .send()
                .await
                .into_ta_result()?;

            let login = res.json::<ApiResponse<LoginResponse>>().await.into_ta_result()?;
            *guard = Some(login.data.token.clone());
            Ok(login.data.token)
        }
    }
    
    async fn get<T: DeserializeOwned>(&self, path: &str, params: &[(&str, &str)]) -> TAResult<T> {
        let token = self.get_token().await?;
        let client = &self.client;
        let url = format!("{base}{path}", base = self.base_url);
        let res: Response = client.get(url)
            .header("Authorization", format!("Bearer {token}"))
            .query(params)
            .send()
            .await
            .and_then(|r| r.error_for_status())
            .into_ta_result()?;

        let body: String = res.text().await.into_ta_result()?;
        serde_json::from_str::<T>(&body).into_ta_result()
    }

    pub async fn get_by_search(&self, query: &str) -> TAResult<Vec<Media>> {
        let response = self.get::<TVDBResponseArray>("/search", &[("query", query), ("type", "series")]).await?;

        Ok(response.into())
    }

    pub async fn get_by_id(&self, id: u32) -> TAResult<Vec<Media>> {
        let response = self.get::<ApiResponse<TVDBResponse>>(&format!("/series/{id}"), &[]).await?;

        Ok(vec![response.data.into()])
    }

    pub async fn get_extended(&self, id: &str) -> TAResult<Vec<Media>> {
        let response = self.get::<ApiResponse<TVDBResponse>>(&format!("/series/{id}/extended"), &[("meta", "episodes")]).await?;
        Ok(vec![response.data.into()])
    }
}