use actix::{clock::sleep, Actor, ActorContext, Addr, AsyncContext, Context, Handler, WrapFuture};
use chrono::Utc;
use simconnect_client::{ConditionEnum, Notification, PeriodEnum, SimConnect};
use tracing::{error, info, instrument};

use crate::system::broadcaster_actor::BroadcasterActor;
use crate::system::landing_detection_actor::LandingDetectionActor;
use crate::system::messages::{
    AirportListMessage, GpsDataMessage, OnGroundMessage, RefreshRate, StopMessage,
};

#[derive(Debug)]
pub struct SimconnectActor {
    pub refresh_rate: RefreshRate,
    pub landing_detection_enabled: bool,
    pub broadcaster: Addr<BroadcasterActor>,
    pub landing_detection: Addr<LandingDetectionActor>,
}

impl Actor for SimconnectActor {
    type Context = Context<Self>;

    #[instrument(name = "SimconnectActorStarted", skip(self, ctx))]
    fn started(&mut self, ctx: &mut Self::Context) {
        let refresh_rate = self.refresh_rate;
        let landing_detection_enabled = self.landing_detection_enabled;
        let addr = ctx.address();
        let broadcaster_addr = self.broadcaster.clone();
        let landing_detection_addr = self.landing_detection.clone();

        let sc = match SimConnect::new("Simple Program") {
            Ok(sc) => Some(sc),
            Err(_) => {
                addr.do_send(StopMessage);
                None
            }
        };

        if let Some(sc) = sc {
            info!("Connected");

            let fut = async move {
                let mut last_update = Utc::now();

                loop {
                    let gps_data_request_id = 0;
                    let on_ground_request_id = 1;
                    let airport_list_request_id = 2;

                    let notification = sc.get_next_dispatch().unwrap();

                    match notification {
                        Some(Notification::Open) => {
                            info!("Open");

                            let period = match refresh_rate {
                                RefreshRate::Fast => PeriodEnum::VisualFrame { interval: 5 },
                                RefreshRate::Slow => PeriodEnum::Second,
                            };

                            // GPS data: define
                            sc.add_to_data_definition(
                                gps_data_request_id,
                                "PLANE LATITUDE",
                                "Degrees",
                            )
                            .unwrap_or_else(|_| {
                                addr.do_send(StopMessage);
                            });
                            sc.add_to_data_definition(
                                gps_data_request_id,
                                "PLANE LONGITUDE",
                                "Degrees",
                            )
                            .unwrap_or_else(|_| {
                                addr.do_send(StopMessage);
                            });
                            sc.add_to_data_definition(
                                gps_data_request_id,
                                "PLANE ALTITUDE",
                                "Meters",
                            )
                            .unwrap_or_else(|_| {
                                addr.do_send(StopMessage);
                            });
                            sc.add_to_data_definition(
                                gps_data_request_id,
                                "GPS GROUND MAGNETIC TRACK",
                                "Degrees",
                            )
                            .unwrap_or_else(|_| {
                                addr.do_send(StopMessage);
                            });
                            sc.add_to_data_definition(gps_data_request_id, "MAGVAR", "Degrees")
                                .unwrap_or_else(|_| {
                                    addr.do_send(StopMessage);
                                });
                            sc.add_to_data_definition(
                                gps_data_request_id,
                                "GPS GROUND SPEED",
                                "Meters per second",
                            )
                            .unwrap_or_else(|_| {
                                addr.do_send(StopMessage);
                            });

                            // GPS data: request
                            sc.request_data_on_sim_object(
                                gps_data_request_id,
                                period.clone(),
                                ConditionEnum::None,
                            )
                            .unwrap_or_else(|_| {
                                addr.do_send(StopMessage);
                            });

                            if landing_detection_enabled {
                                // Ground status: define
                                sc.add_to_data_definition(
                                    on_ground_request_id,
                                    "SIM ON GROUND",
                                    "Bool",
                                )
                                .unwrap_or_else(|_| {
                                    addr.do_send(StopMessage);
                                });

                                // Ground status: request
                                sc.request_data_on_sim_object(
                                    on_ground_request_id,
                                    period,
                                    ConditionEnum::Changed,
                                )
                                .unwrap_or_else(|_| {
                                    addr.do_send(StopMessage);
                                });

                                // subscribe to the airport list
                                sc.subscribe_to_airport_list(airport_list_request_id)
                                    .unwrap_or_else(|_| {
                                        addr.do_send(StopMessage);
                                    });
                            }
                        }
                        Some(Notification::Quit) => {
                            info!("Quit");
                            addr.do_send(StopMessage)
                        }
                        Some(Notification::Event(e)) => info!("Event: {:?}", e),
                        Some(Notification::Data(define_id, data)) => match define_id {
                            define_id if define_id == gps_data_request_id => {
                                if last_update + chrono::Duration::milliseconds(100) < Utc::now() {
                                    let sim_data: &GpsDataMessage =
                                        unsafe { std::mem::transmute_copy(&data) };
                                    last_update = Utc::now();
                                    broadcaster_addr.do_send(sim_data.clone());

                                    if landing_detection_enabled {
                                        landing_detection_addr.do_send(sim_data.clone());
                                    }
                                }
                            }
                            define_id if define_id == on_ground_request_id => {
                                let sim_data: &OnGroundMessage =
                                    unsafe { std::mem::transmute_copy(&data) };

                                landing_detection_addr.do_send(sim_data.clone());
                            }
                            _ => (),
                        },
                        Some(Notification::AirportList(airports)) => {
                            landing_detection_addr.do_send(AirportListMessage {
                                data: airports.clone(),
                            });
                        }
                        Some(Notification::Exception(ex)) => {
                            error!("SimConnect Exception: {}", ex);
                        }
                        None => (),
                    }

                    let delay = if refresh_rate == RefreshRate::Fast {
                        20
                    } else {
                        200
                    };

                    sleep(std::time::Duration::from_millis(delay)).await;
                }
            }
            .into_actor(self);

            ctx.spawn(fut);
        }
    }

    #[instrument(name = "SimconnectActorStopped", skip(self, _ctx))]
    fn stopped(&mut self, _ctx: &mut Self::Context) {}
}

impl Handler<StopMessage> for SimconnectActor {
    type Result = ();

    fn handle(&mut self, _: StopMessage, ctx: &mut Context<Self>) -> Self::Result {
        ctx.stop();
    }
}
