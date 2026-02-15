use crate::errors::Result;
use crate::middleware::auth::AuthUser;
use crate::models::session::{CoachMessage, SendMessageRequest};
use crate::services::{coach_service, stress_shield_service};
use axum::{extract::State, Extension, Json};
use sqlx::PgPool;

pub async fn send_message(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<SendMessageRequest>,
) -> Result<Json<CoachMessage>> {
    let message = coach_service::send_message(&pool, auth_user.user_id, req).await?;
    Ok(Json(message))
}

pub async fn get_suggestions(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>> {
    let nsdr_suggestion = stress_shield_service::suggest_nsdr(&pool, auth_user.user_id).await?;
    
    Ok(Json(serde_json::json!({
        "suggestions": [
            {
                "type": "stress_check",
                "message": nsdr_suggestion
            }
        ]
    })))
}

pub async fn get_history(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<CoachMessage>>> {
    let history = coach_service::get_message_history(&pool, auth_user.user_id, 50).await?;
    Ok(Json(history))
}
