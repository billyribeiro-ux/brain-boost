use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserLevel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub level_number: i16,
    pub status: String,
    pub current_week: i16,
    pub current_day: i16,
    pub grace_skip_used: bool,
    pub compliance_rate: f64,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub restarted_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLevelResponse {
    pub id: Uuid,
    pub level_number: i16,
    pub status: String,
    pub current_week: i16,
    pub current_day: i16,
    pub grace_skip_used: bool,
    pub compliance_rate: f64,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<UserLevel> for UserLevelResponse {
    fn from(level: UserLevel) -> Self {
        UserLevelResponse {
            id: level.id,
            level_number: level.level_number,
            status: level.status,
            current_week: level.current_week,
            current_day: level.current_day,
            grace_skip_used: level.grace_skip_used,
            compliance_rate: level.compliance_rate,
            started_at: level.started_at,
            completed_at: level.completed_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DailySession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub level_number: i16,
    pub week_number: i16,
    pub day_number: i16,
    pub day_date: NaiveDate,
    pub slot: String,
    pub status: String,
    pub scheduled_time: Option<NaiveTime>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_planned_seconds: i32,
    pub duration_actual_seconds: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySessionResponse {
    pub id: Uuid,
    pub level_number: i16,
    pub week_number: i16,
    pub day_number: i16,
    pub day_date: NaiveDate,
    pub slot: String,
    pub status: String,
    pub scheduled_time: Option<NaiveTime>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_planned_seconds: i32,
    pub duration_actual_seconds: Option<i32>,
}

impl From<DailySession> for DailySessionResponse {
    fn from(session: DailySession) -> Self {
        DailySessionResponse {
            id: session.id,
            level_number: session.level_number,
            week_number: session.week_number,
            day_number: session.day_number,
            day_date: session.day_date,
            slot: session.slot,
            status: session.status,
            scheduled_time: session.scheduled_time,
            started_at: session.started_at,
            completed_at: session.completed_at,
            duration_planned_seconds: session.duration_planned_seconds,
            duration_actual_seconds: session.duration_actual_seconds,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StartSessionRequest {
    pub session_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CompleteSessionRequest {
    pub session_id: Uuid,
    pub duration_actual_seconds: i32,
}

#[derive(Debug, Serialize)]
pub struct CurrentProgressResponse {
    pub current_level: UserLevelResponse,
    pub today_sessions: Vec<DailySessionResponse>,
}
