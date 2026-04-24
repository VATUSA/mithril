//! Database operations.
//!
//! This module contains CRUD queries. All functions assume proper
//! authentication and authorization has already occurred.

#![allow(unused)]

use crate::{
    db::{ApiKey, Event, NewsPost, Role},
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEvent {
    pub title: String,
    pub body: String,
    pub banner_image_url: String,
    /// This field is for division overwrites; facilities should omit
    /// this in their request
    pub facility: Option<String>,
    pub start_time: i64,
    pub end_time: i64,
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
    pub title: String,
    pub body: String,
    pub banner_image_url: String,
    /// This field is for division overwrites; facilities should omit
    /// this in their request
    pub facility: Option<String>,
    pub start_time: i64,
    pub end_time: i64,
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
    facility: FacilityOverviewF,
    stats: FacilityOverviewStats,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FacilityOverviewF {
    info: FacilityBrief,
    roles: Vec<FacilityOverviewRole>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FacilityOverviewRole {
    id: i64,
    cid: i64,
    facility: String,
    role: String,
    created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FacilityOverviewStats {
    controllers: i32,
    pending_transfers: i32,
}

// I still hate MySQL
#[derive(Debug, ToSchema)]
struct FacilityOverviewPartial {
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
    let facility_info = sqlx::query_as!(
        FacilityOverviewPartial,
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

    todo!()
}
