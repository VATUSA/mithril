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
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_redoc::{Redoc, Servable};

mod change_poller;
mod db;
mod middleware;
mod queries;
mod routes;
mod shared;

const API_DESCRIPTION: &str = r#"VATUSA API.

This API is primarily for VATUSA ARTCCs to be able to interact with the VATUSA
site programmatically. Some routes do not require an API key that facilities have;
these routes can be used by anyone. Some API keys may be granted to users for
specific non-facility use.

Authorization with an API key, facility or otherwise, is done via the `X-API-Key`
header in the request. If this header is supplied, it **must be correct**; supplying
an API key that is invalid will result in an error response, even if the endpoint
allows calling without a key. Some public methods will return additional data when
an API key is included in the request.

This documentation is generated from the code and should always be up to date.
"#;

/// VATUSA API v3.

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    #[arg(long, default_value_t = 4000)]
    port: u16,
}

// https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
/// Install the SIGINT/SIGTERM handlers and fan the result out over a watch
/// channel, returning a receiver that consumers turn into futures via
/// [`shutdown_signal`].
///
/// Registration happens synchronously here rather than inside the task that
/// awaits the signal. Building the signal stream lazily (i.e. inside an `async`
/// block) only registers it when that block is *first polled*, so a signal
/// arriving beforehand is missed permanently — and because the first
/// registration already replaces the default SIGTERM disposition, the process
/// then ignores the signal instead of dying. Fanning out through a watch channel
/// (rather than calling this once per consumer) means a consumer that starts
/// polling late still observes a shutdown that already happened.
fn install_shutdown_handler() -> Result<tokio::sync::watch::Receiver<bool>> {
    let (tx, rx) = tokio::sync::watch::channel(false);

    #[cfg(unix)]
    {
        let mut interrupt = signal::unix::signal(signal::unix::SignalKind::interrupt())
            .context("Could not install the SIGINT handler")?;
        let mut terminate = signal::unix::signal(signal::unix::SignalKind::terminate())
            .context("Could not install the SIGTERM handler")?;
        tokio::spawn(async move {
            tokio::select! {
                _ = interrupt.recv() => {},
                _ = terminate.recv() => {},
            }
            tracing::warn!("got terminate signal");
            let _ = tx.send(true);
        });
    }

    #[cfg(not(unix))]
    tokio::spawn(async move {
        let _ = signal::ctrl_c().await;
        tracing::warn!("got terminate signal");
        let _ = tx.send(true);
    });

    Ok(rx)
}

/// Resolve once shutdown has been signalled, including when it was signalled
/// before this future was first polled.
async fn shutdown_signal(mut rx: tokio::sync::watch::Receiver<bool>) {
    // A send error means the sender was dropped, which also implies shutdown.
    while !*rx.borrow_and_update() {
        if rx.changed().await.is_err() {
            break;
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    info(
        license(name = "VATUSA Website Non-Commercial Public License (VS-NCPL) v1.0"),
        contact(email = "vatusa6@vatusa.net"),
        description=API_DESCRIPTION
    ),
    tags(
        (name = "events", description = "Network events"),
        (name = "news", description = "Division and facility news"),
        (name = "facility", description = "Division facility data"),
        (name = "webhooks", description = "Outbound webhook registration"),
    )
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi
            .components
            .get_or_insert_with(utoipa::openapi::Components::default);
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("x-api-key"))),
        );
    }
}

/// Prefixes every documented path with `/v3` so Redoc's sidebar/operation
/// headings match the public URL, even though the ingress strips that
/// segment before requests reach this app. Must run on the fully-merged
/// `OpenApi` doc (after `split_for_parts`), since path items are only
/// added to it as each router's `routes!()` is composed.
fn prefix_paths(openapi: &mut utoipa::openapi::OpenApi) {
    let paths = std::mem::take(&mut openapi.paths.paths);
    openapi.paths.paths = paths
        .into_iter()
        .map(|(path, item)| (format!("/v3{path}"), item))
        .collect();
}

/// Health-check endpoint
#[utoipa::path(
    method(get, head),
    path = "/health",
    tag = "meta",
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
    tag = "meta",
    responses(
        (status = 200, description = "JSON file", body = str, content_type = "application/json")
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(
        OPENAPI_DOC
            .get()
            .expect("openapi doc set during startup")
            .clone(),
    )
}

static OPENAPI_DOC: std::sync::OnceLock<utoipa::openapi::OpenApi> = std::sync::OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let _ = dotenvy::dotenv();

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,mithril=debug")),
        )
        .finish();
    tracing::subscriber::set_global_default(subscriber).context("Unable to configure logging")?;

    tracing::debug!("connecting to databases ...");
    let app_state = Arc::new(shared::AppState {
        vatusa_db: connect_vatusa().await.context("vatusa db")?,
        cobalt_db: connect_cobalt().await.context("cobalt db")?,
    });
    tracing::debug!("connected");

    tracing::debug!("setting up app ...");
    let (app, mut api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .with_state(app_state.clone())
        .routes(routes!(health))
        .routes(routes!(openapi))
        .nest("/news", routes::news::router(app_state.clone()))
        .nest("/events", routes::events::router(app_state.clone()))
        .nest("/facility", routes::facility::router(app_state.clone()))
        .nest("/webhooks", routes::webhooks::router(app_state.clone()))
        .fallback(routes::fallback)
        .split_for_parts();
    prefix_paths(&mut api);
    if OPENAPI_DOC.set(api.clone()).is_err() {
        panic!("openapi doc set exactly once at startup");
    }

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
    tracing::debug!("set up");

    let shutdown_rx = install_shutdown_handler()?;

    let poller_handle = if std::env::var("DISABLE_ROSTER_POLLER")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false)
    {
        tracing::info!("roster poll task disabled via DISABLE_ROSTER_POLLER");
        None
    } else {
        tracing::info!("starting roster poll task");
        Some(tokio::spawn(change_poller::run(
            app_state.vatusa_db.clone(),
            app_state.cobalt_db.clone(),
            shutdown_signal(shutdown_rx.clone()),
        )))
    };

    let host_and_port = format!("{}:{}", cli.host, cli.port);
    tracing::info!("listening on http://{host_and_port}/");
    let listener = tokio::net::TcpListener::bind(&host_and_port)
        .await
        .context("Could not bind the HTTP listener")?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(shutdown_rx))
        .await
        .context("Could not serve the app")?;

    if let Some(h) = poller_handle {
        let _ = h.await;
    }

    Ok(())
}
