use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommonImage {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommonThumbnail {
    #[serde(default)]
    pub quality: String,
    pub url: String,
    pub width: u32,
    pub height: u32,
}
