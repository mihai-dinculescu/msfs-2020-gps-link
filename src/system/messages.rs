use actix::Message;
use serde::Deserialize;
use simconnect_sdk::{Airport, SimConnectObject};
use tokio::sync;

#[derive(Debug, Clone, Message, SimConnectObject)]
#[rtype(result = "()")]
#[simconnect(period = "second")]
pub struct GpsDataMessage {
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
    pub lat: f64,
    #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
    pub lon: f64,
    #[simconnect(name = "PLANE ALTITUDE", unit = "meters")]
    pub alt: f64,
    #[simconnect(name = "GPS GROUND MAGNETIC TRACK", unit = "Degrees")]
    pub gps_ground_magnetic_track: f64,
    #[simconnect(name = "MAGVAR", unit = "Degrees")]
    pub gps_magnetic_variation: f64,
    #[simconnect(name = "GPS GROUND SPEED", unit = "Meters per second")]
    pub gps_ground_speed: f64,
}

#[derive(Debug, Clone, Message, SimConnectObject)]
#[rtype(result = "()")]
#[simconnect(period = "visual-frame", condition = "changed")]
pub struct OnGroundMessage {
    #[simconnect(name = "SIM ON GROUND")]
    pub sim_on_ground: bool,
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct AirportListMessage {
    pub data: Vec<Airport>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum RefreshRate {
    Slow,
    Fast,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct StopMessage;
