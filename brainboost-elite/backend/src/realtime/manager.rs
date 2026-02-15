use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;
use super::messages::ServerMessage;

#[derive(Clone)]
pub struct ConnectionManager {
    user_channels: Arc<RwLock<HashMap<Uuid, broadcast::Sender<ServerMessage>>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            user_channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_user(&self, user_id: Uuid) -> broadcast::Receiver<ServerMessage> {
        let mut channels = self.user_channels.write().await;
        let (tx, rx) = broadcast::channel(100);
        channels.insert(user_id, tx);
        rx
    }

    pub async fn unregister_user(&self, user_id: Uuid) {
        let mut channels = self.user_channels.write().await;
        channels.remove(&user_id);
    }

    pub async fn send_to_user(&self, user_id: Uuid, message: ServerMessage) -> bool {
        let channels = self.user_channels.read().await;
        if let Some(tx) = channels.get(&user_id) {
            tx.send(message).is_ok()
        } else {
            false
        }
    }

    pub async fn broadcast_to_all(&self, message: ServerMessage) {
        let channels = self.user_channels.read().await;
        for (_, tx) in channels.iter() {
            let _ = tx.send(message.clone());
        }
    }

    pub async fn connected_user_count(&self) -> usize {
        self.user_channels.read().await.len()
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}
