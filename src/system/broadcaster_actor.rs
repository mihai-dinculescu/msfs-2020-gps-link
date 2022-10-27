use std::net::UdpSocket;

use actix::{Actor, ActorContext, AsyncContext, Context, Handler};
use opentelemetry_api::Context as OpenTelemetryContext;
use tracing::{debug, error, info, instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::system::messages::{SimConnectDataMessage, StopMessage};
use crate::system::simconnect_objects::GpsData;

#[derive(Debug, Default)]
pub struct BroadcasterActor {
    context: OpenTelemetryContext,
    broadcast_port: u16,
    broadcast_netmask: String,
    socket: Option<UdpSocket>,
}

impl BroadcasterActor {
    pub fn new(
        context: OpenTelemetryContext,
        broadcast_port: u16,
        broadcast_netmask: String,
    ) -> Self {
        Self {
            context,
            broadcast_port,
            broadcast_netmask,
            ..Default::default()
        }
    }
}

impl Actor for BroadcasterActor {
    type Context = Context<Self>;

    #[instrument(name = "BroadcasterActor::started", skip(self, ctx))]
    fn started(&mut self, ctx: &mut Self::Context) {
        Span::current().set_parent(self.context.clone());

        let addr = ctx.address();
        let local_port = self.broadcast_port - 1;
        let broadcast_netmask = self.broadcast_netmask.clone();

        let socket = match UdpSocket::bind(format!("{}:{}", "0.0.0.0", local_port)) {
            Ok(s) => s,
            Err(e) => panic!("couldn't bind socket: {}", e),
        };

        if let Err(e) = socket.set_broadcast(true) {
            error!(error = ?e, "failed to set UDP broadcast");
            addr.try_send(StopMessage {
                context: Span::current().context(),
                reason: "failed to set UDP broadcast".to_string(),
            })
            .expect("BroadcasterActor queue is full");
        }

        if let Err(e) = socket.connect((broadcast_netmask, local_port)) {
            error!(error = ?e, "failed to connect UDP");
            addr.try_send(StopMessage {
                context: Span::current().context(),
                reason: "failed to connect UDP".to_string(),
            })
            .expect("BroadcasterActor queue is full");
        }

        self.socket = Some(socket);

        info!("BroadcasterActor started");
    }

    #[instrument(name = "BroadcasterActor::stopped", skip(self))]
    fn stopped(&mut self, _: &mut Self::Context) {
        Span::current().set_parent(self.context.clone());
        info!("BroadcasterActor stopped");
    }
}

impl Handler<SimConnectDataMessage<GpsData>> for BroadcasterActor {
    type Result = ();

    #[instrument(
        name = "BroadcasterActor::handle::<GpsDataMessage>",
        skip(self, message, ctx)
    )]
    fn handle(
        &mut self,
        message: SimConnectDataMessage<GpsData>,
        ctx: &mut Context<Self>,
    ) -> Self::Result {
        Span::current().set_parent(message.context);
        let data = message.data;

        if let Some(socket) = &self.socket {
            debug!("Broadcasting GpsDataMessage message");

            let track = data.gps_ground_magnetic_track - data.gps_magnetic_variation;

            let result = socket.send_to(
                format!(
                    "XGPSMSFS,{},{},{},{},{}",
                    data.lon, data.lat, data.alt, track, data.gps_ground_speed
                )
                .as_bytes(),
                format!("{}:{}", &self.broadcast_netmask, self.broadcast_port),
            );

            if let Err(e) = result {
                error!(error = ?e, "failed to send broadcast over UDP");
                let addr = ctx.address();
                addr.try_send(StopMessage {
                    context: Span::current().context(),
                    reason: "failed to send broadcast over UDP".to_string(),
                })
                .expect("BroadcasterActor queue is full");
            }
        } else {
            debug!("UDP socket is not yet open");
        }
    }
}

impl Handler<StopMessage> for BroadcasterActor {
    type Result = ();

    #[instrument(
        name = "BroadcasterActor::handle::<StopMessage>",
        skip(self, message, ctx)
    )]
    fn handle(&mut self, message: StopMessage, ctx: &mut Context<Self>) -> Self::Result {
        Span::current().set_parent(message.context.clone());
        info!(reason = ?message.reason, "BroadcasterActor stopping");
        ctx.stop();
    }
}
