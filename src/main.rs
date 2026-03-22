//! VATUSA API.

use axum::{Router, routing::get};
use clap::Parser;
use std::time::Duration;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;

/// VATUSA API.
#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    #[arg(long, default_value_t = 4000)]
    port: u16,
}

// https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
        tracing::warn!("Got terminate signal");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
        tracing::warn!("Got terminate signal");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Unable to configure logging");

    let home = Router::new().route("/", get(|| async { "Hello, World!" }));
    let app = Router::new().merge(home).layer(
        ServiceBuilder::new()
            .layer(tower_http::trace::TraceLayer::new_for_http())
            .layer(TimeoutLayer::with_status_code(
                http::StatusCode::GATEWAY_TIMEOUT,
                Duration::from_secs(30),
            )),
    );

    let host_and_port = format!("{}:{}", cli.host, cli.port);
    tracing::info!("Listening on http://{host_and_port}/");
    let listener = tokio::net::TcpListener::bind(&host_and_port)
        .await
        .expect("Could not bind the HTTP listener");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Could not serve the app");
}
