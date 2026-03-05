# Spec Technique — crate `miyualicia-api`

<!-- @id: spec.alicia.api -->
<!-- @role: technical-specification -->
<!-- @layer: 7 -->
<!-- @human: Specification technique complete du serveur API REST JWT Alicia -->
<!-- @do: define_miyualicia_api_crate_api -->

**Auteur :** Denis, Chef Dev Senior — Miyukini AI Studio
**Date :** 2026-03-01
**Version :** 1.0
**Reference :** Rapport Fondateur Alicia Home Assistante v1.0 §3.1, §4.2 BT-05

---

## Contexte

`miyualicia-api` est le serveur HTTP REST d'Alicia Home Assistante. Il expose l'etat de la maison
et les commandes domotiques vers les serveurs MWS distants de la famille Miyukini, via une API
securisee par JWT HS256 (cle locale). Il tourne sur le port 7890 (configurable) en parallele
de l'UI Dioxus, lance dans un `tokio::spawn` au demarrage de `apps/central`.

## Portee / Scope

Ce crate couvre :
- Le serveur axum (routage, middlewares)
- L'authentification JWT HS256 : generation, verification, extractor axum
- Les handlers REST (state, devices, rooms, automations, history, health)
- Les DTOs de requete et de reponse (types Rust serialises en JSON)
- Le rate limiting (100 req/min par IP)
- La gestion d'erreurs HTTP (codes appropriees)

