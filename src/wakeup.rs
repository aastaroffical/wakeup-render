use crate::config::WakeupConfig;
use crate::state::AppState;
use log::{info, error};
use reqwest;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

pub async fn start_wakeup_task(state: Arc<AppState>, config: WakeupConfig) {
    let client = reqwest::Client::new();

    loop {
        for service in &config.target_services {
            match wake_service(&client, service).await {
                Ok(_) => {
                    info!("Successfully woke up service: {}", service);
                    state.record_success(service);
                }
                Err(e) => {
                    error!("Failed to wake up service {}: {}", service, e);
                    state.record_failure(service);
                }
            }
        }

        time::sleep(Duration::from_secs(config.interval_seconds)).await;
    }
}

async fn wake_service(client: &reqwest::Client, url: &str) -> Result<(), reqwest::Error> {
    client.get(url).send().await?;
    Ok(())
}