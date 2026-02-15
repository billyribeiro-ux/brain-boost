use crate::realtime::ConnectionManager;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{error, info};

use super::{
    missed_day_checker, metrics_snapshot, readiness_monitor, session_reminder,
};

pub struct JobScheduler {
    pool: PgPool,
    manager: Arc<ConnectionManager>,
}

impl JobScheduler {
    pub fn new(pool: PgPool, manager: Arc<ConnectionManager>) -> Self {
        Self { pool, manager }
    }

    pub async fn start(self) {
        info!("Starting job scheduler");
        
        let pool1 = self.pool.clone();
        let manager1 = self.manager.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                if let Err(e) = session_reminder::run(&pool1, &manager1).await {
                    error!("Session reminder job failed: {}", e);
                }
            }
        });
        
        let pool2 = self.pool.clone();
        let manager2 = self.manager.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(86400));
            loop {
                interval.tick().await;
                if let Err(e) = missed_day_checker::run(&pool2, &manager2).await {
                    error!("Missed day checker job failed: {}", e);
                }
            }
        });
        
        let pool3 = self.pool.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(86400));
            loop {
                interval.tick().await;
                if let Err(e) = metrics_snapshot::run(&pool3).await {
                    error!("Metrics snapshot job failed: {}", e);
                }
            }
        });
        
        let pool4 = self.pool.clone();
        let manager4 = self.manager.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(900));
            loop {
                interval.tick().await;
                if let Err(e) = readiness_monitor::run(&pool4, &manager4).await {
                    error!("Readiness monitor job failed: {}", e);
                }
            }
        });
        
        info!("All background jobs started");
    }
}
