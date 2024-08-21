use serde::{Deserialize, Serialize};

use crate::entities::GospelEntity;

use super::VideoAsset;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Gospel {
    pub id: String,
    pub video_assets: Vec<VideoAsset>,
}

impl Gospel {
    fn new(id: String, video_assets: Vec<VideoAsset>) -> Self {
        Self { id, video_assets }
    }
}

impl From<GospelEntity> for Gospel {
    fn from(value: GospelEntity) -> Self {
        Self::new(
            value.id.to_string(),
            value
                .video_assets
                .into_iter()
                .map(|v| v.into())
                .collect::<Vec<VideoAsset>>(),
        )
    }
}
