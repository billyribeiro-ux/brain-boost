use axum::{
    middleware,
    routing::{get, post, patch, put},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use brainboost_elite_backend::{
    config::Config,
    db::create_pool,
    middleware::{auth::auth_middleware, cors::cors_layer},
    routes,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Config::init()?;
    let config = Config::global();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.rust_log.clone().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting BrainBoost Elite Backend");

    let pool = create_pool(&config.database_url).await?;

    tracing::info!("Running database migrations");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    let public_routes = Router::new()
        .route("/api/auth/register", post(routes::auth::register))
        .route("/api/auth/login", post(routes::auth::login))
        .route("/api/auth/refresh", post(routes::auth::refresh))
        .route("/health", get(routes::health::health_check));

    let protected_routes = Router::new()
        .route("/api/auth/logout", post(routes::auth::logout))
        .route("/api/users/me", get(routes::users::get_me))
        .route("/api/users/me", patch(routes::users::update_me))
        .route("/api/users/me/schedule", get(routes::users::get_schedule))
        .route("/api/users/me/schedule", put(routes::users::update_schedule))
        .route("/api/progress/current", get(routes::progress::get_current_progress))
        .route("/api/progress/levels", get(routes::progress::get_all_levels))
        .route("/api/progress/advance-day", post(routes::progress::advance_day))
        .route("/api/exercises/today", get(routes::exercises::get_today_exercises))
        .route("/api/exercises/:session_id/complete", post(routes::exercises::complete_exercise))
        .route("/api/exercises/history", get(routes::exercises::get_exercise_history))
        .route("/api/metrics/today", get(routes::metrics::get_today_metrics))
        .route("/api/metrics/trend", get(routes::metrics::get_metrics_trend))
        .route("/api/metrics/brain-score", get(routes::metrics::get_brain_score))
        .route("/api/trading/start", post(routes::trading::start_trading))
        .route("/api/trading/decide", post(routes::trading::make_decision))
        .route("/api/trading/history", get(routes::trading::get_trading_history))
        .route("/api/trading/stats", get(routes::trading::get_trading_stats))
        .route("/api/coach/message", post(routes::coach::send_message))
        .route("/api/coach/suggestions", get(routes::coach::get_suggestions))
        .route("/api/coach/history", get(routes::coach::get_history))
        .route("/api/learning/topics", post(routes::learning::create_topic))
        .route("/api/learning/topics", get(routes::learning::get_topics))
        .route("/api/learning/next-lesson", get(routes::learning::get_next_lesson))
        .route("/api/learning/lessons/:lesson_id/complete", post(routes::learning::complete_lesson))
        .route("/api/learning/topics/:topic_id/prediction", get(routes::learning::get_mastery_prediction))
        .route("/api/learning/topics/:topic_id", axum::routing::delete(routes::learning::deactivate_topic))
        .layer(middleware::from_fn(auth_middleware));

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(cors_layer())
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    tracing::info!("Shutdown signal received");
}
