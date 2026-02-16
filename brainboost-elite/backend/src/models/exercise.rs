use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExerciseCompletion {
    pub id: Uuid,
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub exercise_type: String,
    pub exercise_name: String,
    pub duration_seconds: i32,
    pub accuracy_score: Option<f64>,
    pub focus_rating: Option<i16>,
    pub difficulty_level: Option<i16>,
    pub metadata: JsonValue,
    pub completed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CompleteExerciseRequest {
    pub session_id: Uuid,
    #[validate(length(min = 1, max = 100))]
    pub exercise_type: String,
    #[validate(length(min = 1, max = 200))]
    pub exercise_name: String,
    #[validate(range(min = 1, max = 7200))]
    pub duration_seconds: i32,
    #[validate(range(min = 0.0, max = 100.0))]
    pub accuracy_score: Option<f64>,
    #[validate(range(min = 1, max = 10))]
    pub focus_rating: Option<i16>,
    #[validate(range(min = 1, max = 10))]
    pub difficulty_level: Option<i16>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Serialize)]
pub struct ExerciseHistoryResponse {
    pub exercises: Vec<ExerciseCompletion>,
    pub total_count: i64,
}
