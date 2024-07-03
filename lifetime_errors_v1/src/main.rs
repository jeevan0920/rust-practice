use dashmap::DashMap;
use lazy_static::lazy_static;
use log::{debug, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::task;
use tokio::time::{interval, interval_at, Instant};

lazy_static! {
    static ref GLOBAL_MAP: DashMap<String, Arc<dyn std::any::Any + Send + Sync + 'static>> =
        DashMap::new();
}

async fn async_function() {
    let keys: Vec<String> = GLOBAL_MAP.iter().map(|entry| entry.key().clone()).collect();

    for key in keys {
        if let Some(value) = GLOBAL_MAP.get(&key) {
            let value = value.clone();
            debug!("Processing key: {:?}", key);
            // Simulate some async processing
            tokio::time::sleep(Duration::from_millis(100)).await;

            // Here, you could also update the map if necessary
            GLOBAL_MAP.insert(key.clone(), Arc::new(MyStruct { data: "new data" }));
        }
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
        // Simulate updating the map in the main logic
        let key = format!("key{}", rand::random::<u32>());
        let value: Arc<dyn std::any::Any + Send + Sync + 'static> =
            Arc::new(MyStruct { data: "some data" });
        GLOBAL_MAP.insert(key, value);

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

struct MyStruct {
    data: &'static str,
}

// impl std::any::Any for MyStruct {}

impl MyStruct {
    fn new(data: &'static str) -> Self {
        Self { data }
    }
}
