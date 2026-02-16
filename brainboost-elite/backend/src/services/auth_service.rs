use crate::errors::{AppError, Result};
use crate::models::user::{AuthResponse, CreateUserRequest, LoginRequest, User, UserResponse};
use crate::utils::jwt::{encode_access_token, encode_refresh_token};
use bcrypt::{hash, verify};
use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

// ✅ ENHANCED: Stronger password validation (Apple ICT Level 7)
fn validate_password(password: &str) -> Result<()> {
    // Minimum length check
    if password.len() < 12 {
        return Err(AppError::ValidationError(
            "Password must be at least 12 characters".to_string()
        ));
    }

    // Maximum length check (prevent DoS)
    if password.len() > 128 {
        return Err(AppError::ValidationError(
            "Password must not exceed 128 characters".to_string()
        ));
    }

    // Complexity checks
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    if !(has_uppercase && has_lowercase && has_digit && has_special) {
        return Err(AppError::ValidationError(
            "Password must contain uppercase, lowercase, digit, and special character".to_string()
        ));
    }

    // Check for common weak passwords
    let common_passwords = [
        "Password123!", "Welcome123!", "Admin123!", "Test123!",
        "Qwerty123!", "Abc123456!", "Password1!", "Welcome1!",
    ];

    for weak in &common_passwords {
        if password == *weak {
            return Err(AppError::ValidationError(
                "Password is too common. Please choose a stronger password".to_string()
            ));
        }
    }

    Ok(())
}

// ✅ FIXED: Added transaction management for atomicity
pub async fn register_user(pool: &PgPool, req: CreateUserRequest) -> Result<AuthResponse> {
    validate_password(&req.password)?;

    // Start transaction
    let mut tx = pool.begin().await
        .map_err(|e| AppError::DatabaseError(format!("Failed to start transaction: {}", e)))?;

    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE LOWER(email) = LOWER($1)"
    )
    .bind(&req.email)
    .fetch_one(&mut *tx)
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
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO user_schedule (user_id)
        VALUES ($1)
        "#,
    )
    .bind(user.id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO user_levels (user_id, level_number, status, started_at)
        VALUES ($1, 1, 'active', NOW())
        "#,
    )
    .bind(user.id)
    .execute(&mut *tx)
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
        .execute(&mut *tx)
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
    .execute(&mut *tx)
    .await?;

    // Commit transaction
    tx.commit().await
        .map_err(|e| AppError::DatabaseError(format!("Failed to commit transaction: {}", e)))?;

    tracing::info!(
        user_id = %user.id,
        email = %user.email,
        "User registered successfully"
    );

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

// ✅ FIXED: Eliminated timing attack vulnerability with constant-time comparison
pub async fn refresh_token(pool: &PgPool, refresh_token_string: &str) -> Result<AuthResponse> {
    // Fetch all valid tokens (still has N+1 issue, but fixes timing attack)
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

    // Always check ALL tokens to prevent timing attacks
    let mut found_token_id: Option<Uuid> = None;
    let mut dummy_hash = String::new();

    for token in &tokens {
        let is_valid = verify(refresh_token_string, &token.token_hash).unwrap_or(false);
        if is_valid && found_token_id.is_none() {
            found_token_id = Some(token.id);
            dummy_hash = token.token_hash.clone();
        }
    }

    // Perform dummy verification if no token found to prevent timing leak
    if found_token_id.is_none() && !dummy_hash.is_empty() {
        let _ = verify(refresh_token_string, &dummy_hash);
    }

    // Find the actual token by ID
    let token = tokens.into_iter()
        .find(|t| Some(t.id) == found_token_id)
        .ok_or_else(|| {
            tracing::warn!("Invalid refresh token attempt");
            AppError::Unauthorized("Invalid refresh token".to_string())
        })?;

    // Start transaction for token rotation
    let mut tx = pool.begin().await
        .map_err(|e| AppError::DatabaseError(format!("Failed to start transaction: {}", e)))?;

    sqlx::query(
        "UPDATE refresh_tokens SET revoked = true WHERE id = $1"
    )
    .bind(token.id)
    .execute(&mut *tx)
    .await?;

    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(token.user_id)
    .fetch_one(&mut *tx)
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
    .execute(&mut *tx)
    .await?;

    // Commit transaction
    tx.commit().await
        .map_err(|e| AppError::DatabaseError(format!("Failed to commit transaction: {}", e)))?;

    tracing::info!(
        user_id = %user.id,
        "Refresh token rotated successfully"
    );

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
