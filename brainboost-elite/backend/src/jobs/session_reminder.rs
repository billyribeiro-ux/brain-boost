use crate::errors::Result;
use crate::realtime::{ConnectionManager, ServerMessage};
use crate::realtime::messages::{ExerciseSummary, SessionReminderPayload};
use chrono::{Duration, Utc};
use sqlx::PgPool;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref SENT_REMINDERS: Mutex<HashSet<(Uuid, String, String)>> = Mutex::new(HashSet::new());
}

pub async fn run(pool: &PgPool, manager: &Arc<ConnectionManager>) -> Result<()> {
    let now = Utc::now().naive_utc();
    let reminder_window = now + Duration::minutes(5);
    
    let sessions = sqlx::query!(
        r#"
        SELECT 
            ds.id, ds.user_id, ds.slot, ds.status,
            ul.level_number, ul.week_number, ul.current_day
        FROM daily_sessions ds
        JOIN user_levels ul ON ds.user_id = ul.user_id AND ul.status = 'active'
        WHERE ds.session_date = CURRENT_DATE
        AND ds.status = 'pending'
        "#
    )
    .fetch_all(pool)
    .await?;
    
    for session in sessions {
        let key = (session.user_id, session.session_date.to_string(), session.slot.clone());
        
        let mut sent = SENT_REMINDERS.lock().await;
        if sent.contains(&key) {
            continue;
        }
        
        let exercises = vec![
            ExerciseSummary {
                name: "Meditation".to_string(),
                duration_minutes: 10,
                exercise_type: "meditation".to_string(),
            },
            ExerciseSummary {
                name: "Flashcards".to_string(),
                duration_minutes: 15,
                exercise_type: "flashcard".to_string(),
            },
        ];
        
        let message = ServerMessage::SessionReminder(SessionReminderPayload {
            session_id: session.id,
            slot: session.slot.clone(),
            message: format!("Your {} session is starting soon!", session.slot),
            exercises,
            scheduled_time: now.format("%H:%M").to_string(),
            haptic_pattern: "short".to_string(),
        });
        
        if manager.send_to_user(session.user_id, message).await {
            sent.insert(key);
            info!("Sent reminder to user {} for {} session", session.user_id, session.slot);
        }
    }
    
    Ok(())
}
