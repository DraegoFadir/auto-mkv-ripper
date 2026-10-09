use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_string_from_number;
use crate::models::{Episode, Media, Season, TVDBData};

#[derive(Debug, Serialize, Deserialize)]
pub struct TVDBResponse {
    #[serde(deserialize_with = "deserialize_string_from_number")]
    id: String,
    name: Option<String>,
    #[serde(alias = "image")]
    image_url: Option<String>,
    year: Option<String>,
    #[serde(default)]
    seasons: Vec<Season>,
    #[serde(default)]
    episodes: Vec<Episode>
}

impl From<TVDBResponse> for Media {
    fn from(res: TVDBResponse) -> Self {
        Media::Series(TVDBData { 
            id: res.id.rsplit("-").next().unwrap().to_owned(),
            name: res.name, 
            image_url: res.image_url, 
            year: res.year,
            seasons: res.seasons.into_iter().filter(|s| s.season_type.eq_ignore_ascii_case("official")).collect(),
            episodes: res.episodes
        })
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