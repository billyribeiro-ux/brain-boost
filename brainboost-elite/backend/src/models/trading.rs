use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TradingSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub scenario_type: String,
    pub starting_balance: f64,
    pub ending_balance: Option<f64>,
    pub total_trades: i32,
    pub winning_trades: i32,
    pub expected_value_score: Option<f64>,
    pub bias_detections: JsonValue,
    pub process_score: Option<f64>,
    pub decision_speed_ms: Option<i32>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TradingDecision {
    pub id: Uuid,
    pub trading_session_id: Uuid,
    pub decision_type: String,
    pub asset_symbol: String,
    pub price_at_decision: f64,
    pub quantity: Option<f64>,
    pub reasoning: Option<String>,
    pub ai_feedback: Option<String>,
    pub bias_detected: Option<Vec<String>>,
    pub was_optimal: Option<bool>,
    pub pnl: Option<f64>,
    pub decision_time_ms: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct StartTradingRequest {
    pub scenario_type: String,
    pub starting_balance: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct MakeDecisionRequest {
    pub trading_session_id: Uuid,
    pub decision_type: String,
    pub asset_symbol: String,
    pub price_at_decision: f64,
    pub quantity: Option<f64>,
    pub reasoning: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TradingSessionResponse {
    pub id: Uuid,
    pub scenario_type: String,
    pub starting_balance: f64,
    pub ending_balance: Option<f64>,
    pub total_trades: i32,
    pub winning_trades: i32,
    pub expected_value_score: Option<f64>,
    pub process_score: Option<f64>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<TradingSession> for TradingSessionResponse {
    fn from(session: TradingSession) -> Self {
        TradingSessionResponse {
            id: session.id,
            scenario_type: session.scenario_type,
            starting_balance: session.starting_balance,
            ending_balance: session.ending_balance,
            total_trades: session.total_trades,
            winning_trades: session.winning_trades,
            expected_value_score: session.expected_value_score,
            process_score: session.process_score,
            started_at: session.started_at,
            completed_at: session.completed_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TradingStatsResponse {
    pub total_sessions: i64,
    pub total_trades: i64,
    pub win_rate: f64,
    pub avg_expected_value: f64,
}
