use crate::errors::Result;
use crate::services::metrics_service;
use chrono::Utc;
use sqlx::PgPool;
use tracing::info;

pub async fn run(pool: &PgPool) -> Result<()> {
    let yesterday = Utc::now().naive_utc().date() - chrono::Duration::days(1);
    
    let users = sqlx::query!(
        "SELECT DISTINCT user_id FROM user_levels WHERE status = 'active'"
    )
    .fetch_all(pool)
    .await?;
    
    for user in users {
        if let Err(e) = metrics_service::snapshot_daily_metrics(pool, user.user_id).await {
            tracing::error!("Failed to snapshot metrics for user {}: {}", user.user_id, e);
        }
    }
    
    info!("Completed nightly metrics snapshot for {} users", users.len());
    Ok(())
}
