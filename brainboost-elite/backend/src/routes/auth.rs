use crate::errors::Result;
use crate::middleware::auth::AuthUser;
use crate::models::user::{AuthResponse, CreateUserRequest, LoginRequest};
use crate::services::auth_service;
use axum::{extract::State, Extension, Json};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RefreshRequest {
    #[validate(length(min = 1, max = 500))]
    pub refresh_token: String,
}

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
    Json(req): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>> {
    req.validate()
        .map_err(|e| crate::errors::AppError::ValidationError(e.to_string()))?;

    let response = auth_service::refresh_token(&pool, &req.refresh_token).await?;
    Ok(Json(response))
}

pub async fn logout(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>> {
    auth_service::logout_user(&pool, auth_user.user_id).await?;
    Ok(Json(serde_json::json!({
        "message": "Logged out successfully"
    })))
}
