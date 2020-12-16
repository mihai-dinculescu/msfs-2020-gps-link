use actix::Message;
use serde::Deserialize;
use tokio::sync;

#[derive(Debug, Copy, Clone, Message)]
#[rtype(result = "()")]
pub struct GpsData {
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
    pub true_heading: f64,
    pub ground_speed: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum RefreshRate {
    Slow,
    Fast,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub enum CoordinatorMessage {
    Start {
        request_id: String,
        refresh_rate: RefreshRate,
        broadcast_netmask: String,
        broadcast_port: u16,
    },
    Stop {
        request_id: String,
    },
    Status {
        request_id: String,
        response_channel: sync::oneshot::Sender<bool>,
    },
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct StopMessage;