Ce crate ne couvre pas :
- La logique metier (deleguee a `miyualicia::AliciaService`)
- Le stockage KindMother (delegue a `miyualicia`)
- Le WebSocket temps-reel (scope P2, hors Phase 2)
- Le TLS (termine a l'exterieur par un reverse proxy ou absent en LAN)

---

## 1. Emplacement et structure

```
crates/miyualicia-api/
├── Cargo.toml
└── src/
    ├── lib.rs                  # Racine, exports publics, MSCM
    ├── admin_cell.rs           # Cellule Admin
    ├── server.rs               # AliciaApiServer, demarrage axum
    ├── router.rs               # Definition des routes + middlewares
    ├── config.rs               # ApiConfig
    ├── auth.rs                 # JWT : claims, generation, verification, extractor
    ├── dto.rs                  # DTOs request/response
    ├── errors.rs               # ApiError → IntoResponse
    └── handlers/
        ├── mod.rs
        ├── auth_handler.rs     # POST /auth/token
        ├── state_handler.rs    # GET /state
        ├── rooms_handler.rs    # GET /rooms, GET /rooms/{id}
        ├── devices_handler.rs  # GET /devices, GET /devices/{id}, POST /devices/{id}/command
        ├── automations_handler.rs  # CRUD /automations
        ├── history_handler.rs  # GET /history
        └── health_handler.rs   # GET /health (non protege)
```

---

## 2. `Cargo.toml`

```toml
[package]
name = "miyualicia-api"
version = "0.1.0"
edition = "2021"
description = "Serveur API REST JWT — Alicia Home Assistante"
authors = ["Miyukini AI Studio"]

[lints.rust]
unsafe_code = "forbid"

[lints.clippy]
all = "warn"
pedantic = "warn"

[dependencies]
miyualicia         = { path = "../miyualicia" }
miyualicia-devices = { path = "../miyualicia-devices" }

axum               = { version = "0.7", features = ["tokio", "json", "macros"] }
tower              = "0.4"
tower-http         = { version = "0.5", features = ["cors", "trace", "limit", "request-id"] }
tokio              = { version = "1", features = ["full"] }
serde              = { version = "1", features = ["derive"] }
serde_json         = "1"
jsonwebtoken       = "9"
chrono             = { version = "0.4", features = ["serde"] }
uuid               = { version = "1", features = ["v4", "serde"] }
thiserror          = "1"
tracing            = "0.1"

[dev-dependencies]
axum-test          = "0.4"
tokio              = { version = "1", features = ["full", "test-util"] }
```

---

## 3. `src/lib.rs`

```rust
//! # miyualicia-api
//!
//! Serveur API REST pour Alicia Home Assistante.
//! Expose l'etat de la maison et les commandes domotiques via HTTP/JSON,
//! securise par JWT HS256 (cle locale, pas de cloud).
//!
//! ## Port par defaut
//!
//! Port 7890. Configurable dans `alicia.toml` section `[api]`.
//!
//! ## Authentication
//!
//! JWT HS256. Le token est obtenu via `POST /api/v1/alicia/auth/token`.
//! Duree de vie par defaut : 3600 secondes (1 heure).
//!
//! ## Loi d'Autonomie
//!
//! L'API est locale. Elle peut etre appelee depuis un serveur MWS distant
//! mais ne depend d'aucun service externe pour fonctionner.

#![forbid(unsafe_code)]

// @id: service.alicia.rest-api
// @role: http_api_gateway
// @layer: 7
// @human: Serveur REST JWT Alicia ; routes, auth, DTOs, handlers, rate limit.
// @do: expose_alicia_home_as_rest_api

pub mod admin_cell;
pub mod auth;
pub mod config;
pub mod dto;
pub mod errors;
pub mod handlers;
pub mod router;
pub mod server;

pub use config::ApiConfig;
pub use errors::ApiError;
pub use server::AliciaApiServer;
```

---

## 4. `src/config.rs` — `ApiConfig`

```rust
use serde::{Deserialize, Serialize};

/// Configuration du serveur API REST Alicia.
///
/// Chargee depuis la section `[api]` de `alicia.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Port d'ecoute. Defaut : 7890. Enregistre dans le registre des ports COG.
    #[serde(default = "ApiConfig::default_port")]
    pub port: u16,

    /// Hote d'ecoute. "0.0.0.0" pour etre accessible depuis le reseau LAN.
    /// "127.0.0.1" pour localhost uniquement (plus restrictif).
    #[serde(default = "ApiConfig::default_host")]
    pub host: String,

    /// Duree de vie des tokens JWT en secondes. Defaut : 3600 (1h).
    #[serde(default = "ApiConfig::default_token_ttl")]
    pub token_ttl_secs: u64,

    /// Nombre maximal de requetes par minute par adresse IP.
    /// Au-dela, le serveur repond 429 Too Many Requests.
    #[serde(default = "ApiConfig::default_rate_limit")]
    pub rate_limit_per_min: u32,

    /// Activer les headers CORS. Recommande si un frontend web accede a l'API.
    #[serde(default)]
    pub cors_enabled: bool,

    /// Origines CORS autorisees. Ignoree si `cors_enabled = false`.
    #[serde(default)]
    pub cors_origins: Vec<String>,
}

impl ApiConfig {
    fn default_port() -> u16 { 7890 }
    fn default_host() -> String { "0.0.0.0".to_string() }
    fn default_token_ttl() -> u64 { 3600 }
    fn default_rate_limit() -> u32 { 100 }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            port:               Self::default_port(),
            host:               Self::default_host(),
            token_ttl_secs:     Self::default_token_ttl(),
            rate_limit_per_min: Self::default_rate_limit(),
            cors_enabled:       false,
            cors_origins:       vec![],
        }
    }
}
```

---

## 5. `src/auth.rs` — JWT HS256

### 5.1 Claims

```rust
use serde::{Deserialize, Serialize};

/// Scopes d'acces autorises dans un token JWT Alicia.
///
/// Un token peut porter plusieurs scopes. L'absence d'un scope bloque
/// l'acces aux endpoints correspondants avec HTTP 403.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JwtScope {
    /// Lecture de l'etat complet (GET /state, /devices, /rooms).
    Read,
    /// Envoi de commandes (POST /devices/{id}/command).
    Write,
    /// Gestion des automatisations (CRUD /automations).
    Automations,
    /// Lecture de l'historique (GET /history).
    History,
    /// Acces administrateur (toutes les operations).
    Admin,
}

