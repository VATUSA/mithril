//! Database operations.
//!
//! This module contains CRUD queries. All functions assume proper
//! authentication and authorization has already occurred.

#![allow(unused)]

use crate::{
    db::{ApiKey, Event, NewsPost},
    shared::AppError,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::MySqlPool;
use utoipa::ToSchema;

// ---------------------------------------------------
// v3_api_key
// ---------------------------------------------------

/// Get an API key from the DB by its code.
pub async fn get_api_key(db: &MySqlPool, code: &str) -> Result<Option<ApiKey>, AppError> {
    // I hate MySQL
    let api_key = sqlx::query_as!(
        ApiKey,
        r#"SELECT id, code, testing as "testing: bool", facility, notes,
created_at, updated_at FROM v3_api_key WHERE code = ?"#,
        code
    )
    .fetch_optional(db)
    .await?;
    Ok(api_key)
}

// ---------------------------------------------------
// news_post
// ---------------------------------------------------

/// Get all `news_post` rows.
pub async fn get_news_posts(db: &MySqlPool) -> Result<Vec<NewsPost>, AppError> {
    let news = sqlx::query_as!(NewsPost, "SELECT * FROM news_post")
        .fetch_all(db)
        .await?;
    Ok(news)
}

/// Get a single `news_post` row.
pub async fn get_news_post(db: &MySqlPool, id: i32) -> Result<Option<NewsPost>, AppError> {
    let news = sqlx::query_as!(NewsPost, "SELECT * FROM news_post WHERE id = ? LIMIT 1", id)
        .fetch_optional(db)
        .await?;
    Ok(news)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateNewsPost {
    pub title: String,
    pub body: String,
    pub author_cid: i32,
}

/// Create a new `news_post` row.
pub async fn create_news_post(db: &MySqlPool, data: &CreateNewsPost) -> Result<u64, AppError> {
    let id = sqlx::query!(
        "INSERT INTO news_post (title, body, author_cid, post_time) VALUES (?, ?, ?, ?)",
        data.title,
        data.body,
        data.author_cid,
        Utc::now().timestamp_millis()
    )
    .execute(db)
    .await?
    .last_insert_id();
    Ok(id)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateNewsPost {
    pub title: String,
    pub body: String,
}

/// Update a `news_post` row.
pub async fn update_news_post(
    db: &MySqlPool,
    id: i32,
    data: &UpdateNewsPost,
) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE news_post SET title=?, body=?, edit_time=? WHERE id = ?",
        data.title,
        data.body,
        Utc::now().timestamp_millis(),
        id,
    )
    .execute(db)
    .await?;
    Ok(())
}

/// Delete a `news_post` row.
pub async fn delete_news_post(db: &MySqlPool, id: i32) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM news_post WHERE id = ?", id)
        .execute(db)
        .await?;
    Ok(())
}

// ---------------------------------------------------
// event
// ---------------------------------------------------

/// Get all `event` rows.
pub async fn get_events(db: &MySqlPool) -> Result<Vec<Event>, AppError> {
    let events = sqlx::query_as!(Event, "SELECT * FROM event")
        .fetch_all(db)
        .await?;
    Ok(events)
}

/// Get a single `event` row.
pub async fn get_event(db: &MySqlPool, id: i32) -> Result<Option<Event>, AppError> {
    let events = sqlx::query_as!(Event, "SELECT * FROM event WHERE id = ? LIMIT 1", id)
        .fetch_optional(db)
        .await?;
    Ok(events)
}

#[derive(Debug, Deserialize)]
pub struct CreateEvent {
    pub title: String,
    pub body: String,
    pub banner_image_url: String,
    pub facility: String,
    pub start_time: i64,
    pub end_time: i64,
    pub created_by: i32,
}

/// Create a new `event` row.
pub async fn create_event(db: &MySqlPool, data: &CreateEvent) -> Result<u64, AppError> {
    let id = sqlx::query!(
        r#"INSERT INTO event
        (title, body, banner_image_url, facility, start_time, end_time, created_at, created_by)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        data.title,
        data.body,
        data.banner_image_url,
        data.facility,
        data.start_time,
        data.end_time,
        Utc::now().timestamp_millis(),
        data.created_by
    )
    .execute(db)
    .await?
    .last_insert_id();
    Ok(id)
}

#[derive(Debug, Deserialize)]
pub struct UpdateEvent {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub banner_image_url: String,
    pub facility: String,
    pub start_time: i64,
    pub end_time: i64,
    pub created_at: i64,
    pub created_by: i32,
    pub updated_by: i32,
}

/// Update an `event` row.
pub async fn update_event(db: &MySqlPool, data: &UpdateEvent) -> Result<(), AppError> {
    sqlx::query!(
        r#"UPDATE event SET title=?, body=?, banner_image_url=?, facility=?,
        start_time=?, end_time=?, updated_at=?, updated_by=? WHERE id = ?"#,
        data.title,
        data.body,
        data.banner_image_url,
        data.facility,
        data.start_time,
        data.end_time,
        Utc::now().timestamp_millis(),
        data.updated_by,
        data.id,
    )
    .execute(db)
    .await?;
    Ok(())
}

/// Delete an `event` row.
pub async fn delete_event(db: &MySqlPool, id: u64) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM event WHERE id = ?", id)
        .execute(db)
        .await?;
    Ok(())
}

// ---------------------------------------------------
// ?
// ---------------------------------------------------
