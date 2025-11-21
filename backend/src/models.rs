use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
// use mongodb::bson::DateTime; 

#[derive(Debug, Serialize, Deserialize)]
pub struct JeuVideo {
    // Mapping : Dans Mongo c'est "_id", dans Rust on l'appelle "id"
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub titre: String,
    #[serde(default)]
    pub genre: Vec<String>,
    #[serde(default)]
    pub plateforme: Vec<String>,
    pub editeur: Option<String>, 
    pub developpeur: Option<String>,
    pub annee_sortie: Option<i32>,
    pub metacritic_score: Option<i32>, 
    pub temps_jeu_heures: Option<f64>, 
    pub termine: bool,
    pub date_ajout: String,
    pub date_modification: String,
}