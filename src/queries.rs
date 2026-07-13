//! Database operations.
//!
//! This module contains CRUD queries. All functions assume proper
//! authentication and authorization has already occurred.

#![allow(unused)]

use crate::{
    db::{ApiKey, ChangeLogEntry, Event, NewsPost, Role, Webhook},
    shared::{AppError, HasFacility},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use utoipa::ToSchema;

// ---------------------------------------------------
// v3_api_key
// ---------------------------------------------------

/// Get an API key from the DB by its code.
///
/// This function will not return a key that has the `deleted_at` field set
/// to anything other than NULL. This allows for soft-deletes of API keys
/// so as to not disturb previous logs.
pub async fn get_api_key(db: &MySqlPool, code: &str) -> Result<Option<ApiKey>, AppError> {
    // I hate MySQL
    let api_key = sqlx::query_as!(
        ApiKey,
        r#"SELECT id, code, testing as "testing: bool", facility, notes,
        created_at, updated_at, deleted_at FROM v3_api_key WHERE code = ?
        AND deleted_at IS NULL"#,
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
    /// News posting title.
    pub title: String,
    /// News posting body content.
    pub body: String,
    /// News posting author.
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
    /// New title.
    pub title: String,
    /// New body content.
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEvent {
    /// Event title
    pub title: String,
    /// Event body content
    pub body: String,
    /// Event banner image
    pub banner_image_url: String,
    /// This field is for division overwrites; facilities should omit
    /// this in their request.
    pub facility: Option<String>,
    /// Event start time, UTC
    pub start_time: i64,
    /// Event end time, UTC
    pub end_time: i64,
    /// Event author, CIC, leader, etc.
    pub created_by: i32,
}

impl HasFacility for &CreateEvent {
    fn facility(&self) -> Option<String> {
        self.facility.clone()
    }
}

/// Create a new `event` row.
pub async fn create_event(
    db: &MySqlPool,
    data: &CreateEvent,
    facility: &str,
) -> Result<u64, AppError> {
    let id = sqlx::query!(
        r#"INSERT INTO event
        (title, body, banner_image_url, facility, start_time, end_time, created_at, created_by, updated_by, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        data.title,
        data.body,
        data.banner_image_url,
        facility,
        data.start_time,
        data.end_time,
        Utc::now().timestamp_millis(),
        data.created_by,
        0,
        0
    )
    .execute(db)
    .await?
    .last_insert_id();
    Ok(id)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateEvent {
    /// Event title
    pub title: String,
    /// Event body content
    pub body: String,
    /// Event banner image
    pub banner_image_url: String,
    /// This field is for division overwrites; facilities should omit
    /// this in their request.
    pub facility: Option<String>,
    /// Event start time, UTC
    pub start_time: i64,
    /// Event end time, UTC
    pub end_time: i64,
    /// Who owns this change
    pub updated_by: i32,
}

impl HasFacility for &UpdateEvent {
    fn facility(&self) -> Option<String> {
        self.facility.clone()
    }
}

/// Update an `event` row.
pub async fn update_event(
    db: &MySqlPool,
    id: i32,
    data: &UpdateEvent,
    facility: &str,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"UPDATE event SET title=?, body=?, banner_image_url=?, facility=?,
        start_time=?, end_time=?, updated_at=?, updated_by=? WHERE id = ?"#,
        data.title,
        data.body,
        data.banner_image_url,
        facility,
        data.start_time,
        data.end_time,
        Utc::now().timestamp_millis(),
        data.updated_by,
        id,
    )
    .execute(db)
    .await?;
    Ok(())
}

/// Delete an `event` row.
pub async fn delete_event(db: &MySqlPool, id: i32) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM event WHERE id = ?", id)
        .execute(db)
        .await?;
    Ok(())
}

// ---------------------------------------------------
// facilities
// ---------------------------------------------------

/// Generic information available for each facility.
#[derive(Debug, Serialize, ToSchema)]
pub struct FacilityBrief {
    pub id: String,
    pub name: String,
    pub url: String,
    pub region: i32,
    pub atm: u32,
    pub datm: u32,
    pub ta: u32,
    pub ec: u32,
    pub fe: u32,
    pub wm: u32,
}

/// Get active facility information for the division.
pub async fn get_active_facilities(db: &MySqlPool) -> Result<Vec<FacilityBrief>, AppError> {
    let data = sqlx::query_as!(
        FacilityBrief,
        "SELECT id, name, url, region, atm, datm, ta, ec, fe, wm FROM facilities WHERE active = 1"
    )
    .fetch_all(db)
    .await?;
    Ok(data)
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FacilityOverview {
    info: FacilityOverviewInfo,
    roles: Vec<Role>,
}

// I still hate MySQL
#[derive(Debug, Serialize, ToSchema)]
pub struct FacilityOverviewInfo {
    id: Option<String>,
    name: Option<String>,
    url: Option<String>,
    hosted_email_domain: Option<String>,
    region: Option<i32>,
    atm: Option<u32>,
    datm: Option<u32>,
    ta: Option<u32>,
    ec: Option<u32>,
    fe: Option<u32>,
    wm: Option<u32>,
    ace: Option<i32>,
    active: bool,
    controllers: Option<i64>,
    pending_transfers: Option<i64>,
}

/// Get the overview data for the facility.
///
/// More comprehensive than what's returned by the function & endpoint to
/// get all of the division's facilities.
pub async fn get_facility_full_info(
    db: &MySqlPool,
    id: &str,
) -> Result<FacilityOverview, AppError> {
    let info = sqlx::query_as!(
        FacilityOverviewInfo,
        r#"
SELECT
    f.id, f.name, f.url, f.hosted_email_domain, f.region, f.atm, f.datm, f.ta, f.ec, f.fe, f.wm, f.active as "active!: bool", f.ace,
    (SELECT COUNT(*) FROM controllers c WHERE c.facility = f.id) AS controllers,
    (SELECT COUNT(*) FROM transfers t WHERE t.to = f.id AND t.status = 0) AS pending_transfers
FROM facilities f
WHERE f.id = ? AND f.active = 1;"#,
        id
    ).fetch_one(db).await?;
    let roles = sqlx::query_as!(Role, r#"SELECT * FROM roles WHERE facility = ?"#, id)
        .fetch_all(db)
        .await?;
    Ok(FacilityOverview { info, roles })
}

// ---------------------------------------------------
// v3_webhook
// ---------------------------------------------------

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateWebhook {
    /// The URL at which you will receive HTTP requests with data. Must be HTTPS.
    pub url: String,
}

impl HasFacility for &CreateWebhook {
    fn facility(&self) -> Option<String> {
        None
    }
}

/// Create a new `v3_webhook` row. Returns the new row's id.
///
/// `db` must be the Cobalt DB pool.
pub async fn create_webhook(
    db: &MySqlPool,
    facility: &str,
    url: &str,
    secret: &str,
) -> Result<u64, AppError> {
    let id = sqlx::query!(
        "INSERT INTO v3_webhook (facility, url, secret) VALUES (?, ?, ?)",
        facility,
        url,
        secret,
    )
    .execute(db)
    .await?
    .last_insert_id();
    Ok(id)
}

/// Get all non-deleted `v3_webhook` rows for a facility.
///
/// `db` must be the Cobalt DB pool.
pub async fn get_webhooks_for_facility(
    db: &MySqlPool,
    facility: &str,
) -> Result<Vec<Webhook>, AppError> {
    let rows = sqlx::query_as!(
        Webhook,
        r#"SELECT id, facility, url, secret, notes, created_at, updated_at, deleted_at
        FROM v3_webhook WHERE facility = ? AND deleted_at IS NULL"#,
        facility
    )
    .fetch_all(db)
    .await?;
    Ok(rows)
}

// ---------------------------------------------------
// roster_notifications
// ---------------------------------------------------

/// Get unprocessed `roster_notifications` rows, oldest first.
pub async fn get_unprocessed_changes(
    db: &MySqlPool,
    limit: i64,
) -> Result<Vec<ChangeLogEntry>, AppError> {
    let rows = sqlx::query_as!(
        ChangeLogEntry,
        r#"SELECT id, table_name, row_pk, operation,
        old_value as `old_value: serde_json::Value`,
        new_value as `new_value: serde_json::Value`,
        created_at, processed_at
        FROM roster_notifications WHERE processed_at IS NULL
        ORDER BY id ASC LIMIT ?"#,
        limit
    )
    .fetch_all(db)
    .await?;
    Ok(rows)
}

/// Mark a single `roster_notifications` row as processed (sets `processed_at = NOW()`).
pub async fn mark_change_processed(db: &MySqlPool, id: u64) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE roster_notifications SET processed_at = NOW() WHERE id = ?",
        id
    )
    .execute(db)
    .await?;
    Ok(())
}

/// Delete processed `roster_notifications` rows older than `retention_days`.
///
/// Returns the number of rows deleted.
pub async fn delete_processed_changes(
    db: &MySqlPool,
    retention_days: u32,
) -> Result<u64, AppError> {
    let result = sqlx::query!(
        r#"DELETE FROM roster_notifications WHERE processed_at IS NOT NULL
        AND processed_at < NOW() - INTERVAL ? DAY"#,
        retention_days
    )
    .execute(db)
    .await?;
    Ok(result.rows_affected())
}
