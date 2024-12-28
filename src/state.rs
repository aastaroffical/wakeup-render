use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub last_success: Option<DateTime<Utc>>,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub last_failure: Option<DateTime<Utc>>,
    pub success_count: u32,
    pub failure_count: u32,
}

pub struct AppState {
    statuses: Mutex<HashMap<String, ServiceStatus>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            statuses: Mutex::new(HashMap::new()),
        }
    }

    pub fn record_success(&self, service: &str) {
        let mut statuses = self.statuses.lock();
        let status = statuses.entry(service.to_string()).or_insert(ServiceStatus {
            last_success: None,
            last_failure: None,
            success_count: 0,
            failure_count: 0,
        });

        status.last_success = Some(Utc::now());
        status.success_count += 1;
    }

    pub fn record_failure(&self, service: &str) {
        let mut statuses = self.statuses.lock();
        let status = statuses.entry(service.to_string()).or_insert(ServiceStatus {
            last_success: None,
            last_failure: None,
            success_count: 0,
            failure_count: 0,
        });

        status.last_failure = Some(Utc::now());
        status.failure_count += 1;
    }

    pub fn get_status(&self) -> HashMap<String, ServiceStatus> {
        self.statuses.lock().clone()
    }
}