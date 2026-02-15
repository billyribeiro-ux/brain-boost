use crate::errors::Result;
use crate::models::exercise::{CompleteExerciseRequest, ExerciseCompletion};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn complete_exercise(pool: &PgPool, user_id: Uuid, req: CompleteExerciseRequest) -> Result<ExerciseCompletion> {
    let metadata = req.metadata.unwrap_or(serde_json::json!({}));
    
    let completion = sqlx::query_as::<_, ExerciseCompletion>(
        r#"
        INSERT INTO exercise_completions 
        (session_id, user_id, exercise_type, exercise_name, duration_seconds, 
         accuracy_score, focus_rating, difficulty_level, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
    .bind(req.session_id)
    .bind(user_id)
    .bind(&req.exercise_type)
    .bind(&req.exercise_name)
    .bind(req.duration_seconds)
    .bind(req.accuracy_score)
    .bind(req.focus_rating)
    .bind(req.difficulty_level)
    .bind(metadata)
    .fetch_one(pool)
    .await?;

    Ok(completion)
}

pub async fn get_exercise_history(pool: &PgPool, user_id: Uuid, limit: i64) -> Result<Vec<ExerciseCompletion>> {
    let exercises = sqlx::query_as::<_, ExerciseCompletion>(
        r#"
        SELECT * FROM exercise_completions
        WHERE user_id = $1
        ORDER BY completed_at DESC
        LIMIT $2
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(exercises)
}
