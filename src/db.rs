//! Database operations.
//!
//! Unlike many applications, this program is connecting to a database
//! that already exists and is already populated with data. Because of
//! that, you won't find things here like migrations or CREATE TABLE
//! statements.

use crate::shared::AppError;
use serde::Serialize;
use sqlx::MySqlPool;
use std::env;

/// Get a connection pool to the `vatusa-old` database.
///
/// Reads from the `DATABASE_URL_VATUSA` environment variable.
pub async fn connect_vatusa() -> Result<MySqlPool, AppError> {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL_VATUSA")?).await?;
    Ok(pool)
}

/// Get a connection pool to the `cobalt` database.
///
/// Reads from the `DATABASE_URL_COBALT` environment variable.
pub async fn connect_cobalt() -> Result<MySqlPool, AppError> {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL_COBALT")?).await?;
    Ok(pool)
}

#[derive(Debug, Serialize)]
pub struct NewsPost {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub author_cid: i32,
    pub post_time: i64,
    pub edit_time: i64,
}

/// Get all `news_post` rows.
pub async fn get_news(db: &MySqlPool) -> Result<Vec<NewsPost>, AppError> {
    let news = sqlx::query_as!(NewsPost, "SELECT * FROM news_post")
        .fetch_all(db)
        .await?;
    Ok(news)
}

#[derive(Debug, Serialize)]
pub struct Event {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub banner_image_url: String,
    pub facility: String,
    pub start_time: i64,
    pub end_time: i64,
    pub created_at: i64,
    pub created_by: i32,
    pub updated_at: i64,
    pub updated_by: i32,
}

/// Get all `event` rows.
pub async fn get_events(db: &MySqlPool) -> Result<Vec<Event>, AppError> {
    let events = sqlx::query_as!(Event, "SELECT * FROM event")
        .fetch_all(db)
        .await?;
    Ok(events)
}
