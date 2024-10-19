use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Item {
    id: i32,
    name: String,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "message": "서버가 정상 동작 중입니다."
    }))
}