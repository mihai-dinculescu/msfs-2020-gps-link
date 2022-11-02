use actix::{Actor, ActorContext, Context, Handler};
use opentelemetry_api::Context as OpenTelemetryContext;
use simconnect_sdk::Airport;
use tracing::{info, instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::system::messages::{SimConnectDataMessage, StopMessage};
use crate::system::simconnect_objects::{GpsData, OnGround};

#[derive(Debug, Default)]
pub struct LandingDetectionActor {
    context: OpenTelemetryContext,
    last_gps_data: Option<GpsData>,
    take_off_gps_data: Option<GpsData>,
    in_the_air: bool,
}

impl LandingDetectionActor {
    pub fn new(context: OpenTelemetryContext) -> Self {
        Self {
            context,
            ..Default::default()
        }
    }
}

impl Actor for LandingDetectionActor {
    type Context = Context<Self>;

    #[instrument(name = "LandingActor::started", skip(self))]
    fn started(&mut self, _: &mut Self::Context) {
        Span::current().set_parent(self.context.clone());
        info!("LandingDetectionActor started");
    }

    #[instrument(name = "LandingActor::stopped", skip(self))]
    fn stopped(&mut self, _: &mut Self::Context) {
        Span::current().set_parent(self.context.clone());
        info!("LandingDetectionActor stopped");
    }
}

impl Handler<SimConnectDataMessage<OnGround>> for LandingDetectionActor {
    type Result = ();

    #[instrument(
        name = "LandingDetectionActor::handle::<SimConnectDataMessage<OnGround>>",
        skip(self, message)
    )]
    fn handle(
        &mut self,
        message: SimConnectDataMessage<OnGround>,
        _: &mut Context<Self>,
    ) -> Self::Result {
        Span::current().set_parent(message.context);
        let data = message.data;

        // TODO: rework this
        if data.sim_on_ground
            && self.in_the_air
            && self.last_gps_data.is_some()
            && self.take_off_gps_data.is_some()
        {
            // landing
            println!("landing {:?}", self.last_gps_data);
            self.take_off_gps_data = self.last_gps_data.clone();
            self.in_the_air = false;
        } else if !data.sim_on_ground && !self.in_the_air && self.last_gps_data.is_some() {
            // take-off
            println!("take-off {:?}", self.last_gps_data);
            self.take_off_gps_data = self.last_gps_data.clone();
            self.in_the_air = true;
        } else if data.sim_on_ground {
            // reset
            println!("reset");
            self.take_off_gps_data = None;
            self.in_the_air = false;
        }
    }
}

impl Handler<SimConnectDataMessage<GpsData>> for LandingDetectionActor {
    type Result = ();

    #[instrument(
        name = "LandingDetectionActor::handle::<SimConnectDataMessage<GpsData>>",
        skip(self, message)
    )]
    fn handle(
        &mut self,
        message: SimConnectDataMessage<GpsData>,
        _: &mut Context<Self>,
    ) -> Self::Result {
        Span::current().set_parent(message.context);
        self.last_gps_data = Some(message.data);
    }
}

impl Handler<SimConnectDataMessage<Vec<Airport>>> for LandingDetectionActor {
    type Result = ();

    #[instrument(
        name = "LandingDetectionActor::handle::<SimConnectDataMessage<Vec<Airport>>",
        skip(self, message)
    )]
    fn handle(
        &mut self,
        message: SimConnectDataMessage<Vec<Airport>>,
        _: &mut Context<Self>,
    ) -> Self::Result {
        Span::current().set_parent(message.context);
        let _ = message.data;

        // TODO: implement this
    }
}

impl Handler<StopMessage> for LandingDetectionActor {
    type Result = ();

    #[instrument(
        name = "LandingDetectionActor::handle::<StopMessage>",
        skip(self, message, ctx)
    )]
    fn handle(&mut self, message: StopMessage, ctx: &mut Context<Self>) -> Self::Result {
        Span::current().set_parent(message.context.clone());
        info!(reason = ?message.reason, "LandingDetectionActor stopping");
        ctx.stop();
    }
}
