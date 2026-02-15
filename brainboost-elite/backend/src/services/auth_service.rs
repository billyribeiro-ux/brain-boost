use crate::errors::{AppError, Result};
use crate::models::user::{AuthResponse, CreateUserRequest, LoginRequest, User, UserResponse};
use crate::utils::jwt::{encode_access_token, encode_refresh_token};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn register_user(pool: &PgPool, req: CreateUserRequest) -> Result<AuthResponse> {
    let password_hash = hash(&req.password, DEFAULT_COST)?;

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, password_hash, display_name)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&req.email)
    .bind(&password_hash)
    .bind(&req.display_name)
    .fetch_one(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO user_schedule (user_id)
        VALUES ($1)
        "#,
    )
    .bind(user.id)
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO user_levels (user_id, level_number, status)
        VALUES ($1, 1, 'active')
        "#,
    )
    .bind(user.id)
    .execute(pool)
    .await?;

    for level in 2..=6 {
        sqlx::query(
            r#"
            INSERT INTO user_levels (user_id, level_number, status)
            VALUES ($1, $2, 'locked')
            "#,
        )
        .bind(user.id)
        .bind(level)
        .execute(pool)
        .await?;
    }

    let access_token = encode_access_token(user.id)?;
    let refresh_token = encode_refresh_token(user.id)?;
    let expires_at = Utc::now().timestamp() + (24 * 3600);

    Ok(AuthResponse {
        user: user.into(),
        access_token,
        refresh_token,
        expires_at,
    })
}

pub async fn login_user(pool: &PgPool, req: LoginRequest) -> Result<AuthResponse> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE email = $1 AND status = 'active'
        "#,
    )
    .bind(&req.email)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    let valid = verify(&req.password, &user.password_hash)?;
    if !valid {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    let access_token = encode_access_token(user.id)?;
    let refresh_token = encode_refresh_token(user.id)?;
    let expires_at = Utc::now().timestamp() + (24 * 3600);

    Ok(AuthResponse {
        user: user.into(),
        access_token,
        refresh_token,
        expires_at,
    })
}

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<UserResponse> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(user.into())
}
