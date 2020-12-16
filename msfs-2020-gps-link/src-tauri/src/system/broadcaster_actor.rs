use std::net::UdpSocket;

use actix::{Actor, ActorContext, Context, Handler};
use tracing::{error, instrument};

use crate::system::messages::GpsData;

use super::messages::StopMessage;

#[derive(Debug, Default)]
pub struct BroadcasterActor {
    pub broadcast_port: u16,
    pub broadcast_netmask: String,
    pub socket: Option<UdpSocket>,
}

impl Actor for BroadcasterActor {
    type Context = Context<Self>;

    #[instrument(name = "BroadcasterActorStarted", skip(self, _ctx))]
    fn started(&mut self, _ctx: &mut Self::Context) {
        let local_port = self.broadcast_port - 1;

        let socket = match UdpSocket::bind(format!("{}:{}", "0.0.0.0", local_port)) {
            Ok(s) => s,
            Err(e) => panic!("couldn't bind socket: {}", e),
        };

        socket.set_broadcast(true).unwrap();
        socket.connect(("255.255.255.255", local_port)).unwrap();

        self.socket = Some(socket);
    }

    #[instrument(name = "BroadcasterActorStopped", skip(self, _ctx))]
    fn stopped(&mut self, _ctx: &mut Self::Context) {}
}

impl Handler<GpsData> for BroadcasterActor {
    type Result = ();

    #[instrument(name = "BroadcasterActor::Handler<GpsData>", skip(self, _ctx))]
    fn handle(&mut self, sim_data: GpsData, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(socket) = &self.socket {
            // println!(
            //     "sending message {:?} {:?}",
            //     Utc::now().timestamp_millis(),
            //     sim_data
            // );
            socket
                .send_to(
                    format!(
                        "XGPSMSFS,{},{},{},{},{}",
                        sim_data.lon,
                        sim_data.lat,
                        sim_data.alt,
                        sim_data.true_heading,
                        sim_data.ground_speed
                    )
                    .as_bytes(),
                    format!("{}:{}", "255.255.255.255", self.broadcast_port),
                )
                .unwrap();
        } else {
            error!("socket is closed");
        }
    }
}

impl Handler<StopMessage> for BroadcasterActor {
    type Result = ();

    #[instrument(name = "BroadcasterActor::Handler<StopMessage>", skip(self, ctx))]
    fn handle(&mut self, _: StopMessage, ctx: &mut Context<Self>) -> Self::Result {
        ctx.stop();
    }
}
