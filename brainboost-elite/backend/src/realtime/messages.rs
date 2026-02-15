use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ServerMessage {
    SessionReminder(SessionReminderPayload),
    MetricsUpdate(MetricsUpdatePayload),
    CoachSuggestion(CoachSuggestionPayload),
    AchievementUnlocked(AchievementPayload),
    StreakUpdate(StreakPayload),
    StressShieldAlert(StressShieldPayload),
    ReadinessUpdate(ReadinessPayload),
    LevelAdvancement(LevelAdvancementPayload),
    DayMissedWarning(DayMissedPayload),
    SystemNotification(SystemNotificationPayload),
    Pong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ClientMessage {
    Ping,
    SubscribeMetrics,
    UnsubscribeMetrics,
    AcknowledgeNotification { notification_id: Uuid },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionReminderPayload {
    pub session_id: Uuid,
    pub slot: String,
    pub message: String,
    pub exercises: Vec<ExerciseSummary>,
    pub scheduled_time: String,
    pub haptic_pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseSummary {
    pub name: String,
    pub duration_minutes: i32,
    pub exercise_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsUpdatePayload {
    pub brain_score: f64,
    pub stress_index: f64,
    pub readiness: f64,
    pub streak: i32,
    pub completion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoachSuggestionPayload {
    pub suggestion_type: String,
    pub title: String,
    pub description: String,
    pub priority: String,
    pub action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementPayload {
    pub achievement_name: String,
    pub description: String,
    pub icon_name: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreakPayload {
    pub current_streak: i32,
    pub is_milestone: bool,
    pub milestone_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressShieldPayload {
    pub readiness_score: f64,
    pub stress_index: f64,
    pub recommendation: String,
    pub auto_nsdr: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadinessPayload {
    pub readiness_score: f64,
    pub trend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelAdvancementPayload {
    pub new_level: i32,
    pub level_name: String,
    pub new_daily_minutes: i32,
    pub new_techniques: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayMissedPayload {
    pub missed_date: String,
    pub grace_skip_available: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemNotificationPayload {
    pub title: String,
    pub body: String,
    pub notification_type: String,
}
