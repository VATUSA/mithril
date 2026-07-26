//! Events routes.

use crate::{
    db::Event,
    middleware::RequireAuth,
    queries::{self, CreateEvent, UpdateEvent},
    shared::{AppError, AppState, determine_facility},
};
use axum::{
    Json,
    extract::{self, Path, State},
};
use http::StatusCode;
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};

/// Register routes.
pub fn router(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_events, create_event))
        .routes(routes!(get_single_event, update_event, delete_event))
        .with_state(state)
}

/// Retrieve events
#[utoipa::path(
    get,
    path = "/",
    tag = "events",
    responses(
        (status = 200, description = "Events", body = [Event]),
        (status = 500, description = "Server error")
    )
)]
async fn get_events(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Event>>, AppError> {
    let events = queries::get_events(&state.cobalt_db).await?;
    Ok(Json(events))
}

/// Retrieve a single event
#[utoipa::path(
    get,
    path = "/{id}",
    tag = "events",
    responses(
        (status = 200, description = "Event", body = Event),
        (status = 404, description = "No matching event found"),
        (status = 500, description = "Server error")
    ),
    params(
        ("id" = i32, Path, description = "Event ID")
    )
)]
async fn get_single_event(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Event>, AppError> {
    let event = queries::get_event(&state.cobalt_db, id).await?;
    match event {
        Some(e) => Ok(Json(e)),
        None => Err(AppError::NotFound("No event with given id found")),
    }
}

/// Create an event
///
/// Facilities should omit the "facility" field in the payload; this field exists
/// so VATUSA staff can add a specific event should they need to. The facility field
/// will resolve to the facility who owns the API key used in the request.
#[utoipa::path(
    post,
    path = "/",
    tag = "events",
    request_body = CreateEvent,
    responses(
        (status = 201, description = "Event created"),
        (status = 400, description = "Malformed request"),
        (status = 401, description = "Must be called with an API key"),
        (status = 405, description = "Must be called with POST"),
        (status = 422, description = "Malformed request body"),
        (status = 500, description = "Server error")
    ),
    security(
        ("api_key" = [])
    )
)]
async fn create_event(
    State(state): State<Arc<AppState>>,
    auth: RequireAuth,
    extract::Json(data): extract::Json<CreateEvent>,
) -> Result<StatusCode, AppError> {
    let facility = determine_facility(&auth, &data)?;
    if !auth.testing {
        let id = queries::create_event(&state.cobalt_db, &data, &facility).await?;
        tracing::info!(
            "key {} used to create event {}: '{}' for {}",
            auth.key_id,
            id,
            data.title,
            facility
        );
    } else {
        tracing::debug!("testing key {} used on create event endpoint", auth.key_id);
    }
    Ok(StatusCode::CREATED)
}

/// Update an existing event
///
/// An event can only be updated with API keys belonging to the facility that
/// created the event, or to keys belonging to VATUSA staff.
///
/// Facilities should omit the "facility" field in the payload; this field exists
/// so VATUSA staff can edit an event should they need to. The facility field
/// will resolve to the facility who owns the API key used in the request.

#[utoipa::path(
    patch,
    path = "/{id}",
    tag = "events",
    request_body = UpdateEvent,
    responses(
        (status = 200, description = "Event updated"),
        (status = 400, description = "Malformed request"),
        (status = 401, description = "Must be called with an API key"),
        (status = 403, description = "Cannot edit this event"),
        (status = 404, description = "No matching event found"),
        (status = 405, description = "Must be called with PATCH"),
        (status = 422, description = "Malformed request body"),
        (status = 500, description = "Server error")
    ),
    params(
        ("id" = i32, Path, description = "Event ID")
    ),
    security(
        ("api_key" = [])
    )
)]
async fn update_event(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
    auth: RequireAuth,
    extract::Json(data): extract::Json<UpdateEvent>,
) -> Result<StatusCode, AppError> {
    let event = queries::get_event(&state.cobalt_db, id).await?;
    let event = match event {
        Some(e) => e,
        None => {
            return Err(AppError::NotFound("event not found"));
        }
    };
    let facility = determine_facility(&auth, &data)?;
    if event.facility != facility && facility != "ZHQ" {
        tracing::info!(
            "key {} tried to update event for {}",
            auth.key_id,
            event.facility
        );
        return Err(AppError::InsufficientPermissions);
    }
    if !auth.testing {
        queries::update_event(&state.cobalt_db, id, &data, &facility).await?;
        tracing::info!("key {} used to update event {}", auth.key_id, event.id);
    } else {
        tracing::debug!(
            "testing key {} called on event update endpoint",
            auth.key_id
        );
    }
    Ok(StatusCode::OK)
}

/// Delete an existing event
///
/// An event can only be deleted with API keys belonging to the facility that
/// created the event, or to keys belonging to VATUSA staff.

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "events",
    responses(
        (status = 204, description = "Event deleted"),
        (status = 400, description = "Malformed request"),
        (status = 401, description = "Must be called with an API key"),
        (status = 403, description = "Cannot delete this event"),
        (status = 404, description = "No matching event found"),
        (status = 405, description = "Must be called with DELETE"),
        (status = 422, description = "Malformed request body"),
        (status = 500, description = "Server error")
    ),
    params(
        ("id" = i32, Path, description = "Event ID")
    ),
    security(
        ("api_key" = [])
    )
)]
async fn delete_event(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
    auth: RequireAuth,
) -> Result<StatusCode, AppError> {
    let event = queries::get_event(&state.cobalt_db, id).await?;
    let event = match event {
        Some(e) => e,
        None => {
            return Err(AppError::NotFound("event not found"));
        }
    };
    let facility = auth.facility.as_deref().unwrap_or_default();
    if event.facility != facility && facility != "ZHQ" {
        tracing::info!(
            "key {} tried to delete event for {}",
            auth.key_id,
            event.facility
        );
        return Err(AppError::InsufficientPermissions);
    }
    if !auth.testing {
        queries::delete_event(&state.cobalt_db, id).await?;
        tracing::info!(
            "key {} used to delete event {}: was '{}' for {}",
            auth.key_id,
            id,
            event.title,
            event.facility
        );
    } else {
        tracing::debug!(
            "testing key {} called on event delete endpoint",
            auth.key_id
        );
    }
    Ok(StatusCode::NO_CONTENT)
}
