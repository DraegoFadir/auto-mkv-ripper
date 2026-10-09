use serde::{Deserialize, Serialize};

use crate::models::{Media, TVDBData};

#[derive(Debug, Serialize, Deserialize)]
pub struct TVDBResponse {
    tvdb_id: String,
    name: String,
    image_url: Option<String>,
    year: Option<String>,
}

impl From<TVDBResponse> for Media {
    fn from(res: TVDBResponse) -> Self {
        Media::Series(TVDBData { id: res.tvdb_id, name: res.name, image_url: res.image_url, year: res.year })
    }
}

#[derive(Deserialize)]
pub struct TVDBResponseArray {
    data: Vec<TVDBResponse>
}

impl From<TVDBResponseArray> for Vec<Media> {
    fn from(res: TVDBResponseArray) -> Self {
        res.data
            .into_iter()
            .map(|r| r.into())
            .collect()
    }
}