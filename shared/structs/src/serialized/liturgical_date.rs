use enums::liturgy::LiturgicalTime;
use serde::{Deserialize, Serialize};

use crate::{entities::LiturgicalDateEntity, serialized::Commemoration};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct LiturgicalDate {
    pub id: String,
    pub time: LiturgicalTime,
    pub day: u8,
    pub month: u8,
    pub year: i32,
    pub commemorations: Vec<Commemoration>,
}

impl LiturgicalDate {
    pub fn new(
        id: String,
        time: LiturgicalTime,
        day: u8,
        month: u8,
        year: i32,
        commemorations: Vec<Commemoration>,
    ) -> Self {
        Self {
            id,
            time,
            day,
            month,
            year,
            commemorations,
        }
    }

    pub fn get_precedent_commemoration(&self) -> &Commemoration {
        self.commemorations
            .iter()
            .max_by_key(|l| l.category)
            .expect("Commemorations to have a precedent Commemoration")
    }
}

impl From<LiturgicalDateEntity> for LiturgicalDate {
    fn from(value: LiturgicalDateEntity) -> Self {
        Self::new(
            value.id.to_string(),
            value.time,
            value.day,
            value.month,
            value.year,
            value
                .commemorations
                .into_iter()
                .map(|c| c.into())
                .collect::<Vec<Commemoration>>(),
        )
    }
}
