use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
}
