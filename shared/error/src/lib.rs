use gloo::storage::errors::StorageError;
use leptos::ServerFnError;
use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::sync::TryLockError;
use strum::ParseError;
use surrealdb::Error as SurrealDbError;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgnusDailyError {
    pub title: String,
    pub description: String,
}

impl Error for AgnusDailyError {}
impl Display for AgnusDailyError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:/n/n{}", self.title, self.description)
    }
}

impl From<StorageError> for AgnusDailyError {
    fn from(storage_error: StorageError) -> Self {
        let title = String::from("Browser Storage Error");
        let description = storage_error.to_string();
        Self { title, description }
    }
}

impl From<ParseError> for AgnusDailyError {
    fn from(parse_error: ParseError) -> Self {
        let title = String::from("Strum Parse Error");
        let description = parse_error.to_string();
        Self { title, description }
    }
}

impl From<SerdeJsonError> for AgnusDailyError {
    fn from(storage_error: SerdeJsonError) -> Self {
        let title = String::from("Serde Json Error");
        let description = storage_error.to_string();
        Self { title, description }
    }
}

impl From<ReqwestError> for AgnusDailyError {
    fn from(request_error: ReqwestError) -> Self {
        let title = String::from("Reqwest Error");
        let description = request_error.to_string();
        Self { title, description }
    }
}

impl From<SurrealDbError> for AgnusDailyError {
    fn from(surrealdb_error: SurrealDbError) -> Self {
        let title = String::from("SurrealDb Error");
        let description = surrealdb_error.to_string();
        Self { title, description }
    }
}

impl From<ServerFnError> for AgnusDailyError {
    fn from(server_fn_error: ServerFnError) -> Self {
        let title = String::from("Server Fn Error");
        let description = server_fn_error.to_string();
        Self { title, description }
    }
}

impl<T> From<TryLockError<T>> for AgnusDailyError {
    fn from(lock_error: TryLockError<T>) -> Self {
        let title = String::from("Mutex lock error");
        let description = lock_error.to_string();
        Self { title, description }
    }
}

impl AgnusDailyError {
    pub fn from(title: String, description: String) -> Self {
        Self { title, description }
    }

    pub fn format_parse() -> Self {
        let title = String::from("Format Parse Error");
        let description = String::from("Unable to parse format.");
        Self { title, description }
    }

    pub fn element_not_found(id: &str) -> Self {
        let title = String::from("Element Error");
        let description = format!(
            "An element with id: '{}' could not be found in the window.",
            id
        );
        Self { title, description }
    }

    pub fn dyn_into_fail(id: &str) -> Self {
        let title = String::from("Element Error");
        let description = format!(
            "An element with id: '{}' could not be dynamically changed.",
            id
        );
        Self { title, description }
    }

    pub fn no_dash_video_format_available() -> Self {
        let title = String::from("DASH Format Error");
        let description = String::from("No DASH video format available.");
        Self { title, description }
    }

    pub fn no_legacy_format_available() -> Self {
        let title = String::from("Legacy Format Error");
        let description = String::from("No legacy video format available.");
        Self { title, description }
    }

    pub fn no_audio_format_available() -> Self {
        let title = String::from("Audio Format error");
        let description = String::from("No audio format available.");
        Self { title, description }
    }
}
