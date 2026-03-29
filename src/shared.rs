//! Shared data and utilities.

use sqlx::MySqlPool;

/// Application state available to route functions.
pub struct AppState {
    /// Connection to the "old" VATUSA DB
    pub vatusa_db: MySqlPool,
    /// Connection to the new VATUSA backend DB
    pub cobalt_db: MySqlPool,
}
