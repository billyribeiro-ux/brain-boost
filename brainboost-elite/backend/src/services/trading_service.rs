use crate::errors::Result;
use crate::models::trading::{StartTradingRequest, TradingSession, TradingSessionResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn start_trading_session(pool: &PgPool, user_id: Uuid, req: StartTradingRequest) -> Result<TradingSessionResponse> {
    let starting_balance = req.starting_balance.unwrap_or(100000.0);
    
    let session = sqlx::query_as::<_, TradingSession>(
        r#"
        INSERT INTO trading_sessions (user_id, scenario_type, starting_balance)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(&req.scenario_type)
    .bind(starting_balance)
    .fetch_one(pool)
    .await?;

    Ok(session.into())
}

pub async fn get_trading_history(pool: &PgPool, user_id: Uuid, limit: i64) -> Result<Vec<TradingSessionResponse>> {
    let sessions = sqlx::query_as::<_, TradingSession>(
        r#"
        SELECT * FROM trading_sessions
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(sessions.into_iter().map(|s| s.into()).collect())
}
