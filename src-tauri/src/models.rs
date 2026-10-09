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
    pub name: String,
    pub image_url: Option<String>,
    pub year: Option<String>
}


#[derive(Debug, Clone, serde::Serialize, TS)]
#[serde(tag = "kind")]
#[ts(export)]
pub enum Media {
    Movie(TMDBData),
    Series(TVDBData)
}