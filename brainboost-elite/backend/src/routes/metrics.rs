use crate::errors::Result;
use crate::middleware::auth::AuthUser;
use crate::models::metrics::{MetricsResponse, MetricsTrendResponse};
use crate::services::metrics_service;
use axum::{extract::{Query, State}, Extension, Json};
use serde::Deserialize;
use sqlx::PgPool;

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
) -> Result<Json<MetricsTrendResponse>> {
    let metrics = metrics_service::get_metrics_trend(&pool, auth_user.user_id, query.days).await?;
    Ok(Json(MetricsTrendResponse { metrics }))
}

pub async fn get_brain_score(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>> {
    let metrics = metrics_service::get_today_metrics(&pool, auth_user.user_id).await?;
    Ok(Json(serde_json::json!({
        "brain_score": metrics.brain_score,
        "metric_date": metrics.metric_date
    })))
}
