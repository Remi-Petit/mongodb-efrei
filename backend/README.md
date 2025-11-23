```markdown
# 🎮 API Jeux Vidéo — Documentation complète (fichier unique)

## Table des matières
1. Aperçu
2. Fichiers fournis
3. Prérequis & variables d'environnement
4. Démarrage
5. Structure du projet
6. Modèle de données (`JeuVideo`)
7. Paramètres de recherche (`SearchParams`)
8. Endpoints de l'API
9. Validation côté serveur
10. Erreurs & codes de réponse
11. Exemples cURL
12. Suggestions / fichiers manquants
13. Notes techniques

---

## Aperçu
API REST écrite en **Rust** avec **Actix Web**, utilisant **MongoDB** pour la persistance des données. L'API permet :
- CRUD complet sur les jeux vidéo
- Recherche avancée via query params
- Statistiques (agrégation MongoDB)
- Export JSON
- Marquer un jeu en "favori"

---

## Fichiers fournis
- `main.rs` : point d'entrée du serveur
- `db.rs` : initialisation MongoDB & AppState
- `models.rs` : structures `JeuVideo` & `SearchParams`
- `src/api/controller.rs` : logique des endpoints
- `src/api/route.rs` : définition des routes
- `src/api/mod.rs` : module API

---

## Prérequis & variables d'environnement

### Requis
- Rust stable
- Cargo
- MongoDB
- Un fichier `.env`

### Exemple `.env`
```
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
MONGODB_URI=mongodb://localhost:27017
```

---

## Démarrage

### Installation
```bash
cargo build
```

### Lancement
```bash
cargo run
```

Serveur accessible sur :
```
http://127.0.0.1:8080/api
```

---

## Structure du projet
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

## Modèle de données — `JeuVideo`
```rust\#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct JeuVideo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[validate(length(min = 1))]
    pub titre: String,

    #[serde(default)]
    #[validate(length(min = 1))]
    pub genre: Vec<String>,

    #[serde(default)]
    pub plateforme: Vec<String>,

    pub editeur: Option<String>,
    pub developpeur: Option<String>,

    #[serde(default)]
    #[validate(range(min = 1950))]
    pub annee_sortie: Option<i32>,

    #[serde(default)]
    #[validate(range(min = 0, max = 100))]
    pub metacritic_score: Option<i32>,

    #[serde(default)]
    #[validate(range(min = 0.0))]
    pub temps_de_jeu: Option<f32>,

    #[serde(default)]
    pub date_ajout: Option<DateTime>,

    #[serde(default)]
    pub favoris: bool,

    #[serde(default)]
    pub description: Option<String>,
}
```

---

## Paramètres de recherche — `SearchParams`
```rust
pub struct SearchParams {
    pub titre: Option<String>,
    pub genre: Option<String>,
    pub plateforme: Option<String>,
    pub editeur: Option<String>,
    pub developpeur: Option<String>,
    pub annee_min: Option<i32>,
    pub annee_max: Option<i32>,
    pub score_min: Option<i32>,
    pub score_max: Option<i32>,
    pub favoris: Option<bool>,
}
```

---

## Endpoints de l'API

### 🟢 Healthcheck
```
GET /api/health
```
Réponse :
```json
{"status":"ok"}
```

### 🔍 Liste des jeux
```
GET /api/games
```
Query params disponibles : titre, genre, editeur, developpeur, annee_min, annee_max, score_min, score_max, favoris.

### 📘 Récupérer un jeu
```
GET /api/games/{id}
```

### ➕ Créer un jeu
```
POST /api/games
```
Body JSON :
```json
{
  "titre": "Zelda BOTW",
  "genre": ["Aventure"],
  "plateforme": ["Switch"],
  "annee_sortie": 2017
}
```

### ✏️ Mettre à jour un jeu
```
PUT /api/games/{id}
```

### 🗑️ Supprimer un jeu
```
DELETE /api/games/{id}
```

### ⭐ Basculer favori
```
POST /api/games/{id}/favorite
```

### 📊 Statistiques
```
GET /api/stats
```
Retourne par exemple :
- nombre total de jeux
- répartition par genre
- moyenne des scores

### 📤 Export JSON
```
GET /api/games/export
```
Retourne un fichier JSON téléchargeable.

---

## Validation
Utilise `validator` crate :
- titres non vides
- genres non vides
- années ≥ 1950
- scores 0–100
- temps de jeu ≥ 0

---

## Erreurs & codes de réponse
| Code | Signification |
|------|--------------|
| 200 | OK |
| 201 | Créé |
| 400 | Requête invalide |
| 404 | Non trouvé |
| 500 | Erreur serveur |

---

## Exemples cURL

### Ajouter un jeu
```bash
curl -X POST http://localhost:8080/api/games \
  -H "Content-Type: application/json" \
  -d '{"titre":"Halo","genre":["FPS"],"plateforme":["Xbox"]}'
```

### Filtrer
```bash
curl "http://localhost:8080/api/games?titre=zelda&score_min=80"
```

### Supprimer
```bash
curl -X DELETE http://localhost:8080/api/games/ID
```

---

## Suggestions / fichiers manquants
- `Cargo.toml` complet
- `.env.example`
- Tests unitaires
- Middleware logging
- Documentation OpenAPI/Swagger

---

## Notes techniques
- MongoDB utilisé en mode async
- Actix Web 4.x
- Validation automatique via `Validate`

```

