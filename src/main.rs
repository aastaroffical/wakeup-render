mod config;
mod wakeup;
mod api;
mod state;

use actix_web::{web, App, HttpServer};
use env_logger::Env;
use log::{info, error};
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // 加载配置
    let config = match config::WakeupConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load config: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Config load failed"));
        }
    };

    // 初始化全局状态
    let app_state = Arc::new(state::AppState::new());

    // 启动唤醒任务
    let wakeup_state = app_state.clone();
    tokio::spawn(async move {
        wakeup::start_wakeup_task(wakeup_state, config).await;
    });

    // 启动Web服务
    info!("Starting server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(api::config_services)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}