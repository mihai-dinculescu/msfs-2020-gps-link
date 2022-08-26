use actix::{Actor, ActorContext, Context, Handler};
use tracing::instrument;

use super::messages::{AirportListMessage, GpsDataMessage, OnGroundMessage, StopMessage};

#[derive(Debug, Default)]
pub struct LandingDetectionActor {
    last_gps_data: Option<GpsDataMessage>,
    take_off_gps_data: Option<GpsDataMessage>,
    in_the_air: bool,
}

impl Actor for LandingDetectionActor {
    type Context = Context<Self>;

    #[instrument(name = "LandingActor::Started", skip(self, _ctx))]
    fn started(&mut self, _ctx: &mut Self::Context) {}

    #[instrument(name = "LandingActor::Stopped", skip(self, _ctx))]
    fn stopped(&mut self, _ctx: &mut Self::Context) {}
}

impl Handler<OnGroundMessage> for LandingDetectionActor {
    type Result = ();

    #[instrument(name = "LandingDetectionActor::Handler<OnGround>", skip(self, _ctx))]
    fn handle(&mut self, sim_data: OnGroundMessage, _ctx: &mut Context<Self>) -> Self::Result {
        // TODO: rework this
        if sim_data.sim_on_ground == 1.0
            && self.in_the_air
            && self.last_gps_data.is_some()
            && self.take_off_gps_data.is_some()
        {
            // landing
            println!("landing {:?}", self.last_gps_data);
            self.take_off_gps_data = self.last_gps_data.clone();
            self.in_the_air = false;
        } else if sim_data.sim_on_ground == 0.0 && !self.in_the_air && self.last_gps_data.is_some()
        {
            // take-off
            println!("take-off {:?}", self.last_gps_data);
            self.take_off_gps_data = self.last_gps_data.clone();
            self.in_the_air = true;
        } else if sim_data.sim_on_ground == 1.0 {
            // reset
            println!("reset");
            self.take_off_gps_data = None;
            self.in_the_air = false;
        }
    }
}

impl Handler<GpsDataMessage> for LandingDetectionActor {
    type Result = ();

    #[instrument(name = "LandingDetectionActor::Handler<GpsData>", skip(self, _ctx))]
    fn handle(&mut self, sim_data: GpsDataMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self.last_gps_data = Some(sim_data);
    }
}

impl Handler<AirportListMessage> for LandingDetectionActor {
    type Result = ();

    #[instrument(
        name = "LandingDetectionActor::Handler<AirportListMessage>",
        skip(self, _ctx)
    )]
    fn handle(&mut self, sim_data: AirportListMessage, _ctx: &mut Context<Self>) -> Self::Result {
        // TODO: implement this
    }
}

impl Handler<StopMessage> for LandingDetectionActor {
    type Result = ();

    #[instrument(name = "LandingDetectionActor::Handler<StopMessage>", skip(self, ctx))]
    fn handle(&mut self, _: StopMessage, ctx: &mut Context<Self>) -> Self::Result {
        ctx.stop();
    }
}
