use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Serialize, Deserialize)]
pub struct Item {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    name: String,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "message": "서버가 정상 동작 중입니다."
    }))
}

pub async fn create_item(pool: web::Data<MySqlPool>, item: web::Json<Item>) -> impl Responder {
    match sqlx::query!("INSERT INTO item (name) VALUES (?)", item.name)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::BadRequest().finish()   
    }
}

pub async fn read_items(pool: web::Data<MySqlPool>) -> impl Responder {
    match sqlx::query!("SELECT id, name FROM item")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(rows) => {
            let items: Vec<Item> = rows
                .into_iter()
                .map(|row| Item {
                    id: Some(row.id),
                    name: row.name,
                })
                .collect();
            HttpResponse::Ok().json(items)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}