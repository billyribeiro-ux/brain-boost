use crate::errors::Result;
use crate::realtime::{ConnectionManager, ServerMessage};
use crate::realtime::messages::StressShieldPayload;
use crate::services::stress_shield_service;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref LAST_ALERTS: Mutex<HashMap<Uuid, DateTime<Utc>>> = Mutex::new(HashMap::new());
}

pub async fn run(pool: &PgPool, manager: &Arc<ConnectionManager>) -> Result<()> {
    let users = sqlx::query!(
        "SELECT DISTINCT user_id FROM user_levels WHERE status = 'active'"
    )
    .fetch_all(pool)
    .await?;
    
    for user in users {
        let readiness = stress_shield_service::check_readiness(pool, user.user_id).await?;
        
        if readiness.readiness_score < 40.0 {
            let mut alerts = LAST_ALERTS.lock().await;
            let now = Utc::now();
            
            let should_alert = alerts
                .get(&user.user_id)
                .map(|last| (now - *last).num_hours() >= 2)
                .unwrap_or(true);
            
            if should_alert {
                let message = ServerMessage::StressShieldAlert(StressShieldPayload {
                    readiness_score: readiness.readiness_score,
                    stress_index: readiness.stress_index,
                    recommendation: readiness.recommendation,
                    auto_nsdr: readiness.auto_nsdr,
                });
                
                if manager.send_to_user(user.user_id, message).await {
                    alerts.insert(user.user_id, now);
                }
            }
        }
    }
    
    Ok(())
}
