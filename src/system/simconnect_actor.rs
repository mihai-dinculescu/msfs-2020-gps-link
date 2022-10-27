use actix::{clock::sleep, Actor, ActorContext, Addr, AsyncContext, Context, Handler, WrapFuture};
use opentelemetry_api::Context as OpenTelemetryContext;
use simconnect_sdk::{Condition, Notification, Period, SimConnect, SimConnectError};
use tracing::{debug_span, error, info, instrument, trace, Instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::system::broadcaster_actor::BroadcasterActor;
use crate::system::landing_detection_actor::LandingDetectionActor;
use crate::system::messages::{RefreshRate, SimConnectDataMessage, StopMessage};
use crate::system::simconnect_objects::{GpsData, OnGround};

#[derive(Debug)]
pub struct SimConnectActor {
    context: OpenTelemetryContext,
    refresh_rate: RefreshRate,
    landing_detection_enabled: bool,
    broadcaster_addr: Addr<BroadcasterActor>,
    landing_detection_addr: Addr<LandingDetectionActor>,
}

impl SimConnectActor {
    pub fn new(
        context: OpenTelemetryContext,
        refresh_rate: RefreshRate,
        landing_detection_enabled: bool,
        broadcaster_addr: Addr<BroadcasterActor>,
        landing_detection_addr: Addr<LandingDetectionActor>,
    ) -> Self {
        Self {
            context,
            refresh_rate,
            landing_detection_enabled,
            broadcaster_addr,
            landing_detection_addr,
        }
    }
}

impl SimConnectActor {
    #[instrument(
        name = "SimConnectActor::poll_simconnect_messages",
        skip(addr, broadcaster_addr, landing_detection_addr)
    )]
    async fn poll_simconnect_messages(
        refresh_rate: RefreshRate,
        landing_detection_enabled: bool,
        addr: Addr<SimConnectActor>,
        broadcaster_addr: Addr<BroadcasterActor>,
        landing_detection_addr: Addr<LandingDetectionActor>,
    ) {
        let result: Result<(), SimConnectError> = async {
            let mut sc = SimConnect::new("Simple Program")?;

            info!("SimConnect SDK: Connected");

            loop {
                if let Some(notification) = sc.get_next_dispatch()? {
                    let span = debug_span!("get_next_dispatch");
                    span.set_parent(Span::current().context());
                    let _ = span.enter();

                    match notification {
                        Notification::Open => {
                            info!("SimConnect SDK: Received Client Open");

                            let (period, interval) = match refresh_rate {
                                RefreshRate::Fast => (Period::VisualFrame, 6u32),
                                RefreshRate::Slow => (Period::Second, 0u32),
                            };

                            let request_id = sc.register_object::<GpsData>()?;
                            sc.request_data_on_sim_object(
                                request_id,
                                period,
                                Condition::None,
                                interval,
                            )?;

                            if landing_detection_enabled {
                                sc.register_object::<OnGround>()?;

                                // subscribe to the airport list
                                sc.subscribe_to_facilities(simconnect_sdk::FacilityType::Airport)?;
                            }
                        }
                        Notification::Quit => {
                            info!("SimConnect SDK: Received Quit");
                            addr.do_send(StopMessage {
                                context: span.context(),
                                reason: "SimConnect SDK: Received Quit".to_string(),
                            })
                        }
                        Notification::Object(data) => {
                            if let Ok(gps_data) = GpsData::try_from(&data) {
                                trace!("SimConnect SDK: Received GpsData");

                                let message = SimConnectDataMessage {
                                    context: span.context(),
                                    data: gps_data,
                                };

                                if landing_detection_enabled {
                                    landing_detection_addr.do_send(message.clone());
                                }

                                broadcaster_addr.do_send(message);

                                continue;
                            }
                            if let Ok(on_ground_data) = OnGround::try_from(&data) {
                                trace!("SimConnect SDK: Received OnGround");

                                let message = SimConnectDataMessage {
                                    context: span.context(),
                                    data: on_ground_data,
                                };

                                landing_detection_addr.do_send(message);

                                continue;
                            }
                        }
                        Notification::AirportList(airports) => {
                            trace!("SimConnect SDK: Received AirportList");

                            let message = SimConnectDataMessage {
                                context: span.context(),
                                data: airports,
                            };

                            landing_detection_addr.do_send(message);
                        }
                        _ => (),
                    }
                }

                let delay = if refresh_rate == RefreshRate::Fast {
                    20
                } else {
                    200
                };

                sleep(std::time::Duration::from_millis(delay)).await;
            }
        }
        .await;

        if let Err(e) = result {
            error!(error = ?e, "SimConnect SDK Error");
            addr.do_send(StopMessage {
                context: Span::current().context(),
                reason: "SimConnect SDK Error".to_string(),
            });
        }
    }
}

impl Actor for SimConnectActor {
    type Context = Context<Self>;

    #[instrument(name = "SimConnectActor::started", skip(self, ctx))]
    fn started(&mut self, ctx: &mut Self::Context) {
        Span::current().set_parent(self.context.clone());

        let fut = Self::poll_simconnect_messages(
            self.refresh_rate,
            self.landing_detection_enabled,
            ctx.address(),
            self.broadcaster_addr.clone(),
            self.landing_detection_addr.clone(),
        )
        .instrument(Span::current())
        .into_actor(self);

        ctx.spawn(fut);

        info!("SimConnectActor started");
    }

    #[instrument(name = "SimConnectActor::stopped", skip(self))]
    fn stopped(&mut self, _: &mut Self::Context) {
        Span::current().set_parent(self.context.clone());
        info!("SimConnectActor stopped");
    }
}

impl Handler<StopMessage> for SimConnectActor {
    type Result = ();

    #[instrument(
        name = "SimConnectActor::handle::<StopMessage>",
        skip(self, message, ctx)
    )]
    fn handle(&mut self, message: StopMessage, ctx: &mut Context<Self>) -> Self::Result {
        Span::current().set_parent(message.context.clone());
        info!(reason = ?message.reason, "SimConnectActor stopping");
        ctx.stop();
    }
}
