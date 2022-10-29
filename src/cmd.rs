use opentelemetry_api::Context;
use serde::{Deserialize, Serialize};
use std::{thread, time};
use tokio::sync::{self, mpsc::Sender, oneshot::error::TryRecvError};
use tracing::{error, info, info_span, instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::system::messages::{CoordinatorMessage, GetStatusMessage, RefreshRate};

pub struct AppState {
    pub tx: Sender<CoordinatorMessage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartOptions {
    pub refresh_rate: RefreshRate,
    pub broadcast_netmask: String,
    pub broadcast_port: u16,
}

#[derive(Serialize)]
pub struct Response<'a> {
    message: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommandError {
    message: String,
}

impl CommandError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CommandError {}

#[derive(Debug)]
pub struct StatusResponse {
    pub context: Context,
    pub status: bool,
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

#[instrument(name = "cmd::cmd_start", skip(state))]
#[tauri::command]
pub async fn cmd_start(
    request_id: String,
    options: StartOptions,
    state: tauri::State<'_, AppState>,
) -> Result<Response<'_>, CommandError> {
    let tx_local = state.tx.clone();

    let result = tx_local
        .send(CoordinatorMessage::Start {
            context: Span::current().context(),
            refresh_rate: options.refresh_rate,
            broadcast_netmask: options.broadcast_netmask,
            broadcast_port: options.broadcast_port,
        })
        .await;

    match result {
        Ok(_) => {
            let message = "OK";

            info!(response = ?message);
            Ok(Response { message })
        }
        Err(e) => {
            error!(error = ?e, "the mpsc channel has closed");
            Err(CommandError::new("the mpsc channel has closed".to_string()))
        }
    }
}

#[instrument(name = "cmd::cmd_stop", skip(state))]
#[tauri::command]
pub async fn cmd_stop(
    request_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Response<'_>, CommandError> {
    let tx_local = state.tx.clone();
    let result = tx_local
        .send(CoordinatorMessage::Stop {
            context: Span::current().context(),
        })
        .await;

    match result {
        Ok(_) => {
            let message = "OK";

            info!(response = ?message);
            Ok(Response { message })
        }
        Err(e) => {
            error!(error = ?e, "the mpsc channel has closed");
            Err(CommandError::new("the mpsc channel has closed".to_string()))
        }
    }
}

#[instrument(name = "cmd::cmd_get_status", skip(state))]
#[tauri::command]
pub async fn cmd_get_status(
    request_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Response<'_>, CommandError> {
    let tx_local = state.tx.clone();
    let (response_tx, mut response_rx) = sync::oneshot::channel::<StatusResponse>();
    let result = tx_local
        .send(CoordinatorMessage::Status(GetStatusMessage {
            context: Span::current().context(),
            response_channel: response_tx,
        }))
        .await;

    match result {
        Ok(_) => {
            let mut counter = 0;

            loop {
                let response = response_rx.try_recv();

                match response {
                    Ok(value) => {
                        let message = match value.status {
                            true => "CONNECTED",
                            false => "NOT_CONNECTED",
                        };

                        let span = info_span!("cmd::cmd_get_status::recv");
                        span.set_parent(value.context);
                        span.in_scope(|| {
                            info!(response = ?message, "Returning");
                        });

                        return Ok(Response { message });
                    }
                    Err(TryRecvError::Empty) => {
                        counter += 1;

                        if counter >= 10 {
                            break;
                        } else {
                            thread::sleep(time::Duration::from_millis(100));
                        }
                    }
                    Err(TryRecvError::Closed) => {
                        return Err(CommandError::new("ERROR".to_string()))
                    }
                };
            }
            Err(CommandError::new("Timeout".to_string()))
        }
        Err(e) => {
            error!(error = ?e, "the mpsc channel has closed");
            Err(CommandError::new("the mpsc channel has closed".to_string()))
        }
    }
}
