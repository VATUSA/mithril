//! Division routes.

use crate::{
    db::UserRole,
    middleware::RequireAuth,
    queries::{self, DivisionController},
    shared::{AppError, AppState},
};
use axum::{Json, extract::State};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

/// Register routes.
pub fn router(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_controllers_and_roles))
        .with_state(state)
}

#[derive(Debug, Serialize, ToSchema)]
struct ControllersAndRoles {
    controllers: Vec<DivisionController>,
    roles: Vec<UserRole>,
}

/// Get all of the division's controllers and role grants.
#[utoipa::path(
    get,
    path = "/controllers",
    tag = "division",
    responses(
        (status = 200, description = "Controllers and roles", body = ControllersAndRoles),
        (status = 401, description = "Must be called with an API key"),
        (status = 500, description = "Server error")
    ),
    security(
        ("api_key" = [])
    )
)]
async fn get_controllers_and_roles(
    State(state): State<Arc<AppState>>,
    _auth: RequireAuth,
) -> Result<Json<ControllersAndRoles>, AppError> {
    let controllers = queries::get_division_controllers(&state.cobalt_db).await?;
    let roles = queries::get_user_roles(&state.cobalt_db).await?;
    Ok(Json(ControllersAndRoles { controllers, roles }))
}
