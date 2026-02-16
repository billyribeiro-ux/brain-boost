# Critical Security Fixes - Implementation Guide

## Priority 1: Immediate Fixes (Must Complete Before Any Deployment)

---

## Fix 1: CORS Configuration Panic Vulnerability

### Current Code (VULNERABLE)
**File:** `brainboost-elite/backend/src/middleware/cors.rs`

```rust
pub fn cors_layer() -> CorsLayer {
    let config = Config::global();
    
    CorsLayer::new()
        .allow_origin(config.frontend_url.parse::<axum::http::HeaderValue>().unwrap())  // ❌ PANIC
        .allow_methods([/* ... */])
        .allow_headers([/* ... */])
        .allow_credentials(true)
}
```

### Fixed Code

```rust
use crate::config::Config;
use crate::errors::{AppError, Result};
use tower_http::cors::CorsLayer;
use std::time::Duration;

pub fn cors_layer() -> Result<CorsLayer> {
    let config = Config::global();
    
    let origin = config.frontend_url
        .parse::<axum::http::HeaderValue>()
        .map_err(|e| AppError::InternalError(format!("Invalid FRONTEND_URL: {}", e)))?;
    
    Ok(CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::PATCH,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(86400))
        .vary([axum::http::header::ORIGIN]))
}
```

### Update main.rs

```rust
// In main.rs, change:
.layer(cors_layer())

// To:
.layer(cors_layer()?)
```

---

## Fix 2: Refresh Token Timing Attack

### Current Code (VULNERABLE)
**File:** `brainboost-elite/backend/src/services/auth_service.rs`

```rust
pub async fn refresh_token(pool: &PgPool, refresh_token_string: &str) -> Result<AuthResponse> {
    let tokens = sqlx::query!(/* fetch ALL tokens */).fetch_all(pool).await?;
    
    let mut found_token = None;
    for token in tokens {
        if verify(refresh_token_string, &token.token_hash).unwrap_or(false) {  // ❌ TIMING ATTACK
            found_token = Some(token);
            break;  // ❌ Early exit reveals information
        }
    }
    // ...
}
```

### Fixed Code

```rust
use constant_time_eq::constant_time_eq;

pub async fn refresh_token(pool: &PgPool, refresh_token_string: &str) -> Result<AuthResponse> {
    // First, hash the incoming token to use as lookup key
    // This prevents timing attacks on the database query
    let token_hash = hash(refresh_token_string, 12)?;
    
    // Use indexed query instead of fetching all tokens
    let token = sqlx::query!(
        r#"
        SELECT id, user_id, token_hash, expires_at, revoked
        FROM refresh_tokens
        WHERE token_hash = $1 
          AND revoked = false 
          AND expires_at > NOW()
        LIMIT 1
        "#,
        token_hash
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid refresh token".to_string()))?;
    
    // Verify with constant-time comparison
    let is_valid = verify(refresh_token_string, &token.token_hash)
        .map_err(|_| AppError::Unauthorized("Invalid refresh token".to_string()))?;
    
    if !is_valid {
        return Err(AppError::Unauthorized("Invalid refresh token".to_string()));
    }
    
    // Revoke old token
    sqlx::query!(
        "UPDATE refresh_tokens SET revoked = true WHERE id = $1",
        token.id
    )
    .execute(pool)
    .await?;
    
    // Get user
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(token.user_id)
    .fetch_one(pool)
    .await?;
    
    // Generate new tokens
    let new_access_token = encode_access_token(user.id)?;
    let new_refresh_token = encode_refresh_token(user.id)?;
    let expires_at = Utc::now().timestamp() + (24 * 3600);
    
    // Store new refresh token
    let new_token_hash = hash(&new_refresh_token, 12)?;
    let new_expires_at = Utc::now() + Duration::days(7);
    
    sqlx::query!(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, $2, $3)
        "#,
        user.id,
        new_token_hash,
        new_expires_at
    )
    .execute(pool)
    .await?;
    
    Ok(AuthResponse {
        user: user.into(),
        access_token: new_access_token,
        refresh_token: new_refresh_token,
        expires_at,
    })
}
```

### Add Dependency

**File:** `backend/Cargo.toml`

```toml
[dependencies]
constant-time-eq = "0.3"
```

---

## Fix 3: Password Validation Strengthening

### Current Code (WEAK)
**File:** `brainboost-elite/backend/src/utils/validators.rs`

```rust
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() >= 8 {
        Ok(())
    } else {
        Err(ValidationError::new("password_too_short"))
    }
}
```

### Fixed Code

