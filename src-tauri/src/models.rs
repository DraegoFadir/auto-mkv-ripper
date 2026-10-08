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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct Media {
    pub id: u32,
    pub title: String,
    pub poster_path: Option<String>,
    pub release_date: Option<String>
}