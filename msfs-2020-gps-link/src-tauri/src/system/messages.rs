use actix::Message;
use serde::Deserialize;
use tokio::sync;

use simconnect_client::AirportData;

#[derive(Debug, Copy, Clone, Message)]
#[rtype(result = "()")]
pub struct GpsDataMessage {
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
    pub gps_ground_true_track: f64,
    pub gps_ground_speed: f64,
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct AirportListMessage {
    pub data: Vec<AirportData>,
}

#[derive(Debug, Copy, Clone, Message)]
#[rtype(result = "()")]
pub struct OnGroundMessage {
    pub sim_on_ground: f64,
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

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum RefreshRate {
    Slow,
    Fast,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct StopMessage;
