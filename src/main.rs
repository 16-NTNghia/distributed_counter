use std::panic::AssertUnwindSafe;

use app::build_app;
use axum::{Router, routing::post, serve};
use config::{
    env_config::load_env,
    log_config::{log_request, log_response},
};
use futures::FutureExt;
use infrastructure::{kafka::consumer, scylladb::session::create_session};
use interface::http::handler::kafka_handler;
use tokio::{net::TcpListener, spawn};
use tower_http::trace::TraceLayer;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

mod app;
mod application;
mod config;
mod domain;
mod dto;
mod infrastructure;
mod interface;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .init();

    info!("Starting server...");

    load_env();

    let session = create_session().await;

    let app_state = build_app(session);

    let app = Router::new()
        .route("/send", post(kafka_handler::send))
        .route("/count", post(kafka_handler::get_count))
        .with_state(app_state.clone())
        .layer(
            TraceLayer::new_for_http()
                .on_request(log_request)
                .on_response(log_response),
        );
    let addr = TcpListener::bind("0.0.0.0:1609")
        .await
        .expect("Failed to bind to address");

    info!("Listening on {:?}", addr.local_addr());

    spawn(async move {
        if let Err(e) = AssertUnwindSafe(consumer::start(app_state.clone()))
            .catch_unwind()
            .await
        {
            error!("Consumer crashed: {:?}", e);
        }
    });

    if let Err(e) = serve(addr, app).await {
        error!("Failed to start server: {:?}", e);
    }
}
