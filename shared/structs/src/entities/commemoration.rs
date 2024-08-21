use enums::liturgy::{CommemorationCategory, LiturgicalColor};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::GospelEntity;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CommemorationEntity {
    pub id: Thing,
    pub category: CommemorationCategory,
    pub color: LiturgicalColor,
    pub gospel_uuid: Option<String>,
    pub gospel: GospelEntity,
    pub title: String,
}
