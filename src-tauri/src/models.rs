use anyhow_tauri::TAResult;
use serde::{Deserialize, Deserializer};
use ts_rs::TS;

#[derive(Clone, serde::Serialize, TS)]
#[ts(export)]
pub struct TitleProgress {
    pub title_index: u32,
    pub current: u64,
    pub total: Option<u64>,
    pub max: u64,
}

#[derive(Clone, serde::Serialize, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum Status {
    Started,
    Done,
    Failed
}
#[derive(Clone, serde::Serialize, TS)]
#[ts(export)]
pub struct TitleStatus {
    pub title_index: u32,
    pub status: Status
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, TS)]
#[ts(export)]
pub struct TMDBData {
    pub id: u32,
    pub title: String,
    pub poster_path: Option<String>,
    pub release_date: Option<String>
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, TS)]
#[ts(export)]
pub struct TVDBData {
    pub id: String,
    pub name: Option<String>,
    pub image_url: Option<String>,
    pub year: Option<String>,
    pub seasons: Vec<Season>,
    pub episodes: Vec<Episode>
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct Season {
    id: u32,
    number: u32,
    name: Option<String>,
    #[serde(rename = "type", deserialize_with = "inner_kind")]
    pub season_type: String
}

fn inner_kind<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper {
        r#type: String,
    }

    Ok(Wrapper::deserialize(deserializer)?.r#type)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct Episode {
    id: u32,
    number: u32,
    name: Option<String>,
    image: String,
    #[serde(alias = "seasonNumber")]
    season_number: u32,
}

#[derive(Debug, Clone, serde::Serialize, TS)]
#[serde(tag = "kind")]
#[ts(export)]
pub enum Media {
    Movie(TMDBData),
    Series(TVDBData)
}