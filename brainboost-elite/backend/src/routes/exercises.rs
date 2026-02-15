use crate::errors::Result;
use crate::middleware::auth::AuthUser;
use crate::models::exercise::{CompleteExerciseRequest, ExerciseCompletion};
use crate::services::exercise_service;
use axum::{extract::{Path, Query, State}, Extension, Json};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct HistoryQuery {
    #[serde(default = "default_days")]
    days: i64,
}

fn default_days() -> i64 {
    30
}

#[derive(serde::Serialize)]
pub struct TodayExercisesResponse {
    pub sessions: Vec<SessionWithPlan>,
}

#[derive(serde::Serialize)]
pub struct SessionWithPlan {
    pub session_id: Uuid,
    pub slot: String,
    pub status: String,
    pub level_number: i32,
    pub week_number: i32,
    pub day_number: i32,
    pub exercises: Vec<exercise_service::PlannedExercise>,
}

pub async fn get_today_exercises(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<TodayExercisesResponse>> {
    let sessions = exercise_service::get_today_sessions(&pool, auth_user.user_id).await?;
    
    let mut sessions_with_plans = Vec::new();
    
    for session in sessions {
        let exercises = exercise_service::get_daily_plan(
            session.level_number as u8,
            session.week_number as u8,
            &session.slot,
        );
        
        sessions_with_plans.push(SessionWithPlan {
            session_id: session.id,
            slot: session.slot.clone(),
            status: session.status.clone(),
            level_number: session.level_number as i32,
            week_number: session.week_number as i32,
            day_number: session.day_number as i32,
            exercises,
        });
    }
    
    Ok(Json(TodayExercisesResponse {
        sessions: sessions_with_plans,
    }))
}

pub async fn complete_exercise(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Path(session_id): Path<Uuid>,
    Json(req): Json<CompleteExerciseRequest>,
) -> Result<Json<ExerciseCompletion>> {
    let completion = exercise_service::complete_exercise(&pool, auth_user.user_id, req).await?;
    Ok(Json(completion))
}

pub async fn get_exercise_history(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<Vec<ExerciseCompletion>>> {
    let exercises = exercise_service::get_exercise_history(&pool, auth_user.user_id, query.days).await?;
    Ok(Json(exercises))
}
