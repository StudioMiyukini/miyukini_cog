//! Protocole Market — Messages échangés entre Central (client) et Origin (serveur).
//!
//! # Endpoints
//!
//! | Méthode | Route                            | Description                                  |
//! |---------|----------------------------------|----------------------------------------------|
//! | GET     | /api/market/catalog              | Liste tous les services disponibles           |
//! | GET     | /api/market/catalog/{id}         | Détail d'un service                           |
//! | GET     | /api/market/catalog/search?q=... | Recherche dans le catalogue                   |
//! | GET     | /api/market/package/{id}/{ver}   | Télécharge le package d'un service            |
//! | POST    | /api/market/publish              | Publie un nouveau service (upload multipart)  |
//! | DELETE  | /api/market/package/{id}/{ver}   | Retire un package (développeur authentifié)   |

use serde::{Deserialize, Serialize};
use crate::manifest::ServiceManifest;

// ═══════════════════════════════════════════════════════════════════════════
// Réponses du serveur Market (Origin)
// ═══════════════════════════════════════════════════════════════════════════

/// Réponse générique enveloppée.
#[derive(Debug, Serialize, Deserialize)]
pub struct MarketResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> MarketResponse<T> {
    pub fn ok(data: T) -> Self {
        Self { success: true, data: Some(data), error: None }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self { success: false, data: None, error: Some(msg.into()) }
    }
}

/// Entrée du catalogue Market (réponse de GET /api/market/catalog).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketEntry {
    /// Manifeste complet du service.
    #[serde(flatten)]
    pub manifest: ServiceManifest,
    /// Date de publication sur Origin.
    pub published_at: String,
    /// Nombre de téléchargements.
    pub download_count: u64,
    /// Disponible au téléchargement (package présent sur le serveur).
    pub downloadable: bool,
}

/// Catalogue complet (réponse de GET /api/market/catalog).
#[derive(Debug, Serialize, Deserialize)]
pub struct MarketCatalog {
    /// Services officiels.
    pub official: Vec<MarketEntry>,
    /// Services tiers (communauté).
    pub community: Vec<MarketEntry>,
    /// Date de génération.
    pub generated_at: String,
}

/// Résultat de recherche (réponse de GET /api/market/catalog/search?q=...).
#[derive(Debug, Serialize, Deserialize)]
pub struct MarketSearchResult {
    pub results: Vec<MarketEntry>,
    pub query: String,
    pub total: usize,
}

// ═══════════════════════════════════════════════════════════════════════════
// Requêtes vers le serveur Market (Origin)
// ═══════════════════════════════════════════════════════════════════════════

/// Requête de publication (body JSON accompagnant le POST multipart).
#[derive(Debug, Serialize, Deserialize)]
pub struct PublishRequest {
    /// Manifeste du service (lu depuis service.manifest.json dans le package).
    pub manifest: ServiceManifest,
    /// Token d'authentification du développeur.
    pub developer_token: String,
}

/// Réponse de publication.
#[derive(Debug, Serialize, Deserialize)]
pub struct PublishResponse {
    /// ID du service publié.
    pub service_id: String,
    /// Version publiée.
    pub version: String,
    /// Hash SHA-256 du package stocké.
    pub checksum: String,
    /// Taille du package en octets.
    pub package_size: u64,
}

/// Réponse de suppression de package.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnpublishResponse {
    pub service_id: String,
    pub version: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// Routes helpers
// ═══════════════════════════════════════════════════════════════════════════

/// Préfixe des routes Market sur Origin.
pub const MARKET_API_PREFIX: &str = "/api/market";

/// Construit l'URL du catalogue.
pub fn catalog_url(origin_base: &str) -> String {
    format!("{origin_base}{MARKET_API_PREFIX}/catalog")
}

/// Construit l'URL d'un service dans le catalogue.
pub fn service_url(origin_base: &str, service_id: &str) -> String {
    format!("{origin_base}{MARKET_API_PREFIX}/catalog/{service_id}")
}

/// Construit l'URL de recherche.
pub fn search_url(origin_base: &str, query: &str) -> String {
    let encoded = urlencoding(query);
    format!("{origin_base}{MARKET_API_PREFIX}/catalog/search?q={encoded}")
}

/// Construit l'URL de téléchargement d'un package.
pub fn package_download_url(origin_base: &str, service_id: &str, version: &str) -> String {
    format!("{origin_base}{MARKET_API_PREFIX}/package/{service_id}/{version}")
}

/// Construit l'URL de publication.
pub fn publish_url(origin_base: &str) -> String {
    format!("{origin_base}{MARKET_API_PREFIX}/publish")
}

/// Encodage URL minimal pour les query strings.
fn urlencoding(s: &str) -> String {
    s.replace(' ', "%20")
     .replace('&', "%26")
     .replace('=', "%3D")
     .replace('?', "%3F")
}
