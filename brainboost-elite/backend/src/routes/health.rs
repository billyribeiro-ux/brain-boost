use axum::{extract::State, Json};
use sqlx::PgPool;

pub async fn health_check(State(pool): State<PgPool>) -> Json<serde_json::Value> {
    let db_status = match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    Json(serde_json::json!({
        "status": "ok",
        "db": db_status
    }))
}
