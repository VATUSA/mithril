//! Facilities routes.

#![allow(dead_code)]

use crate::{
    queries::{self, FacilityBrief, FacilityOverview},
    shared::{AppError, AppState},
};
use axum::{
    Json,
    extract::{Path, State},
};
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};

/// Register routes.
pub fn router(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_facilities))
        .routes(routes!(get_facility_info))
        .with_state(state)
}

/// Get the division's list of active facilities.
#[utoipa::path(
    get,
    path = "/",
    tag = "facility",
    responses(
        (status = 200, description = "Facilities", body = [FacilityBrief])
    )
)]
async fn get_facilities(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<FacilityBrief>>, AppError> {
    let facilities = queries::get_active_facilities(&state.vatusa_db).await?;
    Ok(Json(facilities))
}

/// Full facility information.
#[utoipa::path(
    get,
    path = "/{id}",
    tag = "facility",
    responses(
        (status = 200, description = "Facility data", body = FacilityOverview),
        (status = 404, description = "Facility not found")
    )
)]
async fn get_facility_info(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<FacilityOverview>, AppError> {
    todo!()
}

async fn get_roster() {
    todo!()
}

async fn add_to_visiting_roster() {
    todo!()
}

async fn delete_from_visiting_roster() {
    todo!()
}

async fn delete_from_home_roster() {
    todo!()
}

async fn get_pending_transfers() {
    todo!()
}

async fn update_pending_transfer() {
    todo!()
}

async fn get_training_records() {
    todo!()
}

async fn get_role_members() {
    todo!()
}
