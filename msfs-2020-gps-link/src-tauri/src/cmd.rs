use serde::{Deserialize, Serialize};

use crate::system::messages::RefreshRate;

#[derive(Debug, Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub struct StartOptions {
    pub refresh_rate: RefreshRate,
    pub broadcast_netmask: String,
    pub broadcast_port: u16,
}

// The commands definitions
// Deserialized from JS
#[derive(Debug, Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    #[serde(rename_all = "camelCase")]
    Start {
        request_id: String,
        options: StartOptions,
        callback: String,
        error: String,
    },
    #[serde(rename_all = "camelCase")]
    Stop {
        request_id: String,
        callback: String,
        error: String,
    },
    #[serde(rename_all = "camelCase")]
    Status {
        request_id: String,
        callback: String,
        error: String,
    },
}

#[derive(Serialize)]
pub struct Response<'a> {
    pub message: &'a str,
}

// An error type we define
// We could also use the `anyhow` lib here
#[derive(Debug, Clone)]
pub struct CommandError {
    message: String,
}

impl<'a> CommandError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl<'a> std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Tauri uses the `anyhow` lib so custom error types must implement std::error::Error
// and the function call should call `.into()` on it
impl<'a> std::error::Error for CommandError {}
