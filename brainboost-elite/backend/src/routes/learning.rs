use crate::errors::Result;
use crate::middleware::auth::AuthUser;
use crate::services::hyper_learning_service;
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTopicRequest {
    pub topic_name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct CompleteLessonRequest {
    pub score: f64,
}

#[derive(Serialize)]
pub struct TopicResponse {
    pub id: Uuid,
    pub topic_name: String,
    pub description: Option<String>,
    pub mastery_percentage: f64,
    pub estimated_mastery_days: i32,
    pub lesson_count: usize,
    pub completed_lessons: usize,
}

pub async fn create_topic(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CreateTopicRequest>,
) -> Result<Json<hyper_learning_service::LearningTopic>> {
    let topic = hyper_learning_service::create_learning_topic(
        &pool,
        auth_user.user_id,
        req.topic_name,
        req.description,
    )
    .await?;
    
    Ok(Json(topic))
}

pub async fn get_topics(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<TopicResponse>>> {
    let topics = hyper_learning_service::get_user_topics(&pool, auth_user.user_id).await?;
    
    let response: Vec<TopicResponse> = topics
        .into_iter()
        .map(|t| {
            let completed = t.lessons.iter().filter(|l| l.completed).count();
            TopicResponse {
                id: t.id,
                topic_name: t.topic_name,
                description: t.description,
                mastery_percentage: t.mastery_percentage,
                estimated_mastery_days: t.estimated_mastery_days,
                lesson_count: t.lessons.len(),
                completed_lessons: completed,
            }
        })
        .collect();
    
    Ok(Json(response))
}

pub async fn get_next_lesson(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Option<hyper_learning_service::MicroLesson>>> {
    let lesson = hyper_learning_service::get_next_lesson(&pool, auth_user.user_id).await?;
    Ok(Json(lesson))
}

pub async fn complete_lesson(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Path(lesson_id): Path<Uuid>,
    Json(req): Json<CompleteLessonRequest>,
) -> Result<Json<hyper_learning_service::MicroLesson>> {
    let lesson = hyper_learning_service::complete_lesson(
        &pool,
        auth_user.user_id,
        lesson_id,
        req.score,
    )
    .await?;
    
    Ok(Json(lesson))
}

pub async fn get_mastery_prediction(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Path(topic_id): Path<Uuid>,
) -> Result<Json<hyper_learning_service::MasteryPrediction>> {
    let prediction = hyper_learning_service::get_mastery_prediction(
        &pool,
        auth_user.user_id,
        topic_id,
    )
    .await?;
    
    Ok(Json(prediction))
}

pub async fn deactivate_topic(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Path(topic_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    sqlx::query(
        "UPDATE learning_topics SET is_active = false WHERE id = $1 AND user_id = $2"
    )
    .bind(topic_id)
    .bind(auth_user.user_id)
    .execute(&pool)
    .await?;
    
    Ok(Json(serde_json::json!({ "message": "Topic deactivated" })))
}