/// Claims du token JWT Alicia.
///
/// # Champs standard JWT
///
/// - `sub` : identifiant du client (exemple : "mws-principal", "mws-monitoring")
/// - `exp` : timestamp UNIX d'expiration (calcule a partir de `token_ttl_secs`)
/// - `iat` : timestamp UNIX d'emission
///
/// # Champs custom Alicia
///
/// - `scopes` : liste des permissions accordees au client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Subject : identifiant du client (client_id fourni lors de la demande de token).
    pub sub:    String,
    /// Expiration : timestamp UNIX en secondes.
    pub exp:    u64,
    /// Issued at : timestamp UNIX en secondes.
    pub iat:    u64,
    /// Scopes d'acces autorises.
    pub scopes: Vec<JwtScope>,
}

impl JwtClaims {
    /// Cree des claims pour un client avec les scopes donnes.
    /// `exp` est calcule a partir de maintenant + `ttl_secs`.
    pub fn new(sub: impl Into<String>, scopes: Vec<JwtScope>, ttl_secs: u64) -> Self;

    /// Retourne `true` si le claim est expire.
    pub fn is_expired(&self) -> bool;

    /// Retourne `true` si le scope demande est present.
    pub fn has_scope(&self, scope: &JwtScope) -> bool;

    /// Retourne `true` si le client a le scope Admin (acces total).
    pub fn is_admin(&self) -> bool;
}
```

### 5.2 Fonctions de generation et verification

```rust
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

/// Genere un token JWT signe HS256.
///
/// # Parametres
///
/// - `claims` : les claims a encoder dans le token
/// - `secret` : la cle secrete HS256 (minimum 32 octets recommande)
///
/// # Securite
///
/// La cle est stockee en KindMother (chiffree). Elle est chargee en memoire
/// uniquement pendant la duree du serveur API. Ne jamais la logger.
///
/// # Erreurs
///
/// Retourne `ApiError::JwtError` si la signature echoue.
pub fn generate_token(claims: &JwtClaims, secret: &[u8]) -> Result<String, ApiError>;

/// Verifie et decode un token JWT HS256.
///
/// # Verification effectuee
///
/// - Signature HMAC-SHA256 valide
/// - Token non expire (champ `exp`)
/// - Algorithme HS256 obligatoire (rejet de tous les autres)
///
/// # Erreurs
///
/// - `ApiError::Unauthorized` si la signature est invalide
/// - `ApiError::TokenExpired` si le token est expire
/// - `ApiError::JwtError` pour toute autre erreur jsonwebtoken
pub fn verify_token(token: &str, secret: &[u8]) -> Result<JwtClaims, ApiError>;
```

### 5.3 Extractor axum `JwtAuth`

```rust
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};

/// Extractor axum qui verifie le JWT dans l'en-tete `Authorization: Bearer <token>`.
///
/// # Usage dans un handler
///
/// ```rust
/// async fn get_state(
///     JwtAuth(claims): JwtAuth,
///     State(alicia): State<Arc<AliciaService>>,
/// ) -> Result<Json<StateDto>, ApiError> {
///     claims.require_scope(&JwtScope::Read)?;
///     // ...
/// }
/// ```
///
/// # Comportement si le token est absent ou invalide
///
/// L'extractor retourne immediatement `ApiError::Unauthorized` (HTTP 401).
pub struct JwtAuth(pub JwtClaims);

#[async_trait]
impl<S> FromRequestParts<S> for JwtAuth
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection>;
}
```

---

## 6. `src/router.rs` — Routes completes

### 6.1 Architecture du routeur

```rust
use axum::{
    Router,
    routing::{delete, get, post, put},
    middleware,
};
use tower_http::{
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    trace::TraceLayer,
};

