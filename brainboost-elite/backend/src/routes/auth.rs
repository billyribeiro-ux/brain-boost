use crate::errors::Result;
use crate::models::user::{AuthResponse, CreateUserRequest, LoginRequest};
use crate::services::auth_service;
use axum::{extract::State, Json};
use sqlx::PgPool;
use validator::Validate;

pub async fn register(
    State(pool): State<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<AuthResponse>> {
    req.validate()
        .map_err(|e| crate::errors::AppError::ValidationError(e.to_string()))?;

    let response = auth_service::register_user(&pool, req).await?;
    Ok(Json(response))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    req.validate()
        .map_err(|e| crate::errors::AppError::ValidationError(e.to_string()))?;

    let response = auth_service::login_user(&pool, req).await?;
    Ok(Json(response))
}

pub async fn refresh(
    State(pool): State<PgPool>,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "Token refresh endpoint - to be implemented with refresh token validation"
    })))
}

pub async fn logout(
    State(pool): State<PgPool>,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "Logged out successfully"
    })))
}