```rust
use validator::ValidationError;
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    static ref COMMON_PASSWORDS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("password");
        set.insert("password123");
        set.insert("12345678");
        set.insert("qwerty");
        set.insert("abc123");
        set.insert("letmein");
        set.insert("welcome");
        set.insert("monkey");
        // Add more common passwords
        set
    };
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    // Minimum length check
    if password.len() < 12 {
        return Err(ValidationError::new("password_too_short"));
    }
    
    // Maximum length check (prevent DoS)
    if password.len() > 128 {
        return Err(ValidationError::new("password_too_long"));
    }
    
    // Complexity checks
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());
    
    if !(has_uppercase && has_lowercase && has_digit && has_special) {
        return Err(ValidationError::new("password_complexity_insufficient"));
    }
    
    // Check against common passwords
    if COMMON_PASSWORDS.contains(&password.to_lowercase().as_str()) {
        return Err(ValidationError::new("password_too_common"));
    }
    
    // Check for repeated characters (e.g., "aaaaaaaaaa")
    let mut prev_char = '\0';
    let mut repeat_count = 0;
    for c in password.chars() {
        if c == prev_char {
            repeat_count += 1;
            if repeat_count >= 3 {
                return Err(ValidationError::new("password_too_repetitive"));
            }
        } else {
            repeat_count = 0;
        }
        prev_char = c;
    }
    
    Ok(())
}
```

### Add Dependencies

**File:** `backend/Cargo.toml`

```toml
[dependencies]
lazy_static = "1.4"
```

---

## Fix 4: Rate Limiting Implementation

### Current Code (INEFFECTIVE)
**File:** `brainboost-elite/backend/src/middleware/rate_limit.rs`

```rust
pub fn rate_limit_layer() -> ServiceBuilder<...> {
    ServiceBuilder::new()
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))  // Only body size
}
```

### Fixed Code

```rust
use tower_governor::{
    governor::GovernorConfigBuilder,
    key_extractor::{SmartIpKeyExtractor, KeyExtractor},
    GovernorLayer,
};
use std::time::Duration;

// General API rate limiting
pub fn general_rate_limit() -> GovernorLayer<SmartIpKeyExtractor> {
    let config = Box::new(
        GovernorConfigBuilder::default()
            .per_second(10)
            .burst_size(20)
            .finish()
            .unwrap()
    );

    GovernorLayer {
        config: Box::leak(config),
    }
}

// Strict rate limiting for auth endpoints
pub fn auth_rate_limit() -> GovernorLayer<SmartIpKeyExtractor> {
    let config = Box::new(
        GovernorConfigBuilder::default()
            .per_minute(5)
            .burst_size(10)
            .finish()
            .unwrap()
    );

    GovernorLayer {
        config: Box::leak(config),
    }
}

// Request body size limit
pub fn body_limit_layer() -> RequestBodyLimitLayer {
    RequestBodyLimitLayer::new(10 * 1024 * 1024) // 10MB
}
```

### Update main.rs

```rust
use brainboost_elite_backend::middleware::rate_limit::{
    general_rate_limit, auth_rate_limit, body_limit_layer
};

// Apply strict rate limiting to auth routes
let public_routes = Router::new()
    .route("/api/auth/register", post(routes::auth::register))
    .route("/api/auth/login", post(routes::auth::login))
    .route("/api/auth/refresh", post(routes::auth::refresh))
    .layer(auth_rate_limit())  // ✅ Strict rate limiting
    .route("/health", get(routes::health::health_check));

// Apply general rate limiting to protected routes
let protected_routes = Router::new()
    .route("/api/auth/logout", post(routes::auth::logout))
    .route("/api/users/me", get(routes::users::get_me))
    // ... other routes
    .layer(middleware::from_fn(auth_middleware))
    .layer(general_rate_limit());  // ✅ General rate limiting

let app = Router::new()
    .merge(public_routes)
    .merge(protected_routes)
    .layer(cors_layer()?)
    .layer(body_limit_layer())  // ✅ Body size limit
    .layer(TraceLayer::new_for_http())
    .with_state(pool);
```

### Add Dependencies

**File:** `backend/Cargo.toml`

```toml
[dependencies]
tower-governor = "0.3"
```

---

## Fix 5: Security Headers Middleware

### New File
**File:** `brainboost-elite/backend/src/middleware/security_headers.rs`