/// Construit le routeur axum complet avec tous les middlewares.
///
/// # Middlewares appliques (de l'exterieur vers l'interieur)
///
/// 1. `TraceLayer`            : logs de chaque requete (method, path, status, duree)
/// 2. `RequestBodyLimitLayer` : corps limite a 64 Ko (protection DOS)
/// 3. `CorsLayer`             : si `config.cors_enabled`
/// 4. Rate limiting           : via `GovernorLayer` (tower) ou middleware custom
///
/// # Routes publiques (sans JWT)
///
/// - `GET /api/v1/alicia/health`
/// - `POST /api/v1/alicia/auth/token`
///
/// # Routes protegees (JWT requis)
///
/// Toutes les autres routes utilisent l'extractor `JwtAuth`.
pub fn build_router(
    alicia:  Arc<AliciaService>,
    config:  ApiConfig,
    secret:  Arc<Vec<u8>>,
) -> Router;
```

### 6.2 Table des routes

Prefixe global : `/api/v1/alicia`

| Methode | Chemin                            | Auth  | Scope         | Handler                    |
|---------|-----------------------------------|-------|---------------|----------------------------|
| POST    | `/auth/token`                     | Non   | —             | `auth_handler::token`      |
| GET     | `/health`                         | Non   | —             | `health_handler::health`   |
| GET     | `/state`                          | JWT   | Read          | `state_handler::get_state` |
| GET     | `/rooms`                          | JWT   | Read          | `rooms_handler::list`      |
| GET     | `/rooms/{room_id}`                | JWT   | Read          | `rooms_handler::get`       |
| GET     | `/rooms/{room_id}/devices`        | JWT   | Read          | `rooms_handler::devices`   |
| GET     | `/devices`                        | JWT   | Read          | `devices_handler::list`    |
| GET     | `/devices/{id}`                   | JWT   | Read          | `devices_handler::get`     |
| POST    | `/devices/{id}/command`           | JWT   | Write         | `devices_handler::command` |
| GET     | `/automations`                    | JWT   | Read          | `automations_handler::list`|
| POST    | `/automations`                    | JWT   | Automations   | `automations_handler::create`|
| PUT     | `/automations/{id}`               | JWT   | Automations   | `automations_handler::update`|
| DELETE  | `/automations/{id}`               | JWT   | Automations   | `automations_handler::delete`|
| POST    | `/automations/{id}/trigger`       | JWT   | Automations   | `automations_handler::trigger`|
| GET     | `/history`                        | JWT   | History       | `history_handler::list`    |

---

## 7. `src/dto.rs` — Data Transfer Objects

### 7.1 DTOs de requete

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Corps de la requete d'authentification.
///
/// Le client fournit son `client_id` et son `secret`.
/// Ces valeurs sont verifiees contre la table `alicia_api_tokens` (hash BLAKE3).
#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    pub client_id: String,
    pub secret:    String,
}

/// Corps d'une commande sur un dispositif.
#[derive(Debug, Deserialize)]
pub struct CommandRequest {
    /// Action a realiser. Exemple : "on", "off", "set_brightness".
    pub action: String,
    /// Valeur optionnelle associee a l'action.
    pub value:  Option<serde_json::Value>,
}

/// Corps de creation/modification d'une automatisation.
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationRequest {
    pub name:       String,
    pub enabled:    bool,
    pub trigger:    serde_json::Value, // type + config, serialises depuis TriggerType
    pub conditions: Vec<serde_json::Value>,
    pub actions:    Vec<serde_json::Value>,
}

/// Parametres de pagination pour GET /history.
#[derive(Debug, Deserialize)]
pub struct HistoryParams {
    /// Nombre maximum d'entrees retournees. Defaut : 50. Maximum : 500.
    #[serde(default = "HistoryParams::default_limit")]
    pub limit: u32,
    /// Offset pour la pagination.
    #[serde(default)]
    pub offset: u32,
    /// Filtrer par source : "voice", "api", "automation", "manual".
    pub source: Option<String>,
    /// Filtrer par device_id.
    pub device_id: Option<Uuid>,
}

impl HistoryParams {
    fn default_limit() -> u32 { 50 }
}
```

