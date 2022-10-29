use actix::Message;
use opentelemetry_api::Context;
use serde::Deserialize;
use tokio::sync;

use crate::cmd::StatusResponse;

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct SimConnectDataMessage<T> {
    pub context: Context,
    pub data: T,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub enum CoordinatorMessage {
    Start {
        context: Context,
        refresh_rate: RefreshRate,
        broadcast_netmask: String,
        broadcast_port: u16,
    },
    Stop {
        context: Context,
    },
    Status(GetStatusMessage),
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct GetStatusMessage {
    pub context: Context,
    pub response_channel: sync::oneshot::Sender<StatusResponse>,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct GetStatusResponseMessage {
    pub context: Context,
    pub status: bool,
    pub response_channel: sync::oneshot::Sender<StatusResponse>,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct SimConnectPing;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum RefreshRate {
    Slow,
    Fast,
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct StopMessage {
    pub reason: String,
    pub context: Context,
}