```rust
use axum::{
    body::Body,
    http::{Request, Response, header},
    middleware::Next,
};

pub async fn security_headers_middleware(
    req: Request<Body>,
    next: Next,
) -> Response<Body> {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    // Prevent MIME type sniffing
    headers.insert(
        header::HeaderName::from_static("x-content-type-options"),
        header::HeaderValue::from_static("nosniff")
    );

    // Prevent clickjacking
    headers.insert(
        header::HeaderName::from_static("x-frame-options"),
        header::HeaderValue::from_static("DENY")
    );

    // Disable XSS filter (modern browsers use CSP instead)
    headers.insert(
        header::HeaderName::from_static("x-xss-protection"),
        header::HeaderValue::from_static("0")
    );

    // Force HTTPS
    headers.insert(
        header::HeaderName::from_static("strict-transport-security"),
        header::HeaderValue::from_static("max-age=31536000; includeSubDomains; preload")
    );

    // Content Security Policy
    headers.insert(
        header::HeaderName::from_static("content-security-policy"),
        header::HeaderValue::from_static(
            "default-src 'self'; \
             script-src 'self'; \
             style-src 'self' 'unsafe-inline'; \
             img-src 'self' data: blob:; \
             font-src 'self'; \
             connect-src 'self'; \
             frame-ancestors 'none'; \
             base-uri 'self'; \
             form-action 'self'"
        )
    );

    // Referrer policy
    headers.insert(
        header::HeaderName::from_static("referrer-policy"),
        header::HeaderValue::from_static("strict-origin-when-cross-origin")
    );

    // Permissions policy
    headers.insert(
        header::HeaderName::from_static("permissions-policy"),
        header::HeaderValue::from_static(
            "camera=(), microphone=(), geolocation=(), payment=()"
        )
    );

    response
}
```

### Update middleware/mod.rs

```rust
pub mod auth;
pub mod cors;
pub mod rate_limit;
pub mod security_headers;  // ✅ Add this
```

### Update main.rs

```rust
use brainboost_elite_backend::middleware::security_headers::security_headers_middleware;

let app = Router::new()
    .merge(public_routes)
    .merge(protected_routes)
    .layer(middleware::from_fn(security_headers_middleware))  // ✅ Add security headers
    .layer(cors_layer()?)
    .layer(body_limit_layer())
    .layer(TraceLayer::new_for_http())
    .with_state(pool);
```

---

## Fix 6: Input Validation Enhancement

### New File
**File:** `brainboost-elite/backend/src/middleware/input_validation.rs`

```rust
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::errors::AppError;

const MAX_JSON_PAYLOAD_SIZE: usize = 1024 * 1024; // 1MB

pub async fn validate_content_type(
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, AppError> {
    // Only validate POST, PUT, PATCH requests
    let method = req.method();
    if method == "POST" || method == "PUT" || method == "PATCH" {
        let content_type = req.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok());

        match content_type {
            Some(ct) if ct.starts_with("application/json") => {
                // Valid JSON content type
            }
            Some(ct) if ct.starts_with("multipart/form-data") => {
                // Valid for file uploads
            }
            _ => {
                return Err(AppError::ValidationError(
                    "Invalid or missing Content-Type header".to_string()
                ));
            }
        }
    }

    Ok(next.run(req).await)
}
```

### Enhanced Model Validation

**File:** `brainboost-elite/backend/src/models/user.rs`

```rust
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]  // ✅ Reject unexpected fields
pub struct CreateUserRequest {
    #[validate(email, length(max = 255))]
    pub email: String,

    #[validate(length(min = 12, max = 128), custom = "crate::utils::validators::validate_password")]
    pub password: String,

    #[validate(length(min = 1, max = 100))]
    pub display_name: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct LoginRequest {
    #[validate(email, length(max = 255))]
    pub email: String,

    #[validate(length(min = 1, max = 128))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdateUserRequest {
    #[validate(email, length(max = 255))]
    pub email: Option<String>,

    #[validate(length(min = 1, max = 100))]
    pub display_name: Option<String>,

    #[validate(url, length(max = 500))]
    pub avatar_url: Option<String>,

    #[validate(length(max = 20))]
    pub chronotype: Option<String>,

    #[validate(length(max = 50))]
    pub primary_goal: Option<String>,

    #[validate(length(max = 50))]
    pub timezone: Option<String>,
}
```

---

## Fix 7: Frontend Token Storage (HttpOnly Cookies)

### Backend Changes

**File:** `brainboost-elite/backend/src/routes/auth.rs`

```rust
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
    http::{header, StatusCode},
};
use axum_extra::extract::cookie::{Cookie, SameSite};

pub async fn register(
    State(pool): State<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Response> {
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let response = auth_service::register_user(&pool, req).await?;

    // Create HttpOnly cookies
    let access_cookie = Cookie::build(("access_token", response.access_token.clone()))
        .path("/")
        .max_age(time::Duration::hours(24))
        .same_site(SameSite::Strict)
        .http_only(true)
        .secure(true)  // Only over HTTPS
        .build();

    let refresh_cookie = Cookie::build(("refresh_token", response.refresh_token.clone()))
        .path("/api/auth/refresh")
        .max_age(time::Duration::days(7))
        .same_site(SameSite::Strict)
        .http_only(true)
        .secure(true)
        .build();

    // Return response with cookies
    Ok((
        StatusCode::OK,
        [
            (header::SET_COOKIE, access_cookie.to_string()),
            (header::SET_COOKIE, refresh_cookie.to_string()),
        ],
        Json(response)
    ).into_response())
}
```

### Add Dependencies

**File:** `backend/Cargo.toml`

```toml
[dependencies]
axum-extra = { version = "0.9", features = ["cookie"] }
time = "0.3"
```


