use enums::liturgy::LiturgicalTime;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::CommemorationEntity;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LiturgicalDateEntity {
    pub id: Thing,
    pub time: LiturgicalTime,
    pub day: u8,
    pub month: u8,
    pub year: i32,
    pub commemorations_ids: Option<Vec<Thing>>,
    pub commemorations: Vec<CommemorationEntity>,
}

impl LiturgicalDateEntity {
    pub fn get_precedent_commemoration(&self) -> &CommemorationEntity {
        self.commemorations
            .iter()
            .max_by_key(|l| l.category)
            .expect("Commemorations to have a precedent Commemoration")
    }
}

