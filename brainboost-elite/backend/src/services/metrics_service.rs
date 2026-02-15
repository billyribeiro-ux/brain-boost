use crate::errors::Result;
use crate::models::metrics::{DailyMetrics, MetricsResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_today_metrics(pool: &PgPool, user_id: Uuid) -> Result<MetricsResponse> {
    let today = Utc::now().date_naive();
    
    let metrics = sqlx::query_as::<_, DailyMetrics>(
        r#"
        SELECT * FROM daily_metrics
        WHERE user_id = $1 AND metric_date = $2
        "#,
    )
    .bind(user_id)
    .bind(today)
    .fetch_optional(pool)
    .await?;

    if let Some(m) = metrics {
        Ok(m.into())
    } else {
        let new_metrics = sqlx::query_as::<_, DailyMetrics>(
            r#"
            INSERT INTO daily_metrics (user_id, metric_date)
            VALUES ($1, $2)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(today)
        .fetch_one(pool)
        .await?;

        Ok(new_metrics.into())
    }
}

pub async fn get_metrics_trend(pool: &PgPool, user_id: Uuid, days: i32) -> Result<Vec<MetricsResponse>> {
    let metrics = sqlx::query_as::<_, DailyMetrics>(
        r#"
        SELECT * FROM daily_metrics
        WHERE user_id = $1
        ORDER BY metric_date DESC
        LIMIT $2
        "#,
    )
    .bind(user_id)
    .bind(days as i64)
    .fetch_all(pool)
    .await?;

    Ok(metrics.into_iter().map(|m| m.into()).collect())
}
