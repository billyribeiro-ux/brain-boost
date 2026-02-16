// Integration tests for BrainBoost Elite Backend
// Apple ICT Level 7 Compliance - Security & Authorization Tests

use brainboost_elite_backend::services::{exercise_service, hyper_learning_service};
use brainboost_elite_backend::models::exercise::CompleteExerciseRequest;
use sqlx::PgPool;
use uuid::Uuid;

// Helper function to create test database pool
async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5433/brainboost_elite".to_string());

    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

// Test: IDOR protection on lesson completion
#[tokio::test]
#[ignore] // Requires database cleanup between runs
async fn test_idor_protection_lesson_completion() {
    let pool = setup_test_db().await;

    // Create two test users
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();

    // Create a topic for user1
    let topic = hyper_learning_service::create_learning_topic(
        &pool,
        user1_id,
        "Rust Programming".to_string(),
        Some("Test Description".to_string()),
    ).await.expect("Failed to create topic");

    // Get the first lesson
    let lessons = sqlx::query!("SELECT id FROM micro_lessons WHERE topic_id = $1 LIMIT 1", topic.id)
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch lesson");

    // Try to complete user1's lesson as user2 (should fail)
    let result = hyper_learning_service::complete_lesson(
        &pool,
        user2_id,  // Different user!
        lessons.id,
        85.0,
    ).await;

    assert!(result.is_err(), "Should prevent IDOR attack on lesson completion");
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("access denied") || error_msg.contains("not found"), "Should return access denied error");

    pool.close().await;
}

// Test: IDOR protection on exercise completion
#[tokio::test]
#[ignore] // Requires database cleanup between runs
async fn test_idor_protection_exercise_completion() {
    let pool = setup_test_db().await;

    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    
    // Create a session for user1
    let session_id: Uuid = sqlx::query_scalar(
        "INSERT INTO daily_sessions (user_id, level_number, week_number, day_number, slot, session_date, status)
         VALUES ($1, 1, 1, 1, 'morning', CURRENT_DATE, 'pending') RETURNING id"
    )
    .bind(user1_id)
    .fetch_one(&pool)
    .await
    .expect("Failed to create session");
    
    // Try to complete user1's session as user2 (should fail)
    let req = CompleteExerciseRequest {
        session_id,
        exercise_type: "meditation".to_string(),
        exercise_name: "Test Exercise".to_string(),
        duration_seconds: 300,
        accuracy_score: Some(85.0),
        focus_rating: Some(8),
        difficulty_level: Some(5),
        metadata: None,
    };

    let result = exercise_service::complete_exercise(&pool, user2_id, req).await;

    assert!(result.is_err(), "Should prevent IDOR attack on exercise completion");
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("access denied") || error_msg.contains("not found"), "Should return access denied error");

    pool.close().await;
}

// Test: IDOR protection on mastery prediction
#[tokio::test]
#[ignore] // Requires database cleanup between runs
async fn test_idor_protection_mastery_prediction() {
    let pool = setup_test_db().await;

    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();

    // Create a topic for user1
    let topic = hyper_learning_service::create_learning_topic(
        &pool,
        user1_id,
        "Rust Programming".to_string(),
        Some("Test Description".to_string()),
    ).await.expect("Failed to create topic");

    // Try to access user1's topic prediction as user2 (should fail)
    let result = hyper_learning_service::get_mastery_prediction(
        &pool,
        user2_id,  // Different user!
        topic.id,
    ).await;

    assert!(result.is_err(), "Should prevent IDOR attack on mastery prediction");
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("access denied") || error_msg.contains("not found"), "Should return access denied error");

    pool.close().await;
}