### 7.2 DTOs de reponse

```rust
use chrono::{DateTime, Utc};

/// Reponse d'authentification : le token JWT.
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    /// Le token JWT a inclure dans les requetes suivantes.
    pub access_token: String,
    /// Type : toujours "Bearer".
    pub token_type:   String,
    /// Duree de validite en secondes.
    pub expires_in:   u64,
    /// Scopes accordes.
    pub scopes:       Vec<String>,
}

/// Etat complet de la maison.
#[derive(Debug, Serialize)]
pub struct StateDto {
    pub rooms:          Vec<RoomDto>,
    pub mqtt_connected: bool,
    pub total_devices:  usize,
    pub active_devices: usize,
    pub timestamp:      DateTime<Utc>,
}

/// Etat d'une piece.
#[derive(Debug, Serialize)]
pub struct RoomDto {
    pub id:      String,
    pub name:    String,
    pub devices: Vec<DeviceDto>,
}

/// Etat d'un dispositif (version exposition API — sans credentials).
///
/// # Securite
///
/// Le champ `config` de `Device` contient potentiellement des credentials chiffres.
/// `DeviceDto` n'expose JAMAIS `config.auth_credential`.
#[derive(Debug, Serialize)]
pub struct DeviceDto {
    pub id:           Uuid,
    pub room_id:      String,
    pub device_type:  String,
    pub name:         String,
    pub protocol:     String,
    pub address:      String,
    pub capabilities: DeviceCapabilitiesDto,
    pub state:        DeviceStateDto,
    pub active:       bool,
}

/// Capacites d'un dispositif (exposition API).
#[derive(Debug, Serialize)]
pub struct DeviceCapabilitiesDto {
    pub on_off:             bool,
    pub dimmer:             bool,
    pub rgb:                bool,
    pub position:           bool,
    pub temperature_target: bool,
    pub power_measure:      bool,
}

/// Etat courant d'un dispositif (exposition API).
#[derive(Debug, Serialize)]
pub struct DeviceStateDto {
    pub on:                  Option<bool>,
    pub brightness:          Option<u8>,
    pub color_rgb:           Option<[u8; 3]>,
    pub position:            Option<u8>,
    pub temperature_current: Option<f32>,
    pub temperature_target:  Option<f32>,
    pub power_w:             Option<f32>,
    pub locked:              Option<bool>,
    pub motion:              Option<bool>,
    pub contact:             Option<bool>,
    pub humidity:            Option<f32>,
    pub last_updated:        DateTime<Utc>,
    /// `true` si l'etat est connu, `false` si tous les champs sont None.
    pub is_known:            bool,
}

/// Reponse a une commande.
#[derive(Debug, Serialize)]
pub struct CommandResponse {
    pub success:    bool,
    pub device_id:  Uuid,
    pub action:     String,
    pub latency_ms: u64,
    pub message:    Option<String>,
}

/// Entree dans le journal d'activite.
#[derive(Debug, Serialize)]
pub struct HistoryEntryDto {
    pub id:            Uuid,
    pub source:        String,
    pub source_detail: Option<String>,
    pub device_id:     Option<Uuid>,
    pub device_name:   Option<String>,
    pub command:       serde_json::Value,
    pub success:       bool,
    pub error_message: Option<String>,
    pub latency_ms:    Option<u64>,
    pub executed_at:   DateTime<Utc>,
}

/// Reponse paginee de l'historique.
#[derive(Debug, Serialize)]
pub struct HistoryResponse {
    pub entries: Vec<HistoryEntryDto>,
    pub total:   u64,
    pub limit:   u32,
    pub offset:  u32,
}

/// Etat de sante du service.
#[derive(Debug, Serialize)]
pub struct HealthDto {
    pub status:         &'static str, // "ok"
    pub version:        &'static str,
    pub uptime_secs:    u64,
    pub mqtt_connected: bool,
    pub devices_count:  usize,
}
```

