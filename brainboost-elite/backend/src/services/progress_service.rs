use crate::errors::Result;
use crate::models::progress::{CurrentProgressResponse, DailySession, UserLevel, UserLevelResponse, DailySessionResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_current_progress(pool: &PgPool, user_id: Uuid) -> Result<CurrentProgressResponse> {
    let current_level = sqlx::query_as::<_, UserLevel>(
        r#"
        SELECT * FROM user_levels 
        WHERE user_id = $1 AND status = 'active'
        ORDER BY level_number ASC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    let today = Utc::now().date_naive();
    let sessions = sqlx::query_as::<_, DailySession>(
        r#"
        SELECT * FROM daily_sessions
        WHERE user_id = $1 AND day_date = $2
        ORDER BY slot ASC
        "#,
    )
    .bind(user_id)
    .bind(today)
    .fetch_all(pool)
    .await?;

    Ok(CurrentProgressResponse {
        current_level: current_level.into(),
        today_sessions: sessions.into_iter().map(|s| s.into()).collect(),
    })
}

pub async fn get_all_levels(pool: &PgPool, user_id: Uuid) -> Result<Vec<UserLevelResponse>> {
    let levels = sqlx::query_as::<_, UserLevel>(
        r#"
        SELECT * FROM user_levels
        WHERE user_id = $1
        ORDER BY level_number ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(levels.into_iter().map(|l| l.into()).collect())
}
