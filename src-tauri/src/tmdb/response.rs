use serde::{Deserialize, Serialize};

use crate::models::{Media, TMDBData};

#[derive(Debug, Serialize, Deserialize)]
pub struct TMDBResponse {
    id: u32,
    title: String,
    poster_path: Option<String>,
    release_date: Option<String>,
}

impl From<TMDBResponse> for Media {
    fn from(res: TMDBResponse) -> Self {
        Media::Movie(TMDBData { id: res.id, title: res.title, poster_path: res.poster_path, release_date: res.release_date })
    }
}

#[derive(Deserialize)]
pub struct TMDBResponseArray {
    results: Vec<TMDBResponse>
}

impl From<TMDBResponseArray> for Vec<Media> {
    fn from(res: TMDBResponseArray) -> Self {
        res.results
            .into_iter()
            .map(|r| r.into())
            .collect()
    }
}