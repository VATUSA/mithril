//! Database operations.
//!
//! This module contains CRUD queries.

use crate::{
    db::{Event, NewsPost},
    shared::AppError,
};
use sqlx::MySqlPool;

/// Get all `news_post` rows.
pub async fn get_news(db: &MySqlPool) -> Result<Vec<NewsPost>, AppError> {
    let news = sqlx::query_as!(NewsPost, "SELECT * FROM news_post")
        .fetch_all(db)
        .await?;
    Ok(news)
}

/// Get all `event` rows.
pub async fn get_events(db: &MySqlPool) -> Result<Vec<Event>, AppError> {
    let events = sqlx::query_as!(Event, "SELECT * FROM event")
        .fetch_all(db)
        .await?;
    Ok(events)
}
