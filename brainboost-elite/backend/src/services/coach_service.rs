use crate::errors::Result;
use crate::models::session::{CoachMessage, SendMessageRequest};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn send_message(pool: &PgPool, user_id: Uuid, req: SendMessageRequest) -> Result<CoachMessage> {
    let user_message = sqlx::query_as::<_, CoachMessage>(
        r#"
        INSERT INTO coach_messages (user_id, role, content)
        VALUES ($1, 'user', $2)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(&req.content)
    .fetch_one(pool)
    .await?;

    let coach_response = format!("I understand you're asking about: {}. As your AI coach, I'm here to help optimize your cognitive training journey.", req.content);
    
    let coach_message = sqlx::query_as::<_, CoachMessage>(
        r#"
        INSERT INTO coach_messages (user_id, role, content)
        VALUES ($1, 'coach', $2)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(&coach_response)
    .fetch_one(pool)
    .await?;

    Ok(coach_message)
}

pub async fn get_message_history(pool: &PgPool, user_id: Uuid, limit: i64) -> Result<Vec<CoachMessage>> {
    let messages = sqlx::query_as::<_, CoachMessage>(
        r#"
        SELECT * FROM coach_messages
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(messages)
}
