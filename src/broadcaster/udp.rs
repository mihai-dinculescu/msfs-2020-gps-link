use std::{io, net::UdpSocket};

use tracing::{error, instrument, trace};

use crate::system::simconnect_objects::GpsData;

use super::{BroadcasterExt, UdpConfig};

#[derive(Debug)]
pub struct Udp {
    socket: UdpSocket,
    port: u16,
    netmask: String,
}

impl Udp {
    #[instrument(name = "Udp::new")]
    pub fn new(config: UdpConfig) -> Result<Box<dyn BroadcasterExt>, io::Error> {
        let local_port = config.port - 1;
        let broadcast_netmask = config.netmask.clone();

        let socket = UdpSocket::bind(format!("{}:{}", "0.0.0.0", local_port)).map_err(|e| {
            error!(error = ?e, "failed to bind to the UDP socket");
            e
        })?;
        socket.set_broadcast(true).map_err(|e| {
            error!(error = ?e, "failed to set the UDP socket to broadcast");
            e
        })?;
        socket
            .connect((broadcast_netmask, local_port))
            .map_err(|e| {
                error!(error = ?e, "failed to connect to the UDP socket");
                e
            })?;

        Ok(Box::new(Udp {
            socket,
            port: config.port,
            netmask: config.netmask,
        }))
    }
}

impl BroadcasterExt for Udp {
    #[instrument(name = "Udp::send", skip(self, data))]
    fn send(&mut self, data: GpsData) -> Result<(), io::Error> {
        let track = data.gps_ground_magnetic_track * 0.0174533; //convert radians to degrees.

        self.socket
            .send_to(
                format!(
                    "XGPSMSFS,{},{},{},{},{}",
                    data.lon, data.lat, data.alt, track, data.gps_ground_speed
                )
                .as_bytes(),
                format!("{}:{}", &self.netmask, self.port),
            )
            .map(|_| {
                trace!("Successfully sent broadcast over UDP");
            })
            .map_err(|e| {
                error!(error = ?e, "failed to broadcast over UDP");
                e
            })
    }
}
