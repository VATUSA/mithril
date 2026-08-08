//! Events routes.

use crate::{
    db::Event,
    middleware::{AuthExtractor, RequireAuth},
    queries::{self, CreateEvent, UpdateEvent},
    shared::{AppError, AppState, can_view_unapproved, determine_facility, viewer_facility},
};
use axum::{
    Json,
    extract::{self, Path, Query, State},
    response::{AppendHeaders, IntoResponse},
};
use chrono::NaiveDate;
use http::StatusCode;
use serde::Deserialize;
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};

/// Default number of events returned per page when `count` is omitted.
const DEFAULT_PAGE_COUNT: u32 = 25;
/// Maximum number of events that can be requested per page.
const MAX_PAGE_COUNT: u32 = 100;

/// Query parameters for paginating and filtering the events list.
#[derive(Debug, Deserialize)]
struct EventsQuery {
    /// 1-indexed page number. Defaults to 1.
    page: Option<u32>,
    /// Number of events per page. Defaults to 25, capped at 100.
    count: Option<u32>,
    /// Only include events starting on or after this UTC date (YYYY-MM-DD).
    start_date_from: Option<String>,
    /// Only include events starting on or before this UTC date (YYYY-MM-DD).
    start_date_to: Option<String>,
}

/// Parse a `YYYY-MM-DD` querystring date as a UTC calendar day and return the
/// unix-epoch timestamp of its midnight (`00:00:00 UTC`).
fn parse_utc_date_start(s: &str) -> Result<i64, AppError> {
    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|_| {
        AppError::BadRequest("start_date_from/start_date_to must be in YYYY-MM-DD format")
    })?;
    Ok(date
        .and_hms_opt(0, 0, 0)
        .expect("midnight is always a valid time")
        .and_utc()
        .timestamp())
}

/// Register routes.
pub fn router(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_events, create_event))
        .routes(routes!(get_single_event, update_event, delete_event))
        .with_state(state)
}

/// Retrieve events
///
/// Results are paginated via `page` (1-indexed, default 1) and `count`
/// (default 25, max 100), and can be narrowed to a range of start dates via
/// `start_date_from` and `start_date_to` (`YYYY-MM-DD`, inclusive, UTC
/// calendar days). Since VATUSA events are scheduled by the UTC day they
/// start on, even when they run past midnight UTC into the next calendar
/// day, filtering is based on `start_time` alone. When `page` or `count` is
/// present, the response includes `X-Total-Count` (total matching events)
/// and `X-Total-Pages` headers; the response body remains a plain array.
#[utoipa::path(
    get,
    path = "/",
    tag = "events",
    params(
        ("page" = Option<u32>, Query, description = "1-indexed page number, default 1"),
        ("count" = Option<u32>, Query, description = "Events per page, default 25, max 100"),
        ("start_date_from" = Option<String>, Query, description = "Only include events starting on/after this UTC date (YYYY-MM-DD)"),
        ("start_date_to" = Option<String>, Query, description = "Only include events starting on/before this UTC date (YYYY-MM-DD)")
    ),
    responses(
        (status = 200, description = "Posted events", body = [Event]),
        (status = 400, description = "Malformed start_date_from/start_date_to"),
        (status = 500, description = "Server error")
    )
)]
async fn get_events(
    auth: AuthExtractor,
    Query(filtering): Query<EventsQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let facility = viewer_facility(&auth.0);
    let count = filtering
        .count
        .unwrap_or(DEFAULT_PAGE_COUNT)
        .clamp(1, MAX_PAGE_COUNT);
    let page = filtering.page.unwrap_or(1).max(1);
    let offset = page.saturating_sub(1).saturating_mul(count);

    let start_from = filtering
        .start_date_from
        .as_deref()
        .map(parse_utc_date_start)
        .transpose()?;
    let start_to = filtering
        .start_date_to
        .as_deref()
        .map(parse_utc_date_start)
        .transpose()?
        .map(|ts| ts + 86400);
    if let (Some(from), Some(to)) = (start_from, start_to)
        && from >= to
    {
        return Err(AppError::BadRequest(
            "start_date_from must not be after start_date_to",
        ));
    }

    let events = queries::get_events(
        &state.cobalt_db,
        facility,
        start_from,
        start_to,
        count,
        offset,
    )
    .await?;
    let total = queries::count_events(&state.cobalt_db, facility, start_from, start_to).await?;
    let total = u32::try_from(total).unwrap_or(u32::MAX);
    let total_pages = total.div_ceil(count).max(1);

    Ok((
        AppendHeaders([
            ("X-Total-Count", total.to_string()),
            ("X-Total-Pages", total_pages.to_string()),
        ]),
        Json(events),
    ))
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
    auth: AuthExtractor,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Event>, AppError> {
    let event = queries::get_event(&state.cobalt_db, id).await?;
    let Some(event) = event else {
        return Err(AppError::NotFound("event not found"));
    };
    if event.review_status.as_deref().unwrap_or_default() != "approved"
        && !can_view_unapproved(&auth.0, &event.facility)
    {
        return Err(AppError::NotFound("event not found"));
    }
    Ok(Json(event))
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
    let Some(event) = event else {
        return Err(AppError::NotFound("event not found"));
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
    let Some(event) = event else {
        return Err(AppError::NotFound("event not found"));
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
