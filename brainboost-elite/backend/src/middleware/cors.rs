use crate::config::Config;
use tower_http::cors::{Any, CorsLayer};

pub fn cors_layer() -> CorsLayer {
    let config = Config::global();
    
    CorsLayer::new()
        .allow_origin(config.frontend_url.parse::<axum::http::HeaderValue>().unwrap())
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
        ])
        .allow_credentials(true)
}
