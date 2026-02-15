use crate::errors::Result;
use crate::realtime::{ConnectionManager, ServerMessage};
use crate::realtime::messages::DayMissedPayload;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::info;

pub async fn run(pool: &PgPool, manager: &Arc<ConnectionManager>) -> Result<()> {
    let yesterday = Utc::now().naive_utc().date() - chrono::Duration::days(1);
    
    sqlx::query(
        r#"
        UPDATE daily_sessions
        SET status = 'missed'
        WHERE session_date = $1
        AND status = 'pending'
        "#
    )
    .bind(yesterday)
    .execute(pool)
    .await?;
    
    let users_with_all_missed = sqlx::query!(
        r#"
        SELECT user_id, COUNT(*) as missed_count
        FROM daily_sessions
        WHERE session_date = $1 AND status = 'missed'
        GROUP BY user_id
        HAVING COUNT(*) = 3
        "#,
        yesterday
    )
    .fetch_all(pool)
    .await?;
    
    for user in users_with_all_missed {
        let level = sqlx::query!(
            r#"
            SELECT grace_skip_available, restarted_count
            FROM user_levels
            WHERE user_id = $1 AND status = 'active'
            "#,
            user.user_id
        )
        .fetch_one(pool)
        .await?;
        
        if level.grace_skip_available {
            let message = ServerMessage::DayMissedWarning(DayMissedPayload {
                missed_date: yesterday.to_string(),
                grace_skip_available: true,
                message: "You missed yesterday. Use your grace skip to continue?".to_string(),
            });
            
            manager.send_to_user(user.user_id, message).await;
        } else {
            sqlx::query(
                r#"
                UPDATE user_levels
                SET current_week = 1, current_day = 1, 
                    restarted_count = restarted_count + 1,
                    compliance_rate = 0
                WHERE user_id = $1 AND status = 'active'
                "#
            )
            .bind(user.user_id)
            .execute(pool)
            .await?;
            
            let message = ServerMessage::DayMissedWarning(DayMissedPayload {
                missed_date: yesterday.to_string(),
                grace_skip_available: false,
                message: "Level restarted due to missed day. Let's start fresh!".to_string(),
            });
            
            manager.send_to_user(user.user_id, message).await;
            info!("Restarted level for user {} due to missed day", user.user_id);
        }
    }
    
    Ok(())
}
