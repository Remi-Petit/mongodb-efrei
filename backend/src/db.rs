use mongodb::{Client, options::ClientOptions};
use std::env;
use std::io::{Error, ErrorKind};

#[derive(Clone)]
pub struct AppState {
    pub db_client: Client,
}

pub async fn init() -> std::io::Result<Client> {
    let client_uri = env::var("MONGODB_URI")
        .expect("MONGODB_URI n'est pas défini dans le .env");

    println!("{}", client_uri);

    let client_options = ClientOptions::parse(&client_uri).await.map_err(|e| {
        Error::new(ErrorKind::Other, format!("Invalid Mongo URI: {}", e))
    })?;

    let client = Client::with_options(client_options).map_err(|e| {
        Error::new(ErrorKind::Other, format!("Client init failed: {}", e))
    })?;

    println!("✅ Connexion à MongoDB initialisée avec succès.");

    Ok(client)
}