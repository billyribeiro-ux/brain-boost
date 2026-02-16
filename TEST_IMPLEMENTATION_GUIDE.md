# BrainBoost Elite - Comprehensive Test Implementation Guide

## Overview

This guide provides detailed implementation instructions for establishing comprehensive test coverage meeting Apple ICT Level 7 standards.

---

## 1. Backend Test Infrastructure Setup

### 1.1 Test Database Configuration

**File:** `backend/tests/helpers/test_db.rs`

```rust
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Once;

static INIT: Once = Once::new();

pub async fn setup_test_db() -> PgPool {
    INIT.call_once(|| {
        dotenvy::from_filename(".env.test").ok();
    });
    
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    pool
}

pub async fn cleanup_test_db(pool: &PgPool) {
    // Clean all tables in reverse dependency order
    sqlx::query("TRUNCATE TABLE exercise_completions CASCADE")
        .execute(pool).await.unwrap();
    sqlx::query("TRUNCATE TABLE daily_sessions CASCADE")
        .execute(pool).await.unwrap();
    sqlx::query("TRUNCATE TABLE daily_metrics CASCADE")
        .execute(pool).await.unwrap();
    sqlx::query("TRUNCATE TABLE refresh_tokens CASCADE")
        .execute(pool).await.unwrap();
    sqlx::query("TRUNCATE TABLE user_levels CASCADE")
        .execute(pool).await.unwrap();
    sqlx::query("TRUNCATE TABLE user_schedule CASCADE")
        .execute(pool).await.unwrap();
    sqlx::query("TRUNCATE TABLE users CASCADE")
        .execute(pool).await.unwrap();
}
```

### 1.2 Test Fixtures

**File:** `backend/tests/helpers/test_fixtures.rs`

```rust
use uuid::Uuid;
use chrono::Utc;
use crate::models::user::{User, CreateUserRequest};
use crate::services::auth_service;
use sqlx::PgPool;

pub struct TestUser {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn create_test_user(pool: &PgPool, email: &str) -> TestUser {
    let password = "TestPass123!";
    let request = CreateUserRequest {
        email: email.to_string(),
        password: password.to_string(),
        display_name: "Test User".to_string(),
    };
    
    let response = auth_service::register_user(pool, request)
        .await
        .expect("Failed to create test user");
    
    TestUser {
        id: response.user.id,
        email: email.to_string(),
        password: password.to_string(),
        access_token: response.access_token,
        refresh_token: response.refresh_token,
    }
}

pub async fn create_test_session(
    pool: &PgPool,
    user_id: Uuid,
    level: i16,
    week: i16,
    day: i16,
) -> Uuid {
    let session_id = sqlx::query_scalar!(
        r#"
        INSERT INTO daily_sessions (
            user_id, level_number, week_number, day_number,
            day_date, slot, status, duration_planned_seconds
        )
        VALUES ($1, $2, $3, $4, CURRENT_DATE, 'morning', 'pending', 1800)
        RETURNING id
        "#,
        user_id, level, week, day
    )
    .fetch_one(pool)
    .await
    .expect("Failed to create test session");
    
    session_id
}
```

### 1.3 Update Cargo.toml

**File:** `backend/Cargo.toml`

```toml
[dev-dependencies]
tokio-test = "0.4"
mockall = "0.12"
fake = "2.9"
proptest = "1.4"
criterion = "0.5"
axum-test = "14.0"
```

---

## 2. Unit Tests Implementation

### 2.1 JWT Token Tests

**File:** `backend/tests/unit/jwt_tests.rs`

```rust
#[cfg(test)]
mod jwt_tests {
    use brainboost_elite_backend::utils::jwt::*;
    use brainboost_elite_backend::config::Config;
    use uuid::Uuid;
    use chrono::Utc;
    
    #[test]
    fn test_encode_decode_access_token_success() {
        Config::init().ok();
        let user_id = Uuid::new_v4();
        
        let token = encode_access_token(user_id)
            .expect("Failed to encode token");
        
        let claims = decode_token(&token)
            .expect("Failed to decode token");
        
        assert_eq!(claims.sub, user_id.to_string());
        assert!(claims.exp > Utc::now().timestamp());
        assert!(claims.iat <= Utc::now().timestamp());
    }
    
    #[test]
    fn test_decode_invalid_token_fails() {
        Config::init().ok();
        let invalid_token = "invalid.jwt.token";
        
        let result = decode_token(invalid_token);
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_decode_tampered_token_fails() {
        Config::init().ok();
        let user_id = Uuid::new_v4();
        let token = encode_access_token(user_id).unwrap();
        
        // Tamper with token
        let mut parts: Vec<&str> = token.split('.').collect();
        parts[1] = "tampered_payload";
        let tampered = parts.join(".");
        
        let result = decode_token(&tampered);
        
        assert!(result.is_err());
    }
}
```

### 2.2 Password Validation Tests

**File:** `backend/tests/unit/validation_tests.rs`

