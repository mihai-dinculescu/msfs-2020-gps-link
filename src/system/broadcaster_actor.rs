use actix::{Actor, ActorContext, AsyncContext, Context, Handler};
use opentelemetry_api::Context as OpenTelemetryContext;
use tracing::{debug, info, instrument, warn, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::broadcaster::{BroadcasterConfig, BroadcasterExt, Com, Udp};
use crate::system::messages::{SimConnectDataMessage, StopMessage};
use crate::system::simconnect_objects::GpsData;

#[derive(Debug)]
pub struct BroadcasterActor {
    context: OpenTelemetryContext,
    config: BroadcasterConfig,
    broadcaster: Option<Box<dyn BroadcasterExt>>,
}

impl BroadcasterActor {
    pub fn new(context: OpenTelemetryContext, config: BroadcasterConfig) -> Self {
        Self {
            context,
            config,
            broadcaster: None,
        }
    }
}

impl Actor for BroadcasterActor {
    type Context = Context<Self>;

    #[instrument(name = "BroadcasterActor::started", skip(self, ctx))]
    fn started(&mut self, ctx: &mut Self::Context) {
        Span::current().set_parent(self.context.clone());

        let addr = ctx.address();

        let broadcaster = match self.config.clone() {
            BroadcasterConfig::Udp(config) => Udp::new(config),
            BroadcasterConfig::Com(config) => Com::new(config),
        };

        match broadcaster {
            Ok(socket) => {
                self.broadcaster = Some(socket);
                info!("BroadcasterActor started");
            }
            Err(_) => {
                addr.try_send(StopMessage {
                    context: Span::current().context(),
                    reason: "failed to configure broadcaster".to_string(),
                })
                .expect("BroadcasterActor queue is full");
            }
        }
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
        name = "BroadcasterActor::handle::<SimConnectDataMessage<GpsData>>",
        skip(self, message, ctx)
    )]
    fn handle(
        &mut self,
        message: SimConnectDataMessage<GpsData>,
        ctx: &mut Context<Self>,
    ) -> Self::Result {
        Span::current().set_parent(message.context);
        let data = message.data;

        if let Some(broadcaster) = self.broadcaster.as_mut() {
            debug!("Broadcasting SimConnectDataMessage<GpsData> message");

            let result = broadcaster.send(data);

            if result.is_err() {
                let addr = ctx.address();
                addr.try_send(StopMessage {
                    context: Span::current().context(),
                    reason: "failed to send broadcast".to_string(),
                })
                .expect("BroadcasterActor queue is full");
            }
        } else {
            warn!("failed to get the current broadcaster");
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
