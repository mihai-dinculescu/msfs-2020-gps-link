#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use actix::{Actor, Arbiter};
use tokio::sync;
use tracing::subscriber;
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;

mod cmd;
mod system;

use cmd::{do_start, do_status, do_stop, AppState};
use system::{coordinator_actor::CoordinatorActor, messages::CoordinatorMessage};

#[actix::main]
async fn main() {
    setup_logging();
    setup_app();
}

#[tracing::instrument]
fn setup_app() {
    let (tx, rx) = sync::mpsc::channel::<CoordinatorMessage>(8);

    let arbiter = Arbiter::new();
    arbiter.spawn(async {
        let actor = CoordinatorActor {
            handle: None,
            rx: Some(rx),
            broadcaster: None,
            landing_detection: None,
            simconnect: None,
        };

        actor.start();
    });

    tauri::Builder::default()
        .manage(AppState { tx })
        .invoke_handler(tauri::generate_handler![do_start, do_stop, do_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    arbiter.stop();
}

fn setup_logging() {
    let tracer = opentelemetry_jaeger::new_pipeline()
        .install_simple()
        .expect("failed to set up tracing");

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber = Registry::default()
        .with(
            tracing_subscriber::fmt::Layer::default()
                .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL),
        )
        .with(EnvFilter::from_default_env())
        .with(telemetry);

    subscriber::set_global_default(subscriber).expect("setting global default failed");

    LogTracer::init().expect("Could not initialize LogTracer.");
}
