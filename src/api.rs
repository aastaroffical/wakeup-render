use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;
use std::sync::Arc;
use crate::state::AppState;

#[get("/status")]
async fn get_status(state: web::Data<Arc<AppState>>) -> impl Responder {
    let statuses = state.get_status();
    HttpResponse::Ok().json(json!(statuses))
}

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(get_status);
}