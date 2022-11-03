use opentelemetry_api::Context;
use serde::{Deserialize, Serialize};
use std::{fmt, time};
use tokio::sync::oneshot::Receiver;
use tokio::sync::{self, mpsc::Sender, oneshot::error::TryRecvError};
use tokio::time::sleep;
use tracing::{debug, error, info, info_span, instrument, warn, Instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::{
    broadcaster::BroadcasterConfig,
    system::messages::{CoordinatorMessage, GetStatusMessage, RefreshRate},
};

pub struct AppState {
    pub tx: Sender<CoordinatorMessage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartOptions {
    pub refresh_rate: RefreshRate,
    pub config: BroadcasterConfig,
}

#[derive(Debug, Serialize)]
pub struct CommandResponse<T>
where
    T: fmt::Debug + Serialize,
{
    data: T,
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
pub struct ChannelResponse<T>
where
    T: fmt::Debug,
{
    pub context: Context,
    pub data: T,
}

const RESPONSE_CHANNEL_RETRIES: u64 = 10;
const RESPONSE_CHANNEL_RETRY_DELAY_MS: time::Duration = time::Duration::from_millis(100);

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

#[instrument(name = "cmd::cmd_get_available_com_ports", skip(state))]
#[tauri::command]
pub async fn cmd_get_available_com_ports(
    request_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<CommandResponse<Vec<String>>, CommandError> {
    let tx_local = state.tx.clone();
    let (response_tx, response_rx) = sync::oneshot::channel::<ChannelResponse<Vec<String>>>();
    let result = tx_local
        .send(CoordinatorMessage::GetAvailableComPorts {
            context: Span::current().context(),
            response_channel: response_tx,
        })
        .await;

    match result {
        Ok(_) => {
            poll_channel_response(response_rx)
                .instrument(info_span!("cmd::cmd_get_available_com_ports::recv"))
                .await
        }
        Err(e) => {
            error!(error = ?e, "the mpsc channel has closed");
            Err(CommandError::new("ERROR".to_string()))
        }
    }
}

#[instrument(name = "cmd::cmd_start", skip(state))]
#[tauri::command]
pub async fn cmd_start(
    request_id: String,
    options: StartOptions,
    state: tauri::State<'_, AppState>,
) -> Result<CommandResponse<bool>, CommandError> {
    let tx_local = state.tx.clone();

    let result = tx_local
        .send(CoordinatorMessage::Start {
            context: Span::current().context(),
            refresh_rate: options.refresh_rate,
            config: options.config,
        })
        .await;

    match result {
        Ok(_) => {
            let response = CommandResponse { data: true };
            info!(response = ?response, "Returning");
            Ok(response)
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
) -> Result<CommandResponse<bool>, CommandError> {
    let tx_local = state.tx.clone();
    let result = tx_local
        .send(CoordinatorMessage::Stop {
            context: Span::current().context(),
        })
        .await;

    match result {
        Ok(_) => {
            let response = CommandResponse { data: true };
            info!(response = ?response, "Returning");
            Ok(response)
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
) -> Result<CommandResponse<bool>, CommandError> {
    let tx_local = state.tx.clone();
    let (response_tx, response_rx) = sync::oneshot::channel::<ChannelResponse<bool>>();
    let result = tx_local
        .send(CoordinatorMessage::Status(GetStatusMessage {
            context: Span::current().context(),
            response_channel: response_tx,
        }))
        .await;

    match result {
        Ok(_) => {
            poll_channel_response(response_rx)
                .instrument(info_span!("cmd::cmd_get_status::recv"))
                .await
        }
        Err(e) => {
            error!(error = ?e, "the mpsc channel has closed");
            Err(CommandError::new("ERROR".to_string()))
        }
    }
}

#[instrument(name = "cmd::poll_channel_response", skip(rx))]
async fn poll_channel_response<T>(
    mut rx: Receiver<ChannelResponse<T>>,
) -> Result<CommandResponse<T>, CommandError>
where
    T: fmt::Debug + Serialize,
{
    let mut counter = 0;

    loop {
        let response = rx.try_recv();

        match response {
            Ok(value) => {
                let response = CommandResponse { data: value.data };
                debug!(response = ?response, "Returning");
                return Ok(response);
            }
            Err(TryRecvError::Empty) => {
                counter += 1;

                if counter >= RESPONSE_CHANNEL_RETRIES {
                    break;
                } else {
                    sleep(RESPONSE_CHANNEL_RETRY_DELAY_MS).await;
                }
            }
            Err(TryRecvError::Closed) => {
                error!("the onehost channel has closed");
                return Err(CommandError::new("ERROR".to_string()));
            }
        };
    }

    warn!("failed to get a response from the oneshot channel in due time");
    Err(CommandError::new("TIMEOUT".to_string()))
}
