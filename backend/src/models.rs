use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct JeuVideo {
    // Mapping : Dans Mongo c'est "_id", dans Rust on l'appelle "id"
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    // Règle : Requis (car pas Option) + Longueur min 1 caractère
    #[validate(length(min = 1, message = "Le titre ne peut pas être vide"))]
    pub titre: String,

    // Règle : Au moins 1 genre
    #[serde(default)]
    #[validate(length(min = 1, message = "Il faut au moins un genre"))]
    pub genre: Vec<String>,

    #[serde(default)]
    pub plateforme: Vec<String>,

    pub editeur: Option<String>,

    pub developpeur: Option<String>,

    // Règle : Année raisonnable (ex: premier jeu vidéo ~1950)
    #[serde(default)]
    #[validate(range(min = 1950, message = "L'année doit être supérieure à 1950"))]
    pub annee_sortie: Option<i32>,

    // Règle : Score entre 0 et 100
    #[serde(default)]
    #[validate(range(min = 0, max = 100, message = "Le score doit être entre 0 et 100"))]
    pub metacritic_score: Option<i32>,

    // Règle : Pas de temps de jeu négatif
    #[validate(range(min = 0.0, message = "Le temps de jeu ne peut pas être négatif"))]
    pub temps_jeu_heures: Option<f64>,

    #[serde(default)]
    pub termine: bool,

    #[serde(default)]
    pub date_ajout: String,

    #[serde(default)]
    pub date_modification: String,

    #[serde(default)]
    pub favori: bool,
}

#[derive(Deserialize)]
pub struct SearchParams {
    pub genre: Option<String>,
    pub plateforme: Option<String>,
    pub titre: Option<String>,
}