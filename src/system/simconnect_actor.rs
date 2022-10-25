use actix::{clock::sleep, Actor, ActorContext, Addr, AsyncContext, Context, Handler, WrapFuture};
use chrono::Utc;
use simconnect_sdk::{Condition, Notification, Period, SimConnect, SimConnectError};
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

        let fut = async move {
            let result: Result<(), SimConnectError> = async {
                let mut sc = SimConnect::new("Simple Program")?;

                info!("Connected");
                let mut last_update = Utc::now();

                loop {
                    let notification = sc.get_next_dispatch()?;

                    match notification {
                        Some(Notification::Open) => {
                            info!("Open");

                            let (period, interval) = match refresh_rate {
                                RefreshRate::Fast => (Period::VisualFrame, 5u32),
                                RefreshRate::Slow => (Period::Second, 0u32),
                            };

                            let request_id = sc.register_object::<GpsDataMessage>()?;
                            sc.request_data_on_sim_object(
                                request_id,
                                period,
                                Condition::None,
                                interval,
                            )?;

                            if landing_detection_enabled {
                                if sc.register_object::<OnGroundMessage>().is_err() {
                                    addr.do_send(StopMessage);
                                }

                                // subscribe to the airport list
                                sc.subscribe_to_facilities(simconnect_sdk::FacilityType::Airport)?;
                            }
                        }
                        Some(Notification::Quit) => {
                            info!("Quit");
                            addr.do_send(StopMessage)
                        }
                        Some(Notification::Event(e)) => info!("Event: {:?}", e),
                        Some(Notification::Object(data)) => {
                            if let Ok(gps_data) = GpsDataMessage::try_from(&data) {
                                if last_update + chrono::Duration::milliseconds(100) < Utc::now() {
                                    last_update = Utc::now();
                                    broadcaster_addr.do_send(gps_data.clone());

                                    if landing_detection_enabled {
                                        landing_detection_addr.do_send(gps_data.clone());
                                    }
                                }
                                continue;
                            }
                            if let Ok(on_ground_data) = OnGroundMessage::try_from(&data) {
                                landing_detection_addr.do_send(on_ground_data.clone());
                                continue;
                            }
                        }
                        Some(Notification::AirportList(airports)) => {
                            landing_detection_addr.do_send(AirportListMessage {
                                data: airports.clone(),
                            });
                        }
                        _ => (),
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
                error!("SimConnect error: {}", e);
                addr.do_send(StopMessage);
            }
        }
        .into_actor(self);

        ctx.spawn(fut);
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
