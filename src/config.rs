use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WakeupConfig {
    pub target_services: Vec<String>,
    pub interval_seconds: u64,
    pub max_retries: u32,
}

impl WakeupConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // 优先从环境变量读取
        if let Ok(config_path) = env::var("WAKEUP_CONFIG_PATH") {
            if Path::new(&config_path).exists() {
                let config_content = fs::read_to_string(config_path)?;
                return Ok(toml::from_str(&config_content)?);
            }
        }

        // 默认配置
        Ok(WakeupConfig {
            target_services: vec![
                "https://dvt.onrender.com".to_string(),
                "https://dvt-1.onrender.com".to_string(),
                "https://dvt-2.onrender.com".to_string(),
                "https://anotherairaccountcommunitynode.onrender.com".to_string(),
            ],
            interval_seconds: 300, // 5分钟
            max_retries: 3,
        })
    }
}