use std::collections::HashMap;

use enums::liturgy::{CommemorationCategory, LiturgicalColor, LiturgicalTime};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateCommemorationEntityDto {
    pub category: CommemorationCategory,
    pub color: LiturgicalColor,
    pub gospel_uuid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateGospelEntityDto {
    pub locale_video_assets_ids_map: HashMap<String, Thing>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateLiturgicalDateEntityDto {
    pub time: LiturgicalTime,
    pub day: u8,
    pub month: u8,
    pub year: i32,
    pub commemorations_ids: Vec<Thing>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateVideoAssetEntityDto {
    pub yt_id: String,
    pub thumbnail_url: Option<String>,
}
