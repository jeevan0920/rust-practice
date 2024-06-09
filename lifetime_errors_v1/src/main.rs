use dashmap::DashMap;
use lazy_static::lazy_static;
use log::{debug, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::task;
use tokio::time::{interval, interval_at, Instant};

lazy_static! {
    static ref GLOBAL_MAP: DashMap<String, Arc<dyn std::any::Any + Send + Sync>> = DashMap::new();
}

async fn async_function() {
    for entry in GLOBAL_MAP.iter() {
        let key = entry.key().clone();
        let value = entry.value().clone();

        debug!("Processing key: {:?}", key);
        // Simulate some async processing
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn periodic_task() {
    async_function().await;
}

#[tokio::main]
async fn main() {
    // Initialize logger
    // env_logger::init();

    // Schedule the task to run every 10 seconds
    let start = Instant::now() + Duration::from_secs(10); // Start 10 seconds after program start
    let mut interval = interval_at(start, Duration::from_secs(10));
    tokio::spawn(async move {
        loop {
            interval.tick().await;
            warn!("Triggering periodic task");
            periodic_task().await;
        }
    });

    // Simulate some main logic
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
