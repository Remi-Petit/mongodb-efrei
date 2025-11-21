use actix_web::{get, web, HttpResponse, Responder};
use crate::db::AppState;

#[get("/health")]
pub async fn health_check(data: web::Data<AppState>) -> impl Responder {
    match data.db_client.list_database_names().await {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({
                "status": "up",
                "database": "connected"
            }))
        }
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "status": "down",
                "error": e.to_string()
            }))
        }
    }
}