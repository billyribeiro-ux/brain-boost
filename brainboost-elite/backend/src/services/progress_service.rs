use crate::errors::{AppError, Result};
use crate::models::progress::{CurrentProgressResponse, DailySession, UserLevel, UserLevelResponse};
use crate::services::metrics_service;
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

pub async fn check_daily_completion(pool: &PgPool, user_id: Uuid, date: chrono::NaiveDate) -> Result<bool> {
    let completed = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM daily_sessions
        WHERE user_id = $1 AND day_date = $2 AND status = 'completed'
        "#,
        user_id,
        date
    )
    .fetch_one(pool)
    .await?;
    
    Ok(completed == 3)
}

pub async fn calculate_compliance(pool: &PgPool, user_id: Uuid, level_number: i32) -> Result<f64> {
    let stats = sqlx::query!(
        r#"
        SELECT 
            COUNT(*) as total,
            COUNT(CASE WHEN status = 'completed' THEN 1 END) as completed
        FROM daily_sessions
        WHERE user_id = $1 AND level_number = $2
        "#,
        user_id,
        level_number
    )
    .fetch_one(pool)
    .await?;
    
    if stats.total.unwrap_or(0) == 0 {
        return Ok(0.0);
    }
    
    let compliance = (stats.completed.unwrap_or(0) as f64 / stats.total.unwrap_or(1) as f64) * 100.0;
    Ok(compliance)
}

pub async fn advance_day(pool: &PgPool, user_id: Uuid) -> Result<()> {
    let today = Utc::now().date_naive();
    
    let all_completed = check_daily_completion(pool, user_id, today).await?;
    
    if !all_completed {
        return Err(AppError::ValidationError(
            "Cannot advance: not all sessions completed today".to_string()
        ));
    }
    
    metrics_service::snapshot_daily_metrics(pool, user_id, today).await?;
    
    let current_level = sqlx::query!(
        r#"
        SELECT level_number, current_week, current_day
        FROM user_levels
        WHERE user_id = $1 AND status = 'active'
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;
    
    let mut new_day = current_level.current_day + 1;
    let mut new_week = current_level.current_week;
    let mut new_level = current_level.level_number;
    let mut level_completed = false;
    
    if new_day > 7 {
        new_day = 1;
        new_week += 1;
    }
    
    if new_week > 6 {
        let compliance = calculate_compliance(pool, user_id, current_level.level_number).await?;
        
        if compliance >= 80.0 {
            level_completed = true;
            
            sqlx::query(
                r#"
                UPDATE user_levels
                SET status = 'completed', completed_at = NOW()
                WHERE user_id = $1 AND level_number = $2
                "#
            )
            .bind(user_id)
            .bind(current_level.level_number)
            .execute(pool)
            .await?;
            
            if current_level.level_number < 6 {
                new_level = current_level.level_number + 1;
                new_week = 1;
                new_day = 1;
                
                sqlx::query(
                    r#"
                    UPDATE user_levels
                    SET status = 'active', started_at = NOW(), current_week = 1, current_day = 1
                    WHERE user_id = $1 AND level_number = $2
                    "#
                )
                .bind(user_id)
                .bind(new_level)
                .execute(pool)
                .await?;
            }
        } else {
            new_week = 1;
            new_day = 1;
        }
    }
    
    if !level_completed {
        sqlx::query(
            r#"
            UPDATE user_levels
            SET current_week = $1, current_day = $2
            WHERE user_id = $3 AND level_number = $4
            "#
        )
        .bind(new_week)
        .bind(new_day)
        .bind(user_id)
        .bind(current_level.level_number)
        .execute(pool)
        .await?;
    }
    
    Ok(())
}

pub async fn handle_missed_day(pool: &PgPool, user_id: Uuid) -> Result<()> {
    let today = Utc::now().date_naive();
    
    sqlx::query(
        r#"
        UPDATE daily_sessions
        SET status = 'missed'
        WHERE user_id = $1 AND day_date = $2 AND status = 'pending'
        "#
    )
    .bind(user_id)
    .bind(today)
    .execute(pool)
    .await?;
    
    let current_level = sqlx::query!(
        r#"
        SELECT level_number, grace_skips_used
        FROM user_levels
        WHERE user_id = $1 AND status = 'active'
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;
    
    let grace_skips_used = current_level.grace_skips_used + 1;
    
    if grace_skips_used > 3 {
        sqlx::query(
            r#"
            UPDATE user_levels
            SET current_week = 1, current_day = 1, grace_skips_used = 0
            WHERE user_id = $1 AND level_number = $2
            "#
        )
        .bind(user_id)
        .bind(current_level.level_number)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            r#"
            UPDATE user_levels
            SET grace_skips_used = $1
            WHERE user_id = $2 AND level_number = $3
            "#
        )
        .bind(grace_skips_used)
        .bind(user_id)
        .bind(current_level.level_number)
        .execute(pool)
        .await?;
    }
    
    Ok(())
}
