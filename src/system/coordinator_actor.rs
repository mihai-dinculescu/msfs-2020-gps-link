use actix::{
    Actor, ActorContext, Addr, AsyncContext, Context, Handler, SpawnHandle, System, WrapFuture,
};
use tokio::sync::mpsc::Receiver;
use tracing::{debug, error, field, info, instrument, warn, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::cmd::StatusResponse;
use crate::system::{
    broadcaster_actor::BroadcasterActor,
    landing_detection_actor::LandingDetectionActor,
    messages::{CoordinatorMessage, GetStatusMessage, GetStatusResponseMessage, StopMessage},
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
        skip(self, message, ctx),
        fields(request_id = field::Empty)
    )]
    fn handle(&mut self, message: CoordinatorMessage, ctx: &mut Context<Self>) -> Self::Result {
        let span = Span::current();

        match message {
            CoordinatorMessage::Start {
                context,
                refresh_rate,
                config,
            } => {
                span.set_parent(context);
                debug!("CoordinatorActor received Start");

                self.stop_actors(StopMessage {
                    context: span.context(),
                    reason: "Start".to_string(),
                });

                let coordinator_addr = ctx.address();

                let broadcaster_addr = BroadcasterActor::new(span.context(), config).start();

                let landing_detection_addr = LandingDetectionActor::new(span.context()).start();

                let simconnect = SimConnectActor::new(
                    span.context(),
                    refresh_rate,
                    // disabled for now as this functionality is not fully implemented
                    false,
                    coordinator_addr,
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
            CoordinatorMessage::Status(GetStatusMessage {
                context,
                response_channel,
            }) => {
                span.set_parent(context);
                debug!("CoordinatorActor received Status");

                let mut successful_checks = 0u32;

                if let Some(addr) = &self.broadcaster_addr {
                    if addr.connected() {
                        successful_checks += 1;
                    }
                }

                if let Some(addr) = &self.landing_detection_addr {
                    if addr.connected() {
                        successful_checks += 1;
                    }
                }

                if let Some(addr) = &self.simconnect_addr {
                    if addr.connected() {
                        successful_checks += 1;
                    }
                }

                if successful_checks == 3 {
                    // things might be OK but we must check if the SimConnect actor is connected
                    self.simconnect_addr
                        .as_ref()
                        .expect("this should never happen")
                        // it's fine not to check the result here
                        // because the worst that can happen is that the get status command will timeout
                        .do_send(GetStatusMessage {
                            context: Span::current().context(),
                            response_channel,
                        });
                } else {
                    let coordinator_addr = ctx.address();

                    // it's fine not to check the result here
                    // because the worst that can happen is that the get status command will timeout
                    coordinator_addr.do_send(GetStatusResponseMessage {
                        context: Span::current().context(),
                        status: false,
                        response_channel,
                    });
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
                // it's fine not to check the result here
                // because the actor in question will stop itself
                addr.do_send(message.clone());
            }
            self.broadcaster_addr = None;
        }

        if let Some(addr) = &self.landing_detection_addr {
            if addr.connected() {
                // it's fine not to check the result here
                // because the actor in question will stop itself
                addr.do_send(message.clone());
            }
            self.landing_detection_addr = None;
        }

        if let Some(addr) = &self.simconnect_addr {
            if addr.connected() {
                // it's fine not to check the result here
                // because the actor in question will stop itself
                addr.do_send(message);
            }
            self.simconnect_addr = None;
        }
    }
}

impl Handler<GetStatusResponseMessage> for CoordinatorActor {
    type Result = ();

    #[instrument(
        name = "CoordinatorActor::handle::<GetStatusResponseMessage>",
        skip(self, message)
    )]
    fn handle(&mut self, message: GetStatusResponseMessage, _: &mut Context<Self>) -> Self::Result {
        Span::current().set_parent(message.context);

        if let Err(e) = message.response_channel.send(StatusResponse {
            context: Span::current().context(),
            status: message.status,
        }) {
            warn!(error = ?e, "failed to send through the mpsc channel");
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
