use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::VideoAssetEntity;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GospelEntity {
    pub id: Thing,
    pub locale_video_assets_ids_map: Option<HashMap<String, Thing>>,
    pub video_assets: Vec<VideoAssetEntity>,
}
