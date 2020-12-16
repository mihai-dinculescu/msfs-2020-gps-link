use actix::{Actor, Addr, AsyncContext, Context, Handler, SpawnHandle, WrapFuture};
use tokio::sync::mpsc::Receiver;
use tracing::{field, info, instrument, Span};

use super::{
    broadcaster_actor::BroadcasterActor,
    messages::{CoordinatorMessage, StopMessage},
    simconnect_actor::SimconnectActor,
};

#[derive(Debug)]
pub struct CoordinatorActor {
    pub handle: Option<SpawnHandle>,
    pub rx: Option<Receiver<CoordinatorMessage>>,
    pub broadcaster: Option<Addr<BroadcasterActor>>,
    pub simconnect: Option<Addr<SimconnectActor>>,
}

impl Actor for CoordinatorActor {
    type Context = Context<Self>;

    #[instrument(name = "CoordinatorActorStarted", skip(self, ctx))]
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        let mut rx = self.rx.take().expect("mpsc socket is closed");

        let fut = async move {
            loop {
                let data = rx.recv().await;
                match data {
                    Some(msg) => {
                        addr.try_send(msg).expect("Coordinator mailbox is full");
                    }
                    None => {
                        info!("got None");
                    }
                }
            }
        }
        .into_actor(self);

        let handle = ctx.spawn(fut);
        self.handle = Some(handle);
    }

    #[instrument(name = "CoordinatorActorStopped", skip(self, _ctx))]
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Coordinator stopped");
    }
}

impl Handler<CoordinatorMessage> for CoordinatorActor {
    type Result = ();

    #[instrument(
        name = "CoordinatorActor::Handler<CoordinatorMessage>",
        skip(self, _ctx),
        fields(request_id = field::Empty)
    )]
    fn handle(&mut self, message: CoordinatorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let span = Span::current();

        match message {
            CoordinatorMessage::Start {
                request_id,
                refresh_rate,
                broadcast_netmask,
                broadcast_port,
            } => {
                span.record("request_id", &(request_id.as_str()));
                self.stop_actors();

                let broadcaster = BroadcasterActor {
                    socket: None,
                    broadcast_netmask,
                    broadcast_port,
                }
                .start();

                let simconnect = SimconnectActor {
                    refresh_rate,
                    broadcaster: broadcaster.clone(),
                }
                .start();

                self.broadcaster = Some(broadcaster);
                self.simconnect = Some(simconnect);
            }
            CoordinatorMessage::Stop { request_id } => {
                span.record("request_id", &(request_id.as_str()));
                self.stop_actors();
            }
            CoordinatorMessage::Status {
                request_id,
                response_channel,
            } => {
                span.record("request_id", &(request_id.as_str()));
                let mut response = true;

                if let Some(addr) = &self.broadcaster {
                    if !addr.connected() {
                        response = false;
                    }
                } else {
                    response = false;
                }

                if let Some(addr) = &self.simconnect {
                    if !addr.connected() {
                        response = false;
                    }
                } else {
                    response = false;
                }

                let _ = response_channel.send(response);
            }
        }
    }
}

impl CoordinatorActor {
    #[instrument(name = "CoordinatorActor::stop_actors", skip(self))]
    fn stop_actors(&mut self) {
        if let Some(addr) = &self.broadcaster {
            if addr.connected() {
                addr.try_send(StopMessage)
                    .expect("Broadcaster queue is full");
            }
            self.broadcaster = None;
        }

        if let Some(addr) = &self.simconnect {
            if addr.connected() {
                addr.try_send(StopMessage)
                    .expect("Simconnect queue is full");
            }
            self.simconnect = None;
        }
    }
}