```rust
#[cfg(test)]
mod validation_tests {
    use brainboost_elite_backend::utils::validators::*;

    #[test]
    fn test_valid_password_accepted() {
        let result = validate_password("SecurePass123!");
        assert!(result.is_ok());
    }

    #[test]
    fn test_short_password_rejected() {
        let result = validate_password("Short1!");
        assert!(result.is_err());
    }

    #[test]
    fn test_password_without_uppercase_rejected() {
        let result = validate_password("securepass123!");
        assert!(result.is_err());
    }

    #[test]
    fn test_password_without_lowercase_rejected() {
        let result = validate_password("SECUREPASS123!");
        assert!(result.is_err());
    }

    #[test]
    fn test_password_without_digit_rejected() {
        let result = validate_password("SecurePassword!");
        assert!(result.is_err());
    }

    #[test]
    fn test_password_without_special_char_rejected() {
        let result = validate_password("SecurePass123");
        assert!(result.is_err());
    }

    #[test]
    fn test_common_password_rejected() {
        let result = validate_password("Password123!");
        assert!(result.is_err());
    }
}
```

### 2.3 Authentication Service Tests

**File:** `backend/tests/unit/auth_service_tests.rs`

```rust
#[cfg(test)]
mod auth_service_tests {
    use super::*;
    use crate::helpers::{setup_test_db, cleanup_test_db};

    #[tokio::test]
    async fn test_register_user_success() {
        let pool = setup_test_db().await;

        let request = CreateUserRequest {
            email: "newuser@test.com".to_string(),
            password: "SecurePass123!".to_string(),
            display_name: "New User".to_string(),
        };

        let result = auth_service::register_user(&pool, request).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.user.email, "newuser@test.com");
        assert!(!response.access_token.is_empty());
        assert!(!response.refresh_token.is_empty());

        cleanup_test_db(&pool).await;
    }

    #[tokio::test]
    async fn test_register_duplicate_email_fails() {
        let pool = setup_test_db().await;

        let request = CreateUserRequest {
            email: "duplicate@test.com".to_string(),
            password: "SecurePass123!".to_string(),
            display_name: "User 1".to_string(),
        };

        // First registration succeeds
        auth_service::register_user(&pool, request.clone()).await.unwrap();

        // Second registration fails
        let result = auth_service::register_user(&pool, request).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Conflict(_)));

        cleanup_test_db(&pool).await;
    }

    #[tokio::test]
    async fn test_login_with_valid_credentials_success() {
        let pool = setup_test_db().await;
        let test_user = create_test_user(&pool, "login@test.com").await;

        let request = LoginRequest {
            email: test_user.email.clone(),
            password: test_user.password.clone(),
        };

        let result = auth_service::login_user(&pool, request).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.user.email, test_user.email);

        cleanup_test_db(&pool).await;
    }

    #[tokio::test]
    async fn test_login_with_wrong_password_fails() {
        let pool = setup_test_db().await;
        let test_user = create_test_user(&pool, "login@test.com").await;

        let request = LoginRequest {
            email: test_user.email.clone(),
            password: "WrongPassword123!".to_string(),
        };

        let result = auth_service::login_user(&pool, request).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Unauthorized(_)));

        cleanup_test_db(&pool).await;
    }

    #[tokio::test]
    async fn test_refresh_token_success() {
        let pool = setup_test_db().await;
        let test_user = create_test_user(&pool, "refresh@test.com").await;

        let result = auth_service::refresh_token(&pool, &test_user.refresh_token).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.access_token.is_empty());
        assert_ne!(response.access_token, test_user.access_token);

        cleanup_test_db(&pool).await;
    }
}
```

---

## 3. Integration Tests Implementation

### 3.1 API Authentication Tests

**File:** `backend/tests/integration/api_auth_tests.rs`

```rust
use axum_test::TestServer;
use serde_json::json;

#[tokio::test]
async fn test_register_endpoint() {
    let server = create_test_server().await;

    let response = server
        .post("/api/auth/register")
        .json(&json!({
            "email": "newuser@example.com",
            "password": "SecurePass123!",
            "display_name": "New User"
        }))
        .await;

    assert_eq!(response.status(), 200);

    let body: serde_json::Value = response.json();
    assert!(body["access_token"].is_string());
    assert!(body["refresh_token"].is_string());
    assert_eq!(body["user"]["email"], "newuser@example.com");
}

#[tokio::test]
async fn test_register_with_weak_password_fails() {
    let server = create_test_server().await;

    let response = server
        .post("/api/auth/register")
        .json(&json!({
            "email": "user@example.com",
            "password": "weak",
            "display_name": "User"
        }))
        .await;

    assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn test_login_endpoint() {
    let server = create_test_server().await;

    // First register
    server.post("/api/auth/register")
        .json(&json!({
            "email": "login@example.com",
            "password": "SecurePass123!",
            "display_name": "Login User"
        }))
        .await;

    // Then login
    let response = server
        .post("/api/auth/login")
        .json(&json!({
            "email": "login@example.com",
            "password": "SecurePass123!"
        }))
        .await;

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_protected_endpoint_without_token_fails() {
    let server = create_test_server().await;

    let response = server
        .get("/api/users/me")
        .await;

    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn test_protected_endpoint_with_valid_token_succeeds() {
    let server = create_test_server().await;

    // Register and get token
    let auth_response = server
        .post("/api/auth/register")
        .json(&json!({
            "email": "protected@example.com",
            "password": "SecurePass123!",
            "display_name": "Protected User"
        }))
        .await;

    let auth_body: serde_json::Value = auth_response.json();
    let token = auth_body["access_token"].as_str().unwrap();

    // Access protected endpoint
    let response = server
        .get("/api/users/me")
        .add_header("Authorization", format!("Bearer {}", token))
        .await;

    assert_eq!(response.status(), 200);
}
```