---

## 8. `src/errors.rs` — `ApiError`

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Erreurs de l'API REST Alicia.
///
/// Chaque variante se traduit en un code HTTP et un body JSON d'erreur standard.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// Token JWT absent ou malformed.
    #[error("authentification requise")]
    Unauthorized,

    /// Token JWT expire.
    #[error("token expire")]
    TokenExpired,

    /// Token valide mais scope insuffisant.
    #[error("permission insuffisante : scope '{required}' requis")]
    Forbidden { required: String },

    /// Ressource non trouvee.
    #[error("ressource introuvable : {0}")]
    NotFound(String),

    /// Donnees de requete invalides (validation metier).
    #[error("requete invalide : {0}")]
    UnprocessableEntity(String),

    /// Trop de requetes (rate limiting).
    #[error("trop de requetes : reessayer dans {retry_after_secs} secondes")]
    TooManyRequests { retry_after_secs: u64 },

    /// Erreur interne du service Alicia.
    #[error("erreur interne : {0}")]
    InternalError(String),

    /// Erreur JWT (jsonwebtoken).
    #[error("erreur JWT : {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    /// Erreur du crate miyualicia (orchestrateur).
    #[error("erreur service Alicia : {0}")]
    AliciaError(String),
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized        => StatusCode::UNAUTHORIZED,          // 401
            Self::TokenExpired        => StatusCode::UNAUTHORIZED,          // 401
            Self::Forbidden { .. }   => StatusCode::FORBIDDEN,             // 403
            Self::NotFound(_)        => StatusCode::NOT_FOUND,             // 404
            Self::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY, // 422
            Self::TooManyRequests { .. } => StatusCode::TOO_MANY_REQUESTS,    // 429
            Self::InternalError(_)   => StatusCode::INTERNAL_SERVER_ERROR, // 500
            Self::JwtError(_)        => StatusCode::UNAUTHORIZED,          // 401
            Self::AliciaError(_)     => StatusCode::INTERNAL_SERVER_ERROR, // 500
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = json!({
            "error": {
                "code":    status.as_u16(),
                "message": self.to_string(),
                "type":    format!("{:?}", &self).split('(').next().unwrap_or("Unknown"),
            }
        });
        (status, Json(body)).into_response()
    }
}
```

---

## 9. `src/server.rs` — `AliciaApiServer`

```rust
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

/// Serveur API REST Alicia.
///
/// Lance le serveur axum sur le port configure. Cette structure est destinee
/// a etre creee une seule fois et lancee via `tokio::spawn`.
///
/// # Integration dans apps/central
///
/// ```rust
/// // Dans l'initialisation de l'application :
/// let api_server = AliciaApiServer::new(alicia_service.clone(), api_config, jwt_secret);
/// tokio::spawn(async move {
///     if let Err(e) = api_server.start().await {
///         tracing::error!("API Alicia arretee : {e}");
///     }
/// });
/// ```
pub struct AliciaApiServer {
    config:  ApiConfig,
    alicia:  Arc<AliciaService>,
    secret:  Arc<Vec<u8>>,
}

impl AliciaApiServer {
    /// Cree un nouveau serveur API.
    ///
    /// # Parametres
    ///
    /// - `alicia`  : le service Alicia partage (Arc)
    /// - `config`  : configuration API (port, rate limit, etc.)
    /// - `secret`  : cle JWT HS256 chargee depuis KindMother
    pub fn new(
        alicia: Arc<AliciaService>,
        config: ApiConfig,
        secret: Vec<u8>,
    ) -> Self;

