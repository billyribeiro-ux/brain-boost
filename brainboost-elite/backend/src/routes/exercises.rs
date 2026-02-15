use crate::errors::Result;
use crate::middleware::auth::AuthUser;
use crate::models::exercise::{CompleteExerciseRequest, ExerciseCompletion};
use crate::services::exercise_service;
use axum::{extract::State, Extension, Json};
use sqlx::PgPool;

pub async fn get_today_exercises(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "Today's exercise plan - to be implemented with curriculum logic",
        "exercises": []
    })))
}

pub async fn complete_exercise(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CompleteExerciseRequest>,
) -> Result<Json<ExerciseCompletion>> {
    let completion = exercise_service::complete_exercise(&pool, auth_user.user_id, req).await?;
    Ok(Json(completion))
}

pub async fn get_exercise_history(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<ExerciseCompletion>>> {
    let history = exercise_service::get_exercise_history(&pool, auth_user.user_id, 50).await?;
    Ok(Json(history))
}
