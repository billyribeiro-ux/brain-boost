use crate::errors::Result;
use crate::models::metrics::{DailyMetrics, MetricsResponse};
use chrono::{Duration, NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn calculate_brain_score(pool: &PgPool, user_id: Uuid, date: NaiveDate) -> Result<f64> {
    let sessions = sqlx::query!(
        r#"
        SELECT COUNT(*) as total, 
               COUNT(CASE WHEN status = 'completed' THEN 1 END) as completed
        FROM daily_sessions
        WHERE user_id = $1 AND day_date = $2
        "#,
        user_id,
        date
    )
    .fetch_one(pool)
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
        user_id,
        date
    )
    .fetch_one(pool)
    .await?;
    
    let avg_accuracy = exercise_stats.avg_accuracy.unwrap_or(0.0);
    let avg_focus = (exercise_stats.avg_focus.unwrap_or(0.0) / 10.0) * 100.0;
    
    let stress_index = calculate_stress_index(pool, user_id, date).await?;
    
    let brain_score = (completion_rate * 0.40) 
        + (avg_accuracy * 0.30) 
        + (avg_focus * 0.20) 
        + ((100.0 - stress_index) * 0.10);
    
    Ok(brain_score.min(100.0).max(0.0))
}

pub async fn calculate_stress_index(pool: &PgPool, user_id: Uuid, date: NaiveDate) -> Result<f64> {
    let metrics = sqlx::query!(
        r#"
        SELECT sleep_quality, hrv_score
        FROM daily_metrics
        WHERE user_id = $1 AND metric_date = $2
        "#,
        user_id,
        date
    )
    .fetch_optional(pool)
    .await?;
    
    if let Some(m) = metrics {
        if let Some(hrv) = m.hrv_score {
            let normalized_hrv = (hrv / 100.0) * 100.0;
            let sleep_component = (100.0 - (m.sleep_quality.unwrap_or(5.0) * 10.0)) * 0.3;
            let stress_index = ((100.0 - normalized_hrv) * 0.5) + sleep_component + 20.0;
            return Ok(stress_index.min(100.0).max(0.0));
        }
    }
    
    let exercise_stats = sqlx::query!(
        r#"
        SELECT AVG(focus_rating) as avg_focus
        FROM exercise_completions
        WHERE user_id = $1 AND DATE(completed_at) = $2
        "#,
        user_id,
        date
    )
    .fetch_one(pool)
    .await?;
    
    let avg_focus = exercise_stats.avg_focus.unwrap_or(5.0);
    let sleep_quality = metrics.and_then(|m| m.sleep_quality).unwrap_or(5.0);
    
    let missed_sessions = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*) FROM daily_sessions
        WHERE user_id = $1 AND day_date = $2 AND status = 'missed'
        "#
    )
    .bind(user_id)
    .bind(date)
    .fetch_one(pool)
    .await?;
    
    let stress_index = ((100.0 - sleep_quality * 10.0) * 0.5) 
        + ((10.0 - avg_focus) * 10.0 * 0.3) 
        + (missed_sessions as f64 * 20.0 * 0.2);
    
    Ok(stress_index.min(100.0).max(0.0))
}

