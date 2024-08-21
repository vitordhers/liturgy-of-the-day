use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiturgicalTime {
    #[default]
    Ordinary,
    Advent,
    Christmas,
    Lent,
    Easter,
    PaschalTriduum,
}
