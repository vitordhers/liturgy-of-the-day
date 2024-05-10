use gloo::storage::errors::StorageError;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiturgyOfTheDayError {
    pub title: String,
    pub description: String,
}

impl Display for LiturgyOfTheDayError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:/n/n{}", self.title, self.description)
    }
}

impl From<StorageError> for LiturgyOfTheDayError {
    fn from(storage_error: StorageError) -> Self {
        let title = String::from("Browser Storage Error");
        let description = storage_error.to_string();
        Self { title, description }
    }
}

impl From<SerdeJsonError> for LiturgyOfTheDayError {
    fn from(storage_error: SerdeJsonError) -> Self {
        let title = String::from("Serde Json Error");
        let description = storage_error.to_string();
        Self { title, description }
    }
}

impl LiturgyOfTheDayError {
    pub fn from(title: String, description: String) -> Self {
        Self { title, description }
    }
}
