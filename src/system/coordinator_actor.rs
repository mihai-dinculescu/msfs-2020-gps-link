use actix::{
    Actor, ActorContext, Addr, AsyncContext, Context, Handler, SpawnHandle, System, WrapFuture,
};
use tokio::sync::mpsc::Receiver;
use tracing::{debug, error, field, info, instrument, warn, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::system::{
    broadcaster_actor::BroadcasterActor,
    landing_detection_actor::LandingDetectionActor,
    messages::{CoordinatorMessage, StopMessage},
    simconnect_actor::SimConnectActor,
};

#[derive(Debug)]
pub struct CoordinatorActor {
    rx: Option<Receiver<CoordinatorMessage>>,
    handle: Option<SpawnHandle>,
    broadcaster_addr: Option<Addr<BroadcasterActor>>,
    landing_detection_addr: Option<Addr<LandingDetectionActor>>,
    simconnect_addr: Option<Addr<SimConnectActor>>,
}

impl CoordinatorActor {
    pub fn new(rx: Receiver<CoordinatorMessage>) -> Self {
        Self {
            rx: Some(rx),
            handle: None,
            broadcaster_addr: None,
            landing_detection_addr: None,
            simconnect_addr: None,
        }
    }
}

impl Actor for CoordinatorActor {
    type Context = Context<Self>;

    #[instrument(name = "CoordinatorActor::started", skip(self, ctx))]
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();

        match self.rx.take() {
            Some(mut rx) => {
                let fut = async move {
                    loop {
                        match rx.recv().await {
                            Some(message) => {
                                addr.try_send(message)
                                    .expect("CoordinatorActor mailbox is full");
                            }
                            None => {
                                error!("the mpsc channel has closed");
                                addr.try_send(StopMessage {
                                    reason: "the mpsc channel has closed".to_string(),
                                    context: Span::current().context(),
                                })
                                .expect("CoordinatorActor mailbox is full");
                            }
                        }
                    }
                }
                .into_actor(self);

                let handle = ctx.spawn(fut);
                self.handle = Some(handle);

                info!("CoordinatorActor started");
            }
            None => {
                error!("no mpsc receiver has been provided");
                addr.try_send(StopMessage {
                    reason: "no mpsc receiver has been provided".to_string(),
                    context: Span::current().context(),
                })
                .expect("CoordinatorActor mailbox is full");
            }
        }
    }

    #[instrument(name = "CoordinatorActor::stopped", skip(self))]
    fn stopped(&mut self, _: &mut Self::Context) {
        info!("CoordinatorActor stopped");
    }
}

impl Handler<CoordinatorMessage> for CoordinatorActor {
    type Result = ();

    #[instrument(
        name = "CoordinatorActor::Handler<CoordinatorMessage>",
        skip(self, message),
        fields(request_id = field::Empty)
    )]
    fn handle(&mut self, message: CoordinatorMessage, _: &mut Context<Self>) -> Self::Result {
        let span = Span::current();

        match message {
            CoordinatorMessage::Start {
                context,
                refresh_rate,
                broadcast_netmask,
                broadcast_port,
            } => {
                span.set_parent(context);
                debug!("CoordinatorActor received Start");

                self.stop_actors(StopMessage {
                    context: span.context(),
                    reason: "Start".to_string(),
                });

                let broadcaster_addr =
                    BroadcasterActor::new(span.context(), broadcast_port, broadcast_netmask)
                        .start();

                let landing_detection_addr = LandingDetectionActor::new(span.context()).start();

                let simconnect = SimConnectActor::new(
                    span.context(),
                    refresh_rate,
                    // disabled for now as this functionality is not fully implemented
                    false,
                    broadcaster_addr.clone(),
                    landing_detection_addr.clone(),
                )
                .start();

                self.broadcaster_addr = Some(broadcaster_addr);
                self.landing_detection_addr = Some(landing_detection_addr);
                self.simconnect_addr = Some(simconnect);
            }
            CoordinatorMessage::Stop { context } => {
                span.set_parent(context);
                debug!("CoordinatorActor received Stop");

                self.stop_actors(StopMessage {
                    context: span.context(),
                    reason: "Stop".to_string(),
                });
            }
            CoordinatorMessage::Status {
                context,
                response_channel,
            } => {
                span.set_parent(context);
                debug!("CoordinatorActor received Status");

                let mut actors_alive = 0;

                if let Some(addr) = &self.broadcaster_addr {
                    if addr.connected() {
                        actors_alive += 1;
                    }
                }

                if let Some(addr) = &self.landing_detection_addr {
                    if addr.connected() {
                        actors_alive += 1;
                    }
                }

                if let Some(addr) = &self.simconnect_addr {
                    if addr.connected() {
                        actors_alive += 1;
                    }
                }

                let response = actors_alive == 3;

                if let Err(e) = response_channel.send(response) {
                    warn!(error = ?e, "failed to send through the mpsc channel");
                }
            }
        }
    }
}

impl CoordinatorActor {
    #[instrument(name = "CoordinatorActor::stop_actors", skip(self))]
    fn stop_actors(&mut self, message: StopMessage) {
        Span::current().set_parent(message.context);

        let message = StopMessage {
            context: Span::current().context(),
            ..message
        };

        if let Some(addr) = &self.broadcaster_addr {
            if addr.connected() {
                addr.try_send(message.clone())
                    .expect("BroadcasterActor queue is full");
            }
            self.broadcaster_addr = None;
        }

        if let Some(addr) = &self.landing_detection_addr {
            if addr.connected() {
                addr.try_send(message.clone())
                    .expect("LandingDetectionActor queue is full");
            }
            self.landing_detection_addr = None;
        }

        if let Some(addr) = &self.simconnect_addr {
            if addr.connected() {
                addr.try_send(message)
                    .expect("SimConnectActor queue is full");
            }
            self.simconnect_addr = None;
        }
    }
}

impl Handler<StopMessage> for CoordinatorActor {
    type Result = ();

    #[instrument(
        name = "CoordinatorActor::handle::<StopMessage>",
        skip(self, message, ctx)
    )]
    fn handle(&mut self, message: StopMessage, ctx: &mut Context<Self>) -> Self::Result {
        Span::current().set_parent(message.context.clone());

        info!(reason = ?message.reason, "CoordinatorActor stopping");
        ctx.stop();

        info!("Stopping the whole system...");
        System::current().stop();
    }
}
