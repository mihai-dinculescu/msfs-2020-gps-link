#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{thread, time};

use actix::{Actor, Arbiter};
use cmd::{Cmd, CommandError, Response};
use system::{coordinator_actor::CoordinatorActor, messages::CoordinatorMessage};
use tokio::sync;
use tracing::{debug, info, span, subscriber, Level};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;

mod cmd;
mod system;

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

    tauri::AppBuilder::new()
        .invoke_handler(move |_webview, arg| match serde_json::from_str(arg) {
            Err(e) => Err(e.to_string()),
            Ok(command) => {
                let tx_local = tx.clone();

                match command {
                    Cmd::Start {
                        request_id,
                        options,
                        callback,
                        error,
                    } => {
                        let func = move || {
                            span!(
                                Level::INFO,
                                "Command",
                                command = "Start",
                                request_id = &(request_id.as_str())
                            );

                            let result = tx_local.blocking_send(CoordinatorMessage::Start {
                                request_id,
                                refresh_rate: options.refresh_rate,
                                broadcast_netmask: options.broadcast_netmask,
                                broadcast_port: options.broadcast_port,
                            });
                            drop(tx_local);

                            match result {
                                Ok(_) => {
                                    info!(message = "OK");
                                    Ok(Response { message: "OK" })
                                }
                                Err(e) => {
                                    debug!(error = ?e, "Sending Start");
                                    Err(CommandError::new(e.to_string()).into())
                                }
                            }
                        };

                        tauri::execute_promise(_webview, func, callback, error)
                    }
                    Cmd::Stop {
                        request_id,
                        callback,
                        error,
                    } => {
                        let func = move || {
                            span!(
                                Level::INFO,
                                "Command",
                                command = "Stop",
                                request_id = &(request_id.as_str())
                            );
                            let result =
                                tx_local.blocking_send(CoordinatorMessage::Stop { request_id });
                            drop(tx_local);

                            match result {
                                Ok(_) => {
                                    info!(message = "OK");
                                    Ok(Response { message: "OK" })
                                }
                                Err(e) => {
                                    debug!(error = ?e, "Sending Stop");
                                    Err(CommandError::new(e.to_string()).into())
                                }
                            }
                        };

                        tauri::execute_promise(_webview, func, callback, error)
                    }
                    Cmd::Status {
                        request_id,
                        callback,
                        error,
                    } => {
                        let func = move || {
                            span!(
                                Level::INFO,
                                "Command",
                                command = "Status",
                                request_id = &(request_id.as_str())
                            );
                            let (response_tx, mut response_rx) = sync::oneshot::channel::<bool>();
                            let result = tx_local.blocking_send(CoordinatorMessage::Status {
                                request_id,
                                response_channel: response_tx,
                            });
                            drop(tx_local);

                            match result {
                                Ok(_) => {
                                    let mut counter = 0;

                                    loop {
                                        let response = response_rx.try_recv();

                                        match response {
                                            Ok(value) => {
                                                let message = match value {
                                                    true => "OK",
                                                    false => "ERROR",
                                                };

                                                info!(message = ?message, "Status response");

                                                return Ok(Response { message });
                                            }
                                            Err(_) => {
                                                thread::sleep(time::Duration::from_millis(100));
                                            }
                                        };

                                        counter += 1;

                                        if counter >= 5 {
                                            break;
                                        }
                                    }
                                    Err(CommandError::new("Timeout".to_string()).into())
                                }
                                Err(e) => {
                                    debug!(error = ?e, "Sending Status");
                                    Err(CommandError::new(e.to_string()).into())
                                }
                            }
                        };

                        tauri::execute_promise(_webview, func, callback, error)
                    }
                }

                Ok(())
            }
        })
        .build()
        .run();

    arbiter.stop();
}

fn setup_logging() {
    let tracer = opentelemetry_jaeger::new_pipeline()
        .install_simple()
        .expect("asd");

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
