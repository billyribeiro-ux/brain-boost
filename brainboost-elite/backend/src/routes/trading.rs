use crate::errors::Result;
use crate::middleware::auth::AuthUser;
use crate::models::trading::{StartTradingRequest, TradingSessionResponse, TradingStatsResponse};
use crate::services::trading_service;
use axum::{extract::State, Extension, Json};
use sqlx::PgPool;

pub async fn start_trading(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<StartTradingRequest>,
) -> Result<Json<TradingSessionResponse>> {
    let session = trading_service::start_trading_session(&pool, auth_user.user_id, req).await?;
    Ok(Json(session))
}

pub async fn make_decision(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "Trading decision recorded - full implementation pending"
    })))
}

pub async fn get_trading_history(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<TradingSessionResponse>>> {
    let history = trading_service::get_trading_history(&pool, auth_user.user_id, 20).await?;
    Ok(Json(history))
}

pub async fn get_trading_stats(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<TradingStatsResponse>> {
    Ok(Json(TradingStatsResponse {
        total_sessions: 0,
        total_trades: 0,
        win_rate: 0.0,
        avg_expected_value: 0.0,
    }))
}
