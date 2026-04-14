//! VATUSA API.

#![deny(clippy::all)]
#![deny(unsafe_code)]

use crate::{
    db::{connect_cobalt, connect_vatusa},
    middleware::auth_middleware,
};
use anyhow::{Context, Result};
use axum::Json;
use clap::Parser;
use std::{sync::Arc, time::Duration};
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_redoc::{Redoc, Servable};

mod db;
mod middleware;
mod queries;
mod routes;
mod shared;

const API_DESCRIPTION: &'static str = r#"VATUSA API.

This API is primarily for VATUSA ARTCCs to be able to interact with the VATUSA
site programatically. Some routes do not require an API key that facilities have;
these routes can be used by anyone.

Authorization with an API key, facility or otherwise, is done via the `X-API-Key`
header in the request. If this header is supplied, it **must be correct**; supplying
an API key that is invalid will result in an error response. Some public methods
will return additional data when an API key is included in the request.
"#;

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

#[derive(OpenApi)]
#[openapi(info(
    license(name = "VATUSA Website Non-Commercial Public License (VS-NCPL) v1.0"),
    contact(name = "VATUSA6", email = "vatusa6@vatusa.net"),
    description=API_DESCRIPTION
))]
struct ApiDoc;

/// Get health of the API.
#[utoipa::path(
    method(get, head),
    path = "/health",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
async fn health() -> &'static str {
    "ok"
}

/// Return JSON version of an OpenAPI schema
#[utoipa::path(
    get,
    path = "/swagger.json",
    responses(
        (status = 200, description = "JSON file", body = str, content_type = "application/json")
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let _ = dotenv::dotenv();

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).context("Unable to configure logging")?;

    let app_state = Arc::new(shared::AppState {
        vatusa_db: connect_vatusa().await.context("vatusa db")?,
        cobalt_db: connect_cobalt().await.context("cobalt db")?,
    });

    let (app, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .with_state(app_state.clone())
        .routes(routes!(health))
        .routes(routes!(openapi))
        .nest("/news", routes::news::router(app_state.clone()))
        .nest("/events", routes::events::router(app_state.clone()))
        .fallback(routes::fallback)
        .split_for_parts();

    let app = app.merge(Redoc::with_url("/", api)).layer(
        ServiceBuilder::new()
            .layer(tower_http::trace::TraceLayer::new_for_http())
            .layer(TimeoutLayer::with_status_code(
                http::StatusCode::GATEWAY_TIMEOUT,
                Duration::from_secs(60),
            ))
            .layer(axum::middleware::from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            )),
    );

    let host_and_port = format!("{}:{}", cli.host, cli.port);
    tracing::info!("Listening on http://{host_and_port}/");
    let listener = tokio::net::TcpListener::bind(&host_and_port)
        .await
        .context("Could not bind the HTTP listener")?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Could not serve the app")?;

    Ok(())
}
