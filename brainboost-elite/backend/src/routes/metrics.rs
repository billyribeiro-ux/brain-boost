use crate::errors::Result;
use crate::middleware::auth::AuthUser;
use crate::models::metrics::MetricsResponse;
use crate::services::metrics_service;
use axum::{extract::{Query, State}, Extension, Json};
use serde::Deserialize;
use sqlx::PgPool;
use chrono::Utc;

#[derive(Deserialize)]
pub struct TrendQuery {
    #[serde(default = "default_days")]
    days: i32,
}

fn default_days() -> i32 {
    30
}

pub async fn get_today_metrics(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<MetricsResponse>> {
    let metrics = metrics_service::get_today_metrics(&pool, auth_user.user_id).await?;
    Ok(Json(metrics))
}

pub async fn get_metrics_trend(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Query(query): Query<TrendQuery>,
) -> Result<Json<Vec<MetricsResponse>>> {
    let metrics = metrics_service::get_metrics_trend(&pool, auth_user.user_id, query.days).await?;
    Ok(Json(metrics))
}

pub async fn get_brain_score(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>> {
    let today = Utc::now().date_naive();
    let brain_score = metrics_service::calculate_brain_score(&pool, auth_user.user_id, today).await?;
    
    let sessions = sqlx::query!(
        r#"
        SELECT COUNT(*) as total, 
               COUNT(CASE WHEN status = 'completed' THEN 1 END) as completed
        FROM daily_sessions
        WHERE user_id = $1 AND day_date = $2
        "#,
        auth_user.user_id,
        today
    )
    .fetch_one(&pool)
    .await?;
    
    let completion_rate = if sessions.total.unwrap_or(0) > 0 {
        (sessions.completed.unwrap_or(0) as f64 / sessions.total.unwrap_or(1) as f64) * 100.0
    } else {
        0.0
    };
    
    let exercise_stats = sqlx::query!(
        r#"
        SELECT AVG(accuracy_score) as avg_accuracy, AVG(focus_rating) as avg_focus
        FROM exercise_completions
        WHERE user_id = $1 AND DATE(completed_at) = $2
        "#,
        auth_user.user_id,
        today
    )
    .fetch_one(&pool)
    .await?;
    
    let avg_accuracy = exercise_stats.avg_accuracy
        .map(|bd| bd.to_string().parse::<f64>().unwrap_or(0.0))
        .unwrap_or(0.0);
    let avg_focus_raw = exercise_stats.avg_focus
        .map(|bd| bd.to_string().parse::<f64>().unwrap_or(0.0))
        .unwrap_or(0.0);
    let avg_focus = (avg_focus_raw / 10.0) * 100.0;
    let stress_index = metrics_service::calculate_stress_index(&pool, auth_user.user_id, today).await?;
    
    Ok(Json(serde_json::json!({
        "brain_score": brain_score,
        "breakdown": {
            "completion": completion_rate,
            "accuracy": avg_accuracy,
            "focus": avg_focus,
            "stress": stress_index
        }
    })))
}
