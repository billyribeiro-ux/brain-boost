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
        .route("/auth/register", post(routes::auth::register))
        .route("/auth/login", post(routes::auth::login))
        .route("/auth/refresh", post(routes::auth::refresh))
        .route("/health", get(routes::health::health_check));

    let protected_routes = Router::new()
        .route("/auth/logout", post(routes::auth::logout))
        .route("/users/me", get(routes::users::get_me))
        .route("/users/me", patch(routes::users::update_me))
        .route("/users/me/schedule", get(routes::users::get_schedule))
        .route("/users/me/schedule", put(routes::users::update_schedule))
        .route("/progress/current", get(routes::progress::get_current_progress))
        .route("/progress/levels", get(routes::progress::get_all_levels))
        .route("/progress/advance-day", post(routes::progress::advance_day))
        .route("/exercises/today", get(routes::exercises::get_today_exercises))
        .route("/exercises/complete", post(routes::exercises::complete_exercise))
        .route("/exercises/history", get(routes::exercises::get_exercise_history))
        .route("/metrics/today", get(routes::metrics::get_today_metrics))
        .route("/metrics/trend", get(routes::metrics::get_metrics_trend))
        .route("/metrics/brain-score", get(routes::metrics::get_brain_score))
        .route("/trading/start", post(routes::trading::start_trading))
        .route("/trading/decide", post(routes::trading::make_decision))
        .route("/trading/history", get(routes::trading::get_trading_history))
        .route("/trading/stats", get(routes::trading::get_trading_stats))
        .route("/coach/message", post(routes::coach::send_message))
        .route("/coach/suggestions", get(routes::coach::get_suggestions))
        .route("/coach/history", get(routes::coach::get_history))
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
