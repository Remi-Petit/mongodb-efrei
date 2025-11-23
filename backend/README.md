```markdown
# 🎮 Documentation complète — API Jeux Vidéo (Rust + Actix Web + MongoDB)

Ce document regroupe **toute la documentation complète**, entièrement en **Markdown**, prêt à être déposé sur GitHub.

---

## 📌 Aperçu
API REST écrite en **Rust** utilisant :
- **Actix Web** pour le serveur HTTP
- **MongoDB** comme base de données NoSQL
- **Validator** pour la validation des modèles
- **Serde** pour la sérialisation JSON

Fonctionnalités incluses :
- CRUD complet
- Recherche par query params
- Statistiques (Agrégation MongoDB)
- Export JSON
- Gestion du statut « favori »
- Champs automatiques de dates

---

## 📁 Structure du projet
```
src/
 ├── main.rs
 ├── db.rs
 ├── models.rs
 └── api/
      ├── controller.rs
      ├── route.rs
      └── mod.rs
```

---

## ⚙️ Prérequis & Configuration

### Variables d'environnement (`.env`)
```
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
MONGODB_URI=mongodb://localhost:27017
```

### Lancer le serveur
```bash
cargo run
```

---

## 📘 Modèle : `JeuVideo`
```rust
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct JeuVideo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[validate(length(min = 1, message = "Le titre ne peut pas être vide"))]
    pub titre: String,

    #[serde(default)]
    #[validate(length(min = 1, message = "Il faut au moins un genre"))]
    pub genre: Vec<String>,

    #[serde(default)]
    pub plateforme: Vec<String>,

    pub editeur: Option<String>,
    pub developpeur: Option<String>,

    #[serde(default)]
    #[validate(range(min = 1950, message = "L'année doit être supérieure à 1950"))]
    pub annee_sortie: Option<i32>,

    #[serde(default)]
    #[validate(range(min = 0, max = 100, message = "Le score doit être entre 0 et 100"))]
    pub metacritic_score: Option<i32>,

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
```

---

## 🔍 Paramètres de recherche : `SearchParams`
```rust
pub struct SearchParams {
    pub genre: Option<String>,
    pub plateforme: Option<String>,
    pub titre: Option<String>,
}
```

---

# 🛠️ Endpoints — Documentation complète

## 🔥 1. Healthcheck
```
GET /api/health
```
Réponse :
```json
{
  "status": "up",
  "database": "connected"
}
```

---

## 📜 2. Récupérer tous les jeux
```
GET /api/games
```
### Query params disponibles
| Paramètre | Exemple | Description |
|----------|---------|-------------|
| `titre` | `?titre=zelda` | Recherche floue
| `genre` | `?genre=Indie` | Filtrer par genre
| `plateforme` | `?plateforme=PC` | Filtrer par plateforme |

### Exemple
```bash
curl "http://localhost:8080/api/games?titre=celeste&genre=Indie"
```

---

## 📘 3. Récupérer un jeu par ID
```
GET /api/games/{id}
```
Exemple :
```bash
curl http://localhost:8080/api/games/6920c751d63b5f5c333bdef1
```

---

## ➕ 4. Ajouter un jeu
```
POST /api/games
```
### Exemple de body JSON
```json
{
  "titre": "Celeste",
  "genre": ["Platformer", "Indie"],
  "plateforme": ["PC", "Switch", "PS4", "Xbox One"],
  "editeur": "Matt Makes Games",
  "developpeur": "Maddy Makes Games",
  "annee_sortie": 2018,
  "metacritic_score": 92,
  "temps_jeu_heures": 18,
  "termine": true,
  "date_ajout": "2024-02-10T12:00:00.000Z",
  "date_modification": "2024-02-10T12:00:00.000Z"
}
```

### Exemple cURL
```bash
curl -X POST http://localhost:8080/api/games \ 
  -H "Content-Type: application/json" \ 
  -d @game.json
```

---

## ✏️ 5. Mettre à jour un jeu
```
PUT /api/games/{id}
```
### Exemple de body JSON
```json
{
  "titre": "Celeste",
  "genre": ["Platformer", "Indie"],
  "plateforme": ["PC", "Switch", "PS4", "Xbox One"],
  "editeur": "Matt Makes Games",
  "developpeur": "Maddy Makes Games",
  "annee_sortie": 2018,
  "metacritic_score": 92,
  "temps_jeu_heures": 19.0,
  "termine": true,
  "date_ajout": "2024-02-10T12:00:00.000Z",
  "date_modification": "2024-02-10T12:00:00.000Z"
}
```

### Exemple cURL
```bash
curl -X PUT http://localhost:8080/api/games/6920c751d63b5f5c333bdef1 \ 
  -H "Content-Type: application/json" \ 
  -d @update.json
```

---

## 🗑️ 6. Supprimer un jeu
```
DELETE /api/games/{id}
```
Exemple :
```bash
curl -X DELETE http://localhost:8080/api/games/6920c751d63b5f5c333bdef1
```

---

## ⭐ 7. Activer/Désactiver le favori
```
POST /api/games/{id}/favorite
```
Réponse :
```json
{
  "message": "Statut favori mis à jour",
  "favori": true
}
```

---

## 📊 8. Statistiques
```
GET /api/stats
```
### Exemple de réponse
```json
{
  "total_jeux": 42,
  "temps_total_heures": 527.5,
  "jeux_termines": 18,
  "score_moyen": 81.4
}
```

---

## 📤 9. Export JSON
```
GET /api/games/export
```
Télécharge un fichier : `games_export.json`

---

# ❗ Gestion des erreurs
| Code | Signification |
|------|--------------|
| 200 | OK |
| 201 | Créé |
| 400 | Erreur de validation |
| 404 | Ressource introuvable |
| 500 | Erreur interne |

---

# 🧪 Exemples rapides cURL
### Ajouter un jeu
```bash
curl -X POST http://localhost:8080/api/games \
  -H "Content-Type: application/json" \
  -d '{"titre":"Halo","genre":["FPS"],"plateforme":["Xbox"]}'
```

### Filtrer
```bash
curl "http://localhost:8080/api/games?genre=RPG&titre=witcher"
```

---

# 📌 Notes
- API totalement asynchrone
- MongoDB utilisé avec `TryStreamExt` et agrégations
- Respect strict du schéma via `validator`

```

