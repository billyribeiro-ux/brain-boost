use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DailyMetrics {
    pub id: Uuid,
    pub user_id: Uuid,
    pub metric_date: NaiveDate,
    pub brain_score: Option<f64>,
    pub stress_index: Option<f64>,
    pub longevity_score: Option<f64>,
    pub completion_rate: Option<f64>,
    pub avg_accuracy: Option<f64>,
    pub avg_focus: Option<f64>,
    pub total_session_minutes: Option<i32>,
    pub streak_days: i32,
    pub hrv_reading: Option<f64>,
    pub sleep_hours: Option<f64>,
    pub sleep_quality: Option<i16>,
    pub exercise_minutes: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct MetricsResponse {
    pub brain_score: Option<f64>,
    pub stress_index: Option<f64>,
    pub longevity_score: Option<f64>,
    pub completion_rate: Option<f64>,
    pub streak_days: i32,
    pub metric_date: NaiveDate,
}

impl From<DailyMetrics> for MetricsResponse {
    fn from(metrics: DailyMetrics) -> Self {
        MetricsResponse {
            brain_score: metrics.brain_score,
            stress_index: metrics.stress_index,
            longevity_score: metrics.longevity_score,
            completion_rate: metrics.completion_rate,
            streak_days: metrics.streak_days,
            metric_date: metrics.metric_date,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MetricsTrendResponse {
    pub metrics: Vec<MetricsResponse>,
}
