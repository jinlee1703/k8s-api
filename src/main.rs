use std::env;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

mod db;
mod handlers;

#[get("/api/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "message": "서버가 정상 동작 중입니다."
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_db(&database_url).await.expect("Failed to initialize database");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(web::scope("/api")
            .route("/health", web::get().to(handlers::health_check))
            .route("/items", web::post().to(handlers::create_item))
            .route("/items", web::get().to(handlers::read_items))
            .route("/items/{id}", web::get().to(handlers::read_item))
            .route("/items/{id}", web::put().to(handlers::update_item))
            .route("/items/{id}", web::delete().to(handlers::delete_item))
        )
    })
    .bind("127.0.0.1:8080")?        
    .run()
    .await
}
