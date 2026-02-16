use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Flashcard {
    pub id: Uuid,
    pub user_id: Uuid,
    pub topic: String,
    pub question: String,
    pub answer: String,
    pub difficulty: i16,
    pub ease_factor: f64,
    pub interval_days: i32,
    pub repetitions: i32,
    pub next_review_date: NaiveDate,
    pub last_reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFlashcardRequest {
    pub topic: String,
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Deserialize)]
pub struct ReviewFlashcardRequest {
    pub flashcard_id: Uuid,
    pub quality: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LearningTopic {
    pub id: Uuid,
    pub user_id: Uuid,
    pub topic_name: String,
    pub description: Option<String>,
    pub total_micro_lessons: i32,
    pub completed_micro_lessons: i32,
    pub estimated_mastery_days: Option<i32>,
    pub mastery_percentage: f64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTopicRequest {
    pub topic_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MicroLesson {
    pub id: Uuid,
    pub topic_id: Uuid,
    pub lesson_order: i16,
    pub title: String,
    pub content: String,
    pub duration_minutes: i16,
    pub next_scheduled_at: Option<DateTime<Utc>>,
    pub completed: bool,
    pub score: Option<f64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CoachMessage {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub content: String,
    pub suggestion_type: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SendMessageRequest {
    #[validate(length(min = 1, max = 5000))]
    pub content: String,
}
