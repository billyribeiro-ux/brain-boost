use crate::errors::Result;
use crate::middleware::auth::AuthUser;
use crate::models::progress::{CurrentProgressResponse, UserLevelResponse};
use crate::services::progress_service;
use axum::{extract::State, Extension, Json};
use sqlx::PgPool;

pub async fn get_current_progress(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<CurrentProgressResponse>> {
    let progress = progress_service::get_current_progress(&pool, auth_user.user_id).await?;
    Ok(Json(progress))
}

pub async fn get_all_levels(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<UserLevelResponse>>> {
    let levels = progress_service::get_all_levels(&pool, auth_user.user_id).await?;
    Ok(Json(levels))
}

pub async fn advance_day(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>> {
    progress_service::advance_day(&pool, auth_user.user_id).await?;
    Ok(Json(serde_json::json!({
        "message": "Day advanced successfully"
    })))
}
