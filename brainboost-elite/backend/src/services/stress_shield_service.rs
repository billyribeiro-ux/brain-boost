use crate::errors::Result;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn check_stress_level(pool: &PgPool, user_id: Uuid) -> Result<bool> {
    let today = chrono::Utc::now().date_naive();
    
    let stress_index: Option<f64> = sqlx::query_scalar(
        r#"
        SELECT stress_index FROM daily_metrics
        WHERE user_id = $1 AND metric_date = $2
        "#,
    )
    .bind(user_id)
    .bind(today)
    .fetch_optional(pool)
    .await?
    .flatten();

    Ok(stress_index.map(|s| s > 70.0).unwrap_or(false))
}

pub async fn suggest_nsdr(pool: &PgPool, user_id: Uuid) -> Result<String> {
    let needs_nsdr = check_stress_level(pool, user_id).await?;
    
    if needs_nsdr {
        Ok("Your stress levels are elevated. I recommend a 10-minute NSDR (Non-Sleep Deep Rest) session to help you recover.".to_string())
    } else {
        Ok("Your stress levels are within healthy range. Keep up the great work!".to_string())
    }
}
