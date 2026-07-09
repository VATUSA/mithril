//! Database setup & models.
//!
//! Unlike many applications, this program is connecting to a database
//! that already exists and is already populated with data. Because of
//! that, you won't find things here like migrations or `CREATE TABLE`
//! statements. Incremental migrations will be driven by Cobalt, then
//! used in the frontend and in this API.
//!
//! This module contains the DB connection functions and DB models.

#![allow(unused)]

use crate::shared::AppError;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, MySqlPool};
use std::env;
use utoipa::ToSchema;

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

/// The v3 API keys for this application.
///
/// Net-new to the database.
///
/// **Table**: `v3_api_key`
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiKey {
    pub id: i64,
    pub code: String,
    pub testing: bool,
    pub facility: Option<String>,
    pub notes: Option<String>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub deleted_at: Option<i64>,
}

/// News posts, Cobalt DB.
///
/// **Table**: `news_post`
#[derive(Debug, Serialize, ToSchema)]
pub struct NewsPost {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub author_cid: i32,
    pub post_time: i64,
    pub edit_time: i64,
}

/// Event posting, Cobalt DB.
///
/// **Table**: `event`
#[derive(Debug, Serialize, ToSchema)]
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
    pub review_status: Option<String>,
    pub reviewed_by: Option<i32>,
    pub reviewed_on: Option<i64>,
}

