use actix::{
    clock::delay_for, Actor, ActorContext, Addr, AsyncContext, Context, Handler, WrapFuture,
};
use chrono::Utc;
use simconnect_client::{Notification, PeriodEnum, SimConnect};
use tracing::{info, instrument};

use crate::system::{broadcaster_actor::BroadcasterActor, messages::GpsData};

use super::messages::{RefreshRate, StopMessage};

#[derive(Debug)]
pub struct SimconnectActor {
    pub refresh_rate: RefreshRate,
    pub broadcaster: Addr<BroadcasterActor>,
}

impl Actor for SimconnectActor {
    type Context = Context<Self>;

    #[instrument(name = "SimconnectActorStarted", skip(self, ctx))]
    fn started(&mut self, ctx: &mut Self::Context) {
        let refresh_rate = self.refresh_rate;
        let addr = ctx.address();
        let broadcaster_addr = self.broadcaster.clone();

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
                let gps_data_define_id = 0;

                let mut last_update = Utc::now();

                loop {
                    let notification = sc.get_next_dispatch().unwrap();

                    match notification {
                        Some(Notification::Open) => {
                            info!("Open");

                            sc.add_to_data_definition(
                                gps_data_define_id,
                                "PLANE LATITUDE",
                                "Degrees",
                            )
                            .unwrap();
                            sc.add_to_data_definition(
                                gps_data_define_id,
                                "PLANE LONGITUDE",
                                "Degrees",
                            )
                            .unwrap();
                            sc.add_to_data_definition(
                                gps_data_define_id,
                                "PLANE ALTITUDE",
                                "Meters",
                            )
                            .unwrap(); //define_id, units, data_type, datum_id
                            sc.add_to_data_definition(
                                gps_data_define_id,
                                "PLANE HEADING DEGREES TRUE",
                                "Degrees",
                            )
                            .unwrap();
                            sc.add_to_data_definition(
                                gps_data_define_id,
                                "GPS GROUND SPEED",
                                "Meters per second",
                            )
                            .unwrap();

                            let period = match refresh_rate {
                                RefreshRate::Fast => PeriodEnum::VisualFrame,
                                RefreshRate::Slow => PeriodEnum::Second,
                            };

                            sc.request_data_on_sim_object(0, gps_data_define_id, 0, period)
                                .unwrap();
                        }
                        Some(Notification::Quit) => {
                            info!("Quit");
                            addr.do_send(StopMessage)
                        }
                        Some(Notification::Event(e)) => info!("Event: {:?}", e),
                        Some(Notification::Data(define_id, data)) => match define_id {
                            define_id if define_id == gps_data_define_id => {
                                if last_update + chrono::Duration::milliseconds(100) < Utc::now() {
                                    let sim_data: &GpsData =
                                        unsafe { std::mem::transmute_copy(&data) };
                                    last_update = Utc::now();
                                    broadcaster_addr.do_send(sim_data.clone());
                                }
                            }
                            _ => (),
                        },
                        None => (),
                    }

                    let delay = if refresh_rate == RefreshRate::Fast {
                        8
                    } else {
                        250
                    };

                    delay_for(std::time::Duration::from_millis(delay)).await;
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
