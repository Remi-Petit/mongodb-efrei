use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::env;
use dotenv::dotenv;

// Déclaration des modules
mod db;
mod api;
mod models;

use db::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); 

    // Configuration
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let server_address = format!("{}:{}", host, port);

    // Initialisation DB
    let database = db::init().await?; 
    let app_state = AppState { db: database };

    println!("🚀 Serveur lancé sur http://{}", server_address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors) // Ajout du middleware CORS
            .app_data(web::Data::new(app_state.clone())) // Injection de dépendance
            .configure(api::route::config)               // Configuration des routes via le module api
    })
    .bind(server_address)?
    .run()
    .await
}