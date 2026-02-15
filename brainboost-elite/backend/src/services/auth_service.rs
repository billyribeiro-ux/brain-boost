use crate::errors::{AppError, Result};
use crate::models::user::{AuthResponse, CreateUserRequest, LoginRequest, User, UserResponse};
use crate::utils::jwt::{encode_access_token, encode_refresh_token};
use bcrypt::{hash, verify};
use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

fn validate_password(password: &str) -> Result<()> {
    if password.len() < 8 {
        return Err(AppError::ValidationError("Password must be at least 8 characters".to_string()));
    }
    
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    
    if !has_uppercase || !has_lowercase || !has_digit {
        return Err(AppError::ValidationError(
            "Password must contain uppercase, lowercase, and digit".to_string()
        ));
    }
    
    Ok(())
}

pub async fn register_user(pool: &PgPool, req: CreateUserRequest) -> Result<AuthResponse> {
    validate_password(&req.password)?;
    
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE email = $1"
    )
    .bind(&req.email)
    .fetch_one(pool)
    .await?;
    
    if existing > 0 {
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    let password_hash = hash(&req.password, 12)?;

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
        INSERT INTO user_levels (user_id, level_number, status, started_at)
        VALUES ($1, 1, 'active', NOW())
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
    
    let token_hash = hash(&refresh_token, 12)?;
    let expires_at_dt = Utc::now() + Duration::days(7);
    
    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user.id)
    .bind(&token_hash)
    .bind(expires_at_dt)
    .execute(pool)
    .await?;

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
        SELECT * FROM users WHERE email = $1
        "#,
    )
    .bind(&req.email)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;
    
    if user.status != "active" {
        return Err(AppError::Unauthorized("Account is not active".to_string()));
    }

    let valid = verify(&req.password, &user.password_hash)?;
    if !valid {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    let access_token = encode_access_token(user.id)?;
    let refresh_token = encode_refresh_token(user.id)?;
    let expires_at = Utc::now().timestamp() + (24 * 3600);
    
    let token_hash = hash(&refresh_token, 12)?;
    let expires_at_dt = Utc::now() + Duration::days(7);
    
    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user.id)
    .bind(&token_hash)
    .bind(expires_at_dt)
    .execute(pool)
    .await?;

    Ok(AuthResponse {
        user: user.into(),
        access_token,
        refresh_token,
        expires_at,
    })
}

pub async fn refresh_token(pool: &PgPool, refresh_token_string: &str) -> Result<AuthResponse> {
    let tokens = sqlx::query!(
        r#"
        SELECT id, user_id, token_hash, expires_at, revoked
        FROM refresh_tokens
        WHERE revoked = false AND expires_at > NOW()
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;
    
    let mut found_token = None;
    for token in tokens {
        if verify(refresh_token_string, &token.token_hash).unwrap_or(false) {
            found_token = Some(token);
            break;
        }
    }
    
    let token = found_token.ok_or_else(|| AppError::Unauthorized("Invalid refresh token".to_string()))?;
    
    sqlx::query(
        "UPDATE refresh_tokens SET revoked = true WHERE id = $1"
    )
    .bind(token.id)
    .execute(pool)
    .await?;
    
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(token.user_id)
    .fetch_one(pool)
    .await?;
    
    let new_access_token = encode_access_token(user.id)?;
    let new_refresh_token = encode_refresh_token(user.id)?;
    let expires_at = Utc::now().timestamp() + (24 * 3600);
    
    let new_token_hash = hash(&new_refresh_token, 12)?;
    let new_expires_at = Utc::now() + Duration::days(7);
    
    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user.id)
    .bind(&new_token_hash)
    .bind(new_expires_at)
    .execute(pool)
    .await?;
    
    Ok(AuthResponse {
        user: user.into(),
        access_token: new_access_token,
        refresh_token: new_refresh_token,
        expires_at,
    })
}

pub async fn logout_user(pool: &PgPool, user_id: Uuid) -> Result<()> {
    sqlx::query(
        "UPDATE refresh_tokens SET revoked = true WHERE user_id = $1"
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    
    Ok(())
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
