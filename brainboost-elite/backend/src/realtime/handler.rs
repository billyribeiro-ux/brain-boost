use crate::errors::{AppError, Result};
use crate::services::auth_service;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        Query, State, WebSocketUpgrade,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{error, info, warn};
use uuid::Uuid;

use super::manager::ConnectionManager;
use super::messages::{ClientMessage, ServerMessage};

#[derive(Deserialize)]
pub struct WsQuery {
    token: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WsQuery>,
    State(pool): State<PgPool>,
    State(manager): State<Arc<ConnectionManager>>,
) -> Result<Response> {
    let claims = auth_service::verify_access_token(&query.token)
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;
    
    let user_id = claims.sub;
    
    Ok(ws.on_upgrade(move |socket| handle_socket(socket, user_id, pool, manager)))
}

async fn handle_socket(
    socket: WebSocket,
    user_id: Uuid,
    pool: PgPool,
    manager: Arc<ConnectionManager>,
) {
    info!("WebSocket connection established for user {}", user_id);
    
    let (mut sender, mut receiver) = socket.split();
    let mut rx = manager.register_user(user_id).await;
    
    let mut send_task = tokio::spawn(async move {
        let mut heartbeat = interval(Duration::from_secs(30));
        
        loop {
            tokio::select! {
                _ = heartbeat.tick() => {
                    if sender.send(Message::Text(
                        serde_json::to_string(&ServerMessage::Pong).unwrap()
                    )).await.is_err() {
                        break;
                    }
                }
                
                Ok(msg) = rx.recv() => {
                    let json = match serde_json::to_string(&msg) {
                        Ok(j) => j,
                        Err(e) => {
                            error!("Failed to serialize message: {}", e);
                            continue;
                        }
                    };
                    
                    if sender.send(Message::Text(json)).await.is_err() {
                        break;
                    }
                }
            }
        }
    });
    
    let mut recv_task = tokio::spawn(async move {
        let mut last_ping = tokio::time::Instant::now();
        
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(ClientMessage::Ping) => {
                        last_ping = tokio::time::Instant::now();
                    }
                    Ok(ClientMessage::SubscribeMetrics) => {
                        info!("User {} subscribed to metrics", user_id);
                    }
                    Ok(ClientMessage::UnsubscribeMetrics) => {
                        info!("User {} unsubscribed from metrics", user_id);
                    }
                    Ok(ClientMessage::AcknowledgeNotification { notification_id }) => {
                        info!("User {} acknowledged notification {}", user_id, notification_id);
                    }
                    Err(e) => {
                        warn!("Failed to parse client message: {}", e);
                    }
                }
            }
            
            if last_ping.elapsed() > Duration::from_secs(40) {
                warn!("User {} heartbeat timeout", user_id);
                break;
            }
        }
    });
    
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }
    
    manager.unregister_user(user_id).await;
    info!("WebSocket connection closed for user {}", user_id);
}
