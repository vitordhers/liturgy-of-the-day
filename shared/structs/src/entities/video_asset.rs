use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VideoAssetEntity {
    pub id: Thing,
    pub yt_id: String, //FlAXKUp8-X0
    pub thumbnail: Option<String>,
}
