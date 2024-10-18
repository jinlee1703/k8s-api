use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/api/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "message": "서버가 정상 동작 중입니다."
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("서버 시작 중...");

    HttpServer::new(|| {
        App::new()
        .service(health_check)
    })
    .bind("127.0.0.1:8080")?        
    .run()
    .await
}
