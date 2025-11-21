use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::db::AppState;
use crate::models::JeuVideo;
use mongodb::{bson::doc, Collection};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::to_document;
use futures::stream::TryStreamExt;
use chrono::Utc;

#[get("/health")]
pub async fn health_check(data: web::Data<AppState>) -> impl Responder {
    match data.db.run_command(doc! {"ping": 1}).await {
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

#[get("/games")]
pub async fn get_all_games(data: web::Data<AppState>) -> impl Responder {
    let collection: Collection<JeuVideo> = data.db.collection("games"); 
    match collection.find(doc! {}).await {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<JeuVideo>>().await {
                Ok(games) => HttpResponse::Ok().json(games),
                Err(e) => {
                    eprintln!("Erreur lors de la collection des jeux : {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to collect games",
                        "details": e.to_string()
                    }))
                }
            }
        }
        Err(e) => {
            eprintln!("Erreur lors de la récupération des jeux : {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch games",
                "details": e.to_string()
            }))
        }
    }
}


#[get("/games/{id}")]
pub async fn get_game(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_hex = path.into_inner();
    let obj_id = match ObjectId::parse_str(&id_hex) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Format d'ID invalide"),
    };
    let collection: Collection<JeuVideo> = data.db.collection("games");
    match collection.find_one(doc! {"_id": obj_id}).await {
        Ok(Some(game)) => HttpResponse::Ok().json(game),
        Ok(None) => HttpResponse::NotFound().body("Jeu non trouvé"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}


#[put("/games/{id}")]
pub async fn update_game(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<JeuVideo>
) -> impl Responder {
    let id_hex = path.into_inner();
    let obj_id = match ObjectId::parse_str(&id_hex) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Format d'ID invalide"),
    };
    let collection: Collection<JeuVideo> = data.db.collection("games");
    let mut update_doc = match to_document(&body) {
        Ok(doc) => doc,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };
    update_doc.remove("_id");
    match collection.update_one(doc! {"_id": obj_id}, doc! {"$set": update_doc}).await {
        Ok(result) => {
            if result.matched_count == 1 {
                HttpResponse::Ok().json(serde_json::json!({"message": "Jeu mis à jour avec succès"}))
            } else {
                HttpResponse::NotFound().body("Jeu non trouvé pour mise à jour")
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}


#[delete("/games/{id}")]
pub async fn delete_game(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_hex = path.into_inner();
    let obj_id = match ObjectId::parse_str(&id_hex) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Format d'ID invalide"),
    };
    let collection: Collection<JeuVideo> = data.db.collection("games");
    match collection.delete_one(doc! {"_id": obj_id}).await {
        Ok(result) => {
            if result.deleted_count == 1 {
                HttpResponse::Ok().json(serde_json::json!({"message": "Jeu supprimé"}))
            } else {
                HttpResponse::NotFound().body("Jeu non trouvé")
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}


#[post("/games")]
pub async fn create_game(data: web::Data<AppState>, body: web::Json<JeuVideo>) -> impl Responder {
    let collection: Collection<JeuVideo> = data.db.collection("games");
    let mut new_game = body.into_inner();
    new_game.id = Some(ObjectId::new());
    let current_time = Utc::now().to_rfc3339();
    new_game.date_ajout = current_time.clone();
    new_game.date_modification = current_time;
    match collection.insert_one(new_game).await {
        Ok(result) => {
            HttpResponse::Created().json(serde_json::json!({
                "message": "Jeu ajouté avec succès",
                "id": result.inserted_id.as_object_id().unwrap().to_string()
            }))
        }
        Err(e) => {
            eprintln!("Erreur lors de l'ajout du jeu : {}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}