/// **Table**: `academy_exam_assignments`
#[derive(Debug, Serialize, ToSchema)]
pub struct AcademyExamAssignment {
    pub id: u32,
    pub student_id: i32,
    pub instructor_id: i32,
    pub moodle_uid: i32,
    pub course_id: i32,
    pub course_name: String,
    pub quiz_id: i32,
    pub rating_id: i32,
    pub attempt_emails_sent: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// **Table**: `action_log`
#[derive(Debug, Serialize, ToSchema)]
pub struct ActionLog {
    pub id: i32,
    pub r#from: i32,
    pub r#to: i32,
    pub log: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// **Table**: `api_log`
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiLog {
    pub id: u64,
    pub facility: String,
    pub datetime: NaiveDateTime,
    pub method: String,
    pub url: String,
    pub data: String,
}

/// **Table**: `controller_eligibility_cache`
#[derive(Debug, Serialize, ToSchema)]
pub struct ControllerEligibilityCache {
    pub cid: i32,
    pub competency_rating: Option<i32>,
    pub competency_date: Option<NaiveDate>,
    pub is_initial_selection: Option<bool>,
    pub first_selection_date: Option<NaiveDate>,
    pub has_consolidation_hours: Option<bool>,
    pub consolidation_hours: Option<f32>,
    pub last_promotion_date: Option<NaiveDate>,
    pub last_transfer_date: Option<NaiveDate>,
    pub last_visit_date: Option<NaiveDate>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// **Table**: `controllers`
#[allow(non_snake_case)]
#[derive(Debug, Serialize, ToSchema)]
pub struct Controller {
    pub cid: u32,
    pub fname: String,
    pub lname: String,
    pub email: String,
    pub facility: String,
    pub rating: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
    pub flag_needbasic: i32,
    pub flag_xferOverride: i32,
    pub facility_join: NaiveDateTime,
    pub flag_homecontroller: i32,
    pub remember_token: Option<String>,
    pub cert_update: i32,
    pub lastactivity: NaiveDateTime,
    pub flag_broadcastOptedIn: bool,
    pub flag_preventStaffAssign: bool,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires: Option<u64>,
    pub discord_id: Option<String>,
    pub prefname: bool,
    pub prefname_date: Option<NaiveDateTime>,
    pub last_cert_sync: Option<NaiveDateTime>,
    pub flag_nameprivacy: bool,
    pub last_competency_date: Option<NaiveDate>,
}

/// **Table**: `email_accounts`
#[derive(Debug, Serialize, ToSchema)]
pub struct EmailAccount {
    pub id: u32,
    pub facility: String,
    pub username: String,
    pub cid: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// **Table**: `email_config`
#[derive(Debug, Serialize, ToSchema)]
pub struct EmailConfig {
    pub address: String,
    pub config: Option<String>,
    pub destination: Option<String>,
    pub modified_by: Option<i32>,
    pub updated_at: Option<String>,
}

/// **Table**: `email_templates`
#[derive(Debug, Serialize, ToSchema)]
pub struct EmailTemplate {
    pub id: u32,
    pub facility_id: String,
    pub template: String,
    pub body: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// **Table**: `facilities`
#[derive(Debug, Serialize, ToSchema)]
pub struct Facility {
    pub id: String,
    pub name: String,
    pub url: String,
    pub hosted_email_domain: Option<String>,
    pub region: i32,
    pub atm: u32,
    pub datm: u32,
    pub ta: u32,
    pub ec: u32,
    pub fe: u32,
    pub wm: u32,
    pub uls_return: String,
    pub uls_devreturn: String,
    pub uls_secret: String,
    pub uls_jwk: Option<String>,
    pub active: i32,
    pub apikey: String,
    pub ip: String,
    pub api_sandbox_key: String,
    pub api_sandbox_ip: String,
    pub apiv2_jwk: Option<String>,
    pub welcome_text: String,
    pub ace: i32,
    pub apiv2_jwk_dev: Option<String>,
    pub uls_jwk_dev: String,
    pub url_dev: String,
}

/// **Table**: `knowledgebase_categories`
#[derive(Debug, Serialize, ToSchema)]
pub struct KnowledgebaseCategory {
    pub id: u32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// **Table**: `knowledgebase_questions`
#[derive(Debug, Serialize, ToSchema)]
pub struct KnowledgebaseQuestion {
    pub id: u32,
    pub category_id: u32,
    pub r#order: u32,
    pub question: String,
    pub answer: String,
    pub updated_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// **Table**: `ots_evals`
#[derive(Debug, Serialize, ToSchema)]
pub struct OtsEval {
    pub id: u32,
    pub training_record_id: Option<i32>,
    pub student_id: i32,
    pub instructor_id: i32,
    pub facility_id: String,
    pub exam_position: String,
    pub form_id: u32,
    pub notes: Option<String>,
    pub exam_date: NaiveDate,
    pub result: bool,
    pub signature: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// **Table**: `ots_evals_forms`
#[derive(Debug, Serialize, ToSchema)]
pub struct OtsEvalsForm {
    pub id: u32,
    pub name: String,
    pub rating_id: i32,
    pub position: String,
    pub instructor_notes: Option<String>,
    pub is_statement: bool,
    pub description: String,
    pub active: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// **Table**: `ots_evals_indicator_results`
#[derive(Debug, Serialize, ToSchema)]
pub struct OtsEvalsIndicatorResult {
    pub id: u32,
    pub perf_indicator_id: u32,
    pub eval_id: u32,
    pub result: i16,
    pub comment: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// **Table**: `ots_evals_perf_cats`
#[derive(Debug, Serialize, ToSchema)]
pub struct OtsEvalsPerfCat {
    pub id: u32,
    pub label: String,
    pub form_id: u32,
    pub r#order: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// **Table**: `ots_evals_perf_indicators`
#[derive(Debug, Serialize, ToSchema)]
pub struct OtsEvalsPerfIndicator {
    pub id: u32,
    pub perf_cat_id: u32,
    pub label: String,
    pub help_text: Option<String>,
    pub header_type: i16,
    pub is_commendable: Option<bool>,
    pub is_required: Option<bool>,
    pub can_unsat: Option<bool>,
    pub r#order: i32,
    pub extra_options: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

/// **Table**: `promotions`
#[derive(Debug, Serialize, ToSchema)]
pub struct Promotion {
    pub id: i32,
    pub cid: i32,
    pub grantor: u32,
    pub r#to: i32,
    pub r#from: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub exam: NaiveDate,
    pub examiner: u32,
    pub position: String,
    pub eval_id: Option<u32>,
}

/// **Table**: `ratings`
#[derive(Debug, Serialize, ToSchema)]
pub struct Rating {
    pub id: i32,
    pub short: String,
    pub long: String,
}

/// **Table**: `roles`
#[derive(Debug, Serialize, ToSchema)]
pub struct Role {
    pub id: u64,
    pub cid: u32,
    pub facility: String,
    pub role: String,
    pub created_at: chrono::DateTime<Utc>,
}

/// **Table**: `solo_certs`
#[derive(Debug, Serialize, ToSchema)]
pub struct SoloCert {
    pub id: u32,
    pub cid: u32,
    pub position: String,
    pub expires: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// **Table**: `survey_assignments`
#[derive(Debug, Serialize, ToSchema)]
pub struct SurveyAssignment {
    pub id: String,
    pub survey_id: i32,
    pub facility: String,
    pub rating: i32,
    pub misc_data: String,
    pub completed: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// **Table**: `survey_questions`
#[derive(Debug, Serialize, ToSchema)]
pub struct SurveyQuestion {
    pub id: u32,
    pub survey_id: i32,
    pub question: String,
    pub data: String,
    pub r#order: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// **Table**: `survey_submissions`
#[derive(Debug, Serialize, ToSchema)]
pub struct SurveySubmission {
    pub id: u32,
    pub survey_id: i32,
    pub question_id: i32,
    pub response: String,
    pub facility: String,
    pub rating: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// **Table**: `surveys`
#[derive(Debug, Serialize, ToSchema)]
pub struct Survey {
    pub id: u32,
    pub facility: String,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// **Table**: `tmu_facilities`
#[derive(Debug, Serialize, ToSchema)]
pub struct TmuFacility {
    pub id: String,
    pub parent: Option<String>,
    pub name: String,
    pub coords: String,
}

/// **Table**: `tmu_notices`
#[derive(Debug, Serialize, ToSchema)]
pub struct TmuNotice {
    pub id: u32,
    pub tmu_facility_id: String,
    pub priority: i16,
    pub message: String,
    pub start_date: NaiveDateTime,
    pub expire_date: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_delay: bool,
    pub is_pref_route: bool,
}

/// **Table**: `training_records`
#[derive(Debug, Serialize, ToSchema)]
pub struct TrainingRecord {
    pub id: u32,
    pub student_id: i32,
    pub instructor_id: i32,
    pub session_date: NaiveDateTime,
    pub facility_id: String,
    pub position: String,
    pub duration: NaiveTime,
    pub movements: Option<i32>,
    pub score: Option<i32>,
    pub notes: String,
    pub location: i16,
    pub ots_status: i16,
    pub ots_eval_id: Option<i32>,
    pub is_cbt: bool,
    pub solo_granted: bool,
    pub modified_by: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// **Table**: `transfers`
#[derive(Debug, Serialize, ToSchema)]
pub struct Transfer {
    pub id: u32,
    pub cid: u32,
    pub r#to: String,
    pub r#from: String,
    pub reason: String,
    pub status: i32,
    pub actiontext: String,
    pub actionby: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// **Table**: `visits`
#[derive(Debug, Serialize, ToSchema)]
pub struct Visit {
    pub id: u64,
    pub cid: u32,
    pub facility: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// A single logged change to `controllers` or `visits`, written by DB
/// triggers. See `sql/001_roster_notifications.sql`.
///
/// **Table**: `roster_notifications`
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct ChangeLogEntry {
    pub id: u64,
    pub table_name: String,
    pub row_pk: u64,
    pub operation: String, // 'INSERT' | 'UPDATE' | 'DELETE'
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<Utc>,
    pub processed_at: Option<chrono::DateTime<Utc>>,
}