    /// Demarre le serveur et bloque jusqu'a arret.
    ///
    /// # Comportement si le port est deja pris
    ///
    /// Retourne `ApiError::InternalError` immediatement.
    /// L'appelant (`apps/central`) ne doit PAS paniquer mais logger l'erreur
    /// et continuer sans l'API REST.
    ///
    /// # Arrêt propre
    ///
    /// Le serveur ecoute le signal d'arret de tokio (`ctrl_c` ou signal du runtime).
    pub async fn start(self) -> Result<(), ApiError>;

    /// Retourne l'adresse d'ecoute effective.
    pub fn listen_addr(&self) -> SocketAddr;
}
```

---

## 10. Handlers — signatures

### `auth_handler.rs`

```rust
/// POST /api/v1/alicia/auth/token
///
/// # Corps
///
/// ```json
/// { "client_id": "mws-principal", "secret": "mon-secret" }
/// ```
///
/// # Reponse 200
///
/// ```json
/// { "access_token": "eyJ...", "token_type": "Bearer", "expires_in": 3600, "scopes": ["read", "write"] }
/// ```
///
/// # Verification
///
/// Le secret est compare au hash BLAKE3 stocke dans `alicia_api_tokens`.
/// En cas d'echec : HTTP 401.
pub async fn token(
    State(alicia): State<Arc<AliciaService>>,
    State(config): State<ApiConfig>,
    State(secret): State<Arc<Vec<u8>>>,
    Json(req): Json<TokenRequest>,
) -> Result<Json<TokenResponse>, ApiError>;
```

### `state_handler.rs`

```rust
/// GET /api/v1/alicia/state
///
/// Retourne l'etat complet de la maison : toutes les pieces, tous les dispositifs,
/// statut MQTT, timestamp.
///
/// # Auth : JWT scope Read
pub async fn get_state(
    JwtAuth(claims): JwtAuth,
    State(alicia): State<Arc<AliciaService>>,
) -> Result<Json<StateDto>, ApiError>;
```

### `devices_handler.rs`

```rust
/// GET /api/v1/alicia/devices
/// Auth : JWT scope Read
pub async fn list(
    JwtAuth(claims): JwtAuth,
    State(alicia): State<Arc<AliciaService>>,
) -> Result<Json<Vec<DeviceDto>>, ApiError>;

/// GET /api/v1/alicia/devices/{id}
/// Auth : JWT scope Read
pub async fn get(
    JwtAuth(claims): JwtAuth,
    Path(id): Path<Uuid>,
    State(alicia): State<Arc<AliciaService>>,
) -> Result<Json<DeviceDto>, ApiError>;

/// POST /api/v1/alicia/devices/{id}/command
///
/// # Corps
/// ```json
/// { "action": "set_brightness", "value": 75 }
/// ```
///
/// # Reponse 200
/// ```json
/// { "success": true, "device_id": "...", "action": "set_brightness", "latency_ms": 45 }
/// ```
///
/// # Erreurs
/// - 404 si le dispositif est inconnu
/// - 422 si l'action n'est pas supportee par les capacites du dispositif
///
/// Auth : JWT scope Write
pub async fn command(
    JwtAuth(claims): JwtAuth,
    Path(id): Path<Uuid>,
    State(alicia): State<Arc<AliciaService>>,
    Json(req): Json<CommandRequest>,
) -> Result<Json<CommandResponse>, ApiError>;
```

---

## 11. Rate limiting

Implementation recommandee : middleware Tower custom ou crate `governor`.

```rust
// Middleware de rate limiting par IP
// Limite : config.rate_limit_per_min requetes par minute par adresse IP
// En cas de depassement : HTTP 429 avec header Retry-After

// Header de reponse en cas de 429 :
// Retry-After: <secondes restantes>
// X-RateLimit-Limit: <limite configuree>
// X-RateLimit-Remaining: <requetes restantes dans la fenetre>
// X-RateLimit-Reset: <timestamp UNIX de reinitialisation>
```

---

## 12. Tests attendus

### 12.1 Tests unitaires — `auth.rs`

```rust
// TC-API-01 : generation et verification token valide
#[test]
fn test_generate_and_verify_token_roundtrip() { ... }

