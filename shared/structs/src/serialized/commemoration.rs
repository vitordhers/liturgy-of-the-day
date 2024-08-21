use enums::liturgy::{CommemorationCategory, LiturgicalColor};
use serde::{Deserialize, Serialize};

use crate::entities::CommemorationEntity;

use super::Gospel;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Commemoration {
    pub id: String,
    pub title: String,
    pub category: CommemorationCategory,
    pub color: LiturgicalColor,
    pub gospel: Gospel,
}

impl Commemoration {
    pub fn new(
        id: String,
        title: String,
        category: CommemorationCategory,
        color: LiturgicalColor,
        gospel: Gospel,
    ) -> Self {
        Self {
            id,
            title,
            category,
            color,
            gospel,
        }
    }
}

impl From<CommemorationEntity> for Commemoration {
    fn from(value: CommemorationEntity) -> Self {
        Self::new(
            value.id.to_string(),
            value.title,
            value.category,
            value.color,
            value.gospel.into(),
        )
    }
}
