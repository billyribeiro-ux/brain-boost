use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub subscription: String,
    pub status: String,
    pub chronotype: Option<String>,
    pub primary_goal: Option<String>,
    pub timezone: String,
    pub onboarding_completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub subscription: String,
    pub chronotype: Option<String>,
    pub primary_goal: Option<String>,
    pub onboarding_completed: bool,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            subscription: user.subscription,
            chronotype: user.chronotype,
            primary_goal: user.primary_goal,
            onboarding_completed: user.onboarding_completed,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 1, max = 100))]
    pub display_name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, max = 100))]
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub chronotype: Option<String>,
    pub primary_goal: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSchedule {
    pub id: Uuid,
    pub user_id: Uuid,
    pub morning_time: chrono::NaiveTime,
    pub afternoon_time: chrono::NaiveTime,
    pub evening_time: chrono::NaiveTime,
    pub notification_enabled: bool,
    pub haptic_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateScheduleRequest {
    pub morning_time: Option<String>,
    pub afternoon_time: Option<String>,
    pub evening_time: Option<String>,
    pub notification_enabled: Option<bool>,
    pub haptic_enabled: Option<bool>,
}
