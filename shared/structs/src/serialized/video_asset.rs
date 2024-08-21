use crate::entities::VideoAssetEntity;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VideoAsset {
    pub id: String,
    pub lang: String,
    pub yt_id: String,
    pub thumbnail: Option<String>,
}

impl VideoAsset {
    fn new(id: String, lang: String, yt_id: String, thumbnail: Option<String>) -> Self {
        Self {
            id,
            lang,
            yt_id,
            thumbnail,
        }
    }
}

impl From<VideoAssetEntity> for VideoAsset {
    fn from(value: VideoAssetEntity) -> Self {
        let json_input = value.id.id.to_string().replace("'", "\"");
        let id = from_str::<Vec<String>>(&json_input)
            .expect("VideoAssetEntity id to be parseable as JSON");
        Self::new(
            value.id.id.to_string(),
            id.get(1)
                .expect("id to have index 1")
                .to_string(),
            value.yt_id,
            value.thumbnail,
        )
    }
}