// TC-API-02 : token expire retourne TokenExpired
#[test]
fn test_verify_expired_token_fails() { ... }

// TC-API-03 : signature incorrecte retourne Unauthorized
#[test]
fn test_verify_wrong_signature_fails() { ... }

// TC-API-04 : has_scope correct
#[test]
fn test_claims_has_scope() { ... }
```

### 12.2 Tests d'integration — `handlers/`

```rust
// TC-API-05 : POST /auth/token credentials valides -> 200 + token
#[tokio::test]
async fn test_auth_token_success() { ... }

// TC-API-06 : POST /auth/token credentials invalides -> 401
#[tokio::test]
async fn test_auth_token_invalid_secret() { ... }

// TC-API-07 : GET /state sans token -> 401
#[tokio::test]
async fn test_get_state_no_auth() { ... }

// TC-API-08 : GET /state avec token valide scope Read -> 200
#[tokio::test]
async fn test_get_state_authenticated() { ... }

// TC-API-09 : POST /devices/{id}/command token Read sans Write -> 403
#[tokio::test]
async fn test_command_insufficient_scope() { ... }

// TC-API-10 : POST /devices/{id}/command device inconnu -> 404
#[tokio::test]
async fn test_command_device_not_found() { ... }

// TC-API-11 : GET /health sans auth -> 200
#[tokio::test]
async fn test_health_no_auth() { ... }

// TC-API-12 : rate limiting > 100 req/min -> 429
#[tokio::test]
async fn test_rate_limiting_triggers_429() { ... }
```

---

## 13. Annotations MSCM — recap

| Fichier                         | @id                            | @layer | @role                   |
|---------------------------------|--------------------------------|--------|-------------------------|
| `lib.rs`                        | `service.alicia.rest-api`      | 7      | `http_api_gateway`      |
| `admin_cell.rs`                 | `service.alicia.api.admin`     | 7      | `governance_cell`       |
| `auth.rs`                       | (inline)                       | 7      | `jwt_authentication`    |
| `server.rs`                     | (inline)                       | 7      | `axum_server`           |
| `router.rs`                     | (inline)                       | 7      | `route_definition`      |
| `dto.rs`                        | (inline)                       | 7      | `data_transfer_objects` |
| `errors.rs`                     | (inline)                       | 7      | `error_types`           |
| `handlers/auth_handler.rs`      | (inline)                       | 7      | `auth_handler`          |
| `handlers/devices_handler.rs`   | (inline)                       | 7      | `device_command_handler`|

---

## 14. Securite et conformite

- **JWT HS256** : cle secrete minimum 32 octets (256 bits). Generee a la premiere installation
  (`OsRng::fill_bytes`), stockee chiffree dans KindMother. Jamais dans `alicia.toml` en clair.
- **Algorithme fixe** : validation jsonwebtoken configuree avec `Validation::new(Algorithm::HS256)`.
  Tout token presentant un algorithme different est rejete (defense contre "alg:none" attacks).
- **Pas de log des secrets** : `auth_credential`, `secret`, `token` sont marques
  `#[tracing::instrument(skip(...))]`. Jamais dans les traces.
- **Corps limite** : `RequestBodyLimitLayer` a 65 536 octets. Protege contre les body bombs.
- **`DeviceDto` sans credentials** : `config.auth_credential` n'est jamais serialise dans
  les reponses API. Verifie par un test dedie (TC-API-13).
- **Audit trail** : chaque commande reussie ou echouee est inseree dans `alicia_commands_log`
  avec source="api", source_detail=IP du client, via `AliciaService::log_command()`.
- **CORS** : desactive par defaut. Active uniquement si `cors_enabled = true` dans `alicia.toml`.
- **Port 7890** : a enregistrer dans le registre des ports COG (docs/reference/).

---

*Denis — Chef Dev Senior — Miyukini AI Studio — 2026-03-01*
