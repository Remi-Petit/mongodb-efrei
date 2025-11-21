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
    #[serde(default)]
    pub date_ajout: String,
    #[serde(default)]
    pub date_modification: String,
    #[serde(default)]
    pub favori: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GameStats {
    pub total_jeux: i64,
    pub temps_total_heures: f64,
    pub jeux_termines: i64,
    pub score_moyen: Option<f64>,
}


#[derive(Deserialize)]
pub struct SearchParams {
    pub genre: Option<String>,
    pub plateforme: Option<String>,
    pub titre: Option<String>,
}