//! Database setup & models.
//!
//! Unlike many applications, this program is connecting to a database
//! that already exists and is already populated with data. Because of
//! that, you won't find things here like migrations or CREATE TABLE
//! statements.
//!
//! This module contains the DB connection functions and DB models.

#![allow(unused)]

use crate::shared::AppError;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
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

#[derive(Debug, Serialize, ToSchema)]
pub struct NewsPost {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub author_cid: i32,
    pub post_time: i64,
    pub edit_time: i64,
}

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
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AcademyBasicExamEmails {
    pub id: u32,
    pub attempt_id: i32,
    pub student_id: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AcademyCompetency {
    pub id: i32,
    pub cid: Option<i32>,
    pub academy_course_id: Option<i32>,
    pub completion_timestamp: Option<NaiveDateTime>,
    pub expiration_timestamp: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub rating: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AcademyCourse {
    pub id: i32,
    pub name: String,
    pub list_order: Option<i32>,
    pub moodle_enrol_id: Option<i32>,
    pub moodle_quiz_id: Option<i32>,
    pub passing_percent: i32,
    pub rating: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AcademyCourseEnrollment {
    pub id: i32,
    pub cid: i32,
    pub academy_course_id: i32,
    pub assignment_timestamp: Option<NaiveDateTime>,
    pub passed_timestamp: Option<NaiveDateTime>,
    pub status: i32,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AcademyExamAssignments {
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

#[derive(Debug, Serialize, ToSchema)]
pub struct ActionLog {
    pub id: i32,
    pub r#from: i32,
    pub r#to: i32,
    pub log: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiLog {
    pub id: u64,
    pub facility: String,
    pub datetime: NaiveDateTime,
    pub method: String,
    pub url: String,
    pub data: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChecklistData {
    pub id: u32,
    pub checklist_id: u32,
    pub item: String,
    pub r#order: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Checklists {
    pub id: u32,
    pub name: String,
    pub active: i32,
    pub r#order: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

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

#[derive(Debug, Serialize, ToSchema)]
pub struct ControllerTraining {
    pub id: u32,
    pub student_cid: u32,
    pub instructor_cid: u32,
    pub facility: String,
    pub position: String,
    pub r#type: String,
    pub checklist_name: String,
    pub checklist_data: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, ToSchema)]
pub struct Controllers {
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

#[derive(Debug, Serialize, ToSchema)]
pub struct EmailAccounts {
    pub id: u32,
    pub facility: String,
    pub username: String,
    pub cid: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EmailConfig {
    pub address: String,
    pub config: Option<String>,
    pub destination: Option<String>,
    pub modified_by: Option<i32>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EmailOutbound {
    pub id: i32,
    pub from_email: Option<String>,
    pub from_name: Option<String>,
    pub reply_to_email: Option<String>,
    pub to_emails: Option<String>,
    pub bcc_emails: Option<String>,
    pub subject: String,
    pub body: String,
    pub lock_key: Option<String>,
    pub processed: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EmailTemplates {
    pub id: u32,
    pub facility_id: String,
    pub template: String,
    pub body: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExamAssignments {
    pub id: u64,
    pub cid: u32,
    pub exam_id: u32,
    pub instructor_id: u32,
    pub assigned_date: NaiveDateTime,
    pub expire_date: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExamGenerated {
    pub id: u32,
    pub cid: u32,
    pub exam_id: u32,
    pub question_id: u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExamQuestions {
    pub id: u32,
    pub exam_id: u32,
    pub question: String,
    pub r#type: i32,
    pub answer: String,
    pub alt1: String,
    pub alt2: String,
    pub alt3: String,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExamReassignments {
    pub id: u64,
    pub cid: u32,
    pub exam_id: u32,
    pub reassign_date: NaiveDateTime,
    pub instructor_id: u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExamResults {
    pub id: u64,
    pub exam_id: u64,
    pub exam_name: String,
    pub cid: i32,
    pub score: i32,
    pub passed: i32,
    pub date: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExamResultsData {
    pub id: u32,
    pub result_id: u64,
    pub question: String,
    pub correct: String,
    pub selected: Option<String>,
    pub notes: String,
    pub is_correct: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Exams {
    pub id: u32,
    pub facility_id: String,
    pub name: String,
    pub number: i32,
    pub is_active: i32,
    pub cbt_required: Option<u64>,
    pub retake_period: i32,
    pub passing_score: i32,
    pub answer_visibility: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Facilities {
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

#[derive(Debug, Serialize, ToSchema)]
pub struct FacilityTrends {
    pub id: u64,
    pub date: NaiveDate,
    pub facility: String,
    pub obs: i32,
    pub obsg30: i32,
    pub s1: i32,
    pub s2: i32,
    pub s3: i32,
    pub c1: i32,
    pub c3: i32,
    pub i1: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FailedJobs {
    pub id: u64,
    pub uuid: String,
    pub connection: String,
    pub queue: String,
    pub payload: String,
    pub exception: String,
    pub failed_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Flights {
    pub callsign: String,
    pub lat: String,
    pub long: String,
    pub hdg: i32,
    pub dest: String,
    pub dep: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Jobs {
    pub id: u32,
    pub r#type: String,
    pub data: String,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct KnowledgebaseCategories {
    pub id: u32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct KnowledgebaseQuestions {
    pub id: u32,
    pub category_id: u32,
    pub r#order: u32,
    pub question: String,
    pub answer: String,
    pub updated_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginTokens {
    pub token: String,
    pub cid: u32,
    pub timestamp: NaiveDateTime,
    pub ip: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Memberships {
    pub cid: u32,
    pub rating: u32,
    pub facility_id: String,
    pub r#type: i32,
    pub joined: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Migrations {
    pub id: i32,
    pub migration: String,
    pub batch: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OauthClients {
    pub id: u64,
    pub name: String,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub redirect_uris: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OauthLogins {
    pub id: u64,
    pub token: Option<String>,
    pub code: Option<String>,
    pub user_agent: Option<String>,
    pub redirect_uri: Option<String>,
    pub client_id: Option<u64>,
    pub state: Option<String>,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<String>,
    pub scope: Option<String>,
    pub c_id: Option<u64>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OtsEvals {
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

#[derive(Debug, Serialize, ToSchema)]
pub struct OtsEvalsForms {
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

#[derive(Debug, Serialize, ToSchema)]
pub struct OtsEvalsIndicatorResults {
    pub id: u32,
    pub perf_indicator_id: u32,
    pub eval_id: u32,
    pub result: i16,
    pub comment: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OtsEvalsPerfCats {
    pub id: u32,
    pub label: String,
    pub form_id: u32,
    pub r#order: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OtsEvalsPerfIndicators {
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

#[derive(Debug, Serialize, ToSchema)]
pub struct PasswordResets {
    pub email: String,
    pub token: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Policies {
    pub id: u32,
    pub ident: String,
    pub category: u32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub extension: String,
    pub effective_date: Option<NaiveDate>,
    pub perms: String,
    pub visible: bool,
    pub r#order: u16,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PolicyCategories {
    pub id: u32,
    pub name: String,
    pub r#order: u16,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Promotions {
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

#[derive(Debug, Serialize, ToSchema)]
pub struct Promotionstest {
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
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PushLog {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDate>,
    pub title: String,
    pub message: String,
    pub submitted_by: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Ratings {
    pub id: i32,
    pub short: String,
    pub long: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReturnPaths {
    pub id: u32,
    pub r#order: i32,
    pub facility_id: String,
    pub url: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RoleTitles {
    pub role: String,
    pub title: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Roles {
    pub id: u64,
    pub cid: u32,
    pub facility: String,
    pub role: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Sessions {
    pub id: String,
    pub user_id: Option<u32>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub payload: String,
    pub last_activity: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SoloCerts {
    pub id: u32,
    pub cid: u32,
    pub position: String,
    pub expires: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StatsArchive {
    pub id: i64,
    pub date: NaiveDate,
    pub data: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SurveyAssignments {
    pub id: String,
    pub survey_id: i32,
    pub facility: String,
    pub rating: i32,
    pub misc_data: String,
    pub completed: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SurveyQuestions {
    pub id: u32,
    pub survey_id: i32,
    pub question: String,
    pub data: String,
    pub r#order: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SurveySubmissions {
    pub id: u32,
    pub survey_id: i32,
    pub question_id: i32,
    pub response: String,
    pub facility: String,
    pub rating: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Surveys {
    pub id: u32,
    pub facility: String,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Tickets {
    pub id: u32,
    pub cid: i32,
    pub subject: String,
    pub body: String,
    pub status: String,
    pub facility: String,
    pub assigned_to: String,
    pub notes: String,
    pub priority: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TicketsHistory {
    pub id: u64,
    pub ticket_id: u64,
    pub entry: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TicketsNotes {
    pub id: u32,
    pub ticket_id: i32,
    pub cid: i32,
    pub note: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TicketsReplies {
    pub id: u32,
    pub ticket_id: i32,
    pub cid: i32,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TmuColors {
    pub id: String,
    pub black: Option<String>,
    pub brown: Option<String>,
    pub blue: Option<String>,
    pub gray: Option<String>,
    pub green: Option<String>,
    pub lime: Option<String>,
    pub cyan: Option<String>,
    pub orange: Option<String>,
    pub red: Option<String>,
    pub purple: Option<String>,
    pub white: Option<String>,
    pub yellow: Option<String>,
    pub violet: Option<String>,
    pub guide: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TmuFacilities {
    pub id: String,
    pub parent: Option<String>,
    pub name: String,
    pub coords: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TmuNotices {
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

#[derive(Debug, Serialize, ToSchema)]
pub struct TrainingBlocks {
    pub id: u64,
    pub facility: String,
    pub r#order: i32,
    pub name: String,
    pub level: String,
    pub visible: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TrainingChapters {
    pub id: u64,
    pub blockid: u64,
    pub r#order: i32,
    pub name: String,
    pub url: String,
    pub visible: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TrainingProgress {
    pub cid: u32,
    pub chapterid: u64,
    pub date: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TrainingRecords {
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

#[derive(Debug, Serialize, ToSchema)]
pub struct Transfers {
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

#[derive(Debug, Serialize, ToSchema)]
pub struct UlsTokens {
    pub facility: String,
    pub token: String,
    pub date: NaiveDateTime,
    pub ip: String,
    pub cid: u32,
    pub expired: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Users {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub remember_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Visits {
    pub id: u64,
    pub cid: u32,
    pub facility: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