pub async fn calculate_longevity_score(pool: &PgPool, user_id: Uuid) -> Result<f64> {
    let end_date = Utc::now().date_naive();
    let start_date = end_date - Duration::days(7);
    
    let exercise_minutes = sqlx::query_scalar::<_, Option<i64>>(
        r#"
        SELECT SUM(duration_seconds) / 60
        FROM exercise_completions
        WHERE user_id = $1 
        AND DATE(completed_at) >= $2 
        AND DATE(completed_at) <= $3
        AND exercise_type IN ('exercise_light', 'exercise_brisk', 'hiit')
        "#
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_one(pool)
    .await?
    .unwrap_or(0);
    
    let avg_sleep = sqlx::query_scalar::<_, Option<f64>>(
        r#"
        SELECT AVG(sleep_quality)
        FROM daily_metrics
        WHERE user_id = $1 AND metric_date >= $2 AND metric_date <= $3
        "#
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_one(pool)
    .await?
    .unwrap_or(5.0);
    
    let avg_brain_score = sqlx::query_scalar::<_, Option<f64>>(
        r#"
        SELECT AVG(brain_score)
        FROM daily_metrics
        WHERE user_id = $1 AND metric_date >= $2 AND metric_date <= $3
        "#
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_one(pool)
    .await?
    .unwrap_or(50.0);
    
    let meditation_streak = calculate_streak(pool, user_id).await?;
    
    let stress_index = calculate_stress_index(pool, user_id, end_date).await?;
    
    let mut score = 0.0;
    
    if exercise_minutes >= 150 {
        score += 15.0;
    } else {
        score += (exercise_minutes as f64 / 150.0) * 15.0;
    }
    
    if avg_sleep >= 7.0 {
        score += 10.0;
    } else {
        score += (avg_sleep / 7.0) * 10.0;
    }
    
    if avg_brain_score >= 70.0 {
        score += 15.0;
    } else {
        score += (avg_brain_score / 70.0) * 15.0;
    }
    
    score += 5.0;
    
    let diet_logged = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(DISTINCT DATE(completed_at))
        FROM exercise_completions
        WHERE user_id = $1 
        AND DATE(completed_at) >= $2 
        AND DATE(completed_at) <= $3
        AND exercise_type = 'diet_log'
        "#
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_one(pool)
    .await?;
    
    if diet_logged >= 5 {
        score += 10.0;
    } else {
        score += (diet_logged as f64 / 5.0) * 10.0;
    }
    
    if meditation_streak >= 7 {
        score += 10.0;
    } else {
        score += (meditation_streak as f64 / 7.0) * 10.0;
    }
    
    score += 10.0;
    
    if stress_index < 40.0 {
        score += 10.0;
    } else {
        score += ((100.0 - stress_index) / 60.0) * 10.0;
    }
    
    score += 5.0 + 5.0 + 3.0 + 2.0;
    
    Ok(score.min(100.0).max(0.0))
}

pub async fn calculate_streak(pool: &PgPool, user_id: Uuid) -> Result<i32> {
    let mut current_date = Utc::now().date_naive();
    let mut streak = 0;
    
    loop {
        let completed = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM daily_sessions
            WHERE user_id = $1 AND session_date = $2 AND status = 'completed'
            "#
        )
        .bind(user_id)
        .bind(current_date)
        .fetch_one(pool)
        .await?;
        
        let total = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM daily_sessions
            WHERE user_id = $1 AND session_date = $2
            "#
        )
        .bind(user_id)
        .bind(current_date)
        .fetch_one(pool)
        .await?;
        
        if total == 0 {
            break;
        }
        
        if completed == total && total == 3 {
            streak += 1;
            current_date = current_date.pred_opt().unwrap();
        } else {
            break;
        }
        
        if streak > 365 {
            break;
        }
    }
    
    Ok(streak)
}

pub async fn snapshot_daily_metrics(pool: &PgPool, user_id: Uuid, date: NaiveDate) -> Result<()> {
    let brain_score = calculate_brain_score(pool, user_id, date).await?;
    let stress_index = calculate_stress_index(pool, user_id, date).await?;
    let longevity_score = calculate_longevity_score(pool, user_id).await?;
    let streak = calculate_streak(pool, user_id).await?;
    
    let sessions = sqlx::query!(
        r#"
        SELECT COUNT(*) as total, 
               COUNT(CASE WHEN status = 'completed' THEN 1 END) as completed
        FROM daily_sessions
        WHERE user_id = $1 AND day_date = $2
        "#,
        user_id,
        date
    )
    .fetch_one(pool)
    .await?;
    
    let completion_rate = if sessions.total.unwrap_or(0) > 0 {
        (sessions.completed.unwrap_or(0) as f64 / sessions.total.unwrap_or(1) as f64) * 100.0
    } else {
        0.0
    };
    
    sqlx::query(
        r#"
        INSERT INTO daily_metrics 
        (user_id, metric_date, brain_score, stress_index, longevity_score, 
         streak_days, completion_rate)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (user_id, metric_date) 
        DO UPDATE SET
            brain_score = EXCLUDED.brain_score,
            stress_index = EXCLUDED.stress_index,
            longevity_score = EXCLUDED.longevity_score,
            streak_days = EXCLUDED.streak_days,
            completion_rate = EXCLUDED.completion_rate,
            updated_at = NOW()
        "#,
    )
    .bind(user_id)
    .bind(date)
    .bind(brain_score)
    .bind(stress_index)
    .bind(longevity_score)
    .bind(streak)
    .bind(completion_rate)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn get_today_metrics(pool: &PgPool, user_id: Uuid) -> Result<MetricsResponse> {
    let today = Utc::now().date_naive();
    
    snapshot_daily_metrics(pool, user_id, today).await?;
    
    let metrics = sqlx::query_as::<_, DailyMetrics>(
        r#"
        SELECT * FROM daily_metrics
        WHERE user_id = $1 AND metric_date = $2
        "#,
    )
    .bind(user_id)
    .bind(today)
    .fetch_one(pool)
    .await?;

    Ok(metrics.into())
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
