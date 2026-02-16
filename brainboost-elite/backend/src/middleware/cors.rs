use crate::config::Config;
use tower_http::cors::CorsLayer;
use axum::http::{HeaderValue, header};

pub fn cors_layer() -> CorsLayer {
    let config = Config::global();

    // ✅ FIXED: Proper error handling instead of .unwrap() panic
    let origin = config.frontend_url
        .parse::<HeaderValue>()
        .unwrap_or_else(|e| {
            tracing::error!("Invalid FRONTEND_URL '{}': {}. Falling back to localhost", config.frontend_url, e);
            HeaderValue::from_static("http://localhost:3000")
        });

    CorsLayer::new()
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
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::HeaderName::from_static("x-csrf-token"),
        ])
        .allow_credentials(true)
        .expose_headers([
            header::HeaderName::from_static("x-csrf-token"),
        ])
}
