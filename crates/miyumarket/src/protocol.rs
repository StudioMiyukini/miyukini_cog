//! @id     miyumarket_protocol
//! @do     definir_market_response
//! @role   orchestration
//! @layer  core
//! @human  miyumarket — module « protocol ».

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

use crate::manifest::ServiceManifest;
use serde::{Deserialize, Serialize};

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
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
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

/// Usage (séparation de domaine) de la requête signée de publication.
pub const USAGE_PUBLICATION: &str = "origin/market-publication";
/// Usage (séparation de domaine) de la requête signée de retrait.
pub const USAGE_RETRAIT: &str = "origin/market-retrait";
/// Entête du corps de publication.
pub const MAGIC_CORPS_PUBLICATION: &[u8; 4] = b"MSPB";
/// Version du format du corps de publication.
pub const VERSION_CORPS_PUBLICATION: u8 = 0x01;
/// Taille maximale du manifeste JSON dans le corps de publication.
pub const TAILLE_MAX_MANIFESTE_PUBLICATION: usize = 64 * 1024;
const TAILLE_ENTETE_CORPS: usize = 4 + 1 + 4;

/// Corps binaire de `POST /api/market/publish` (plus de jeton : sortie du lot 2).
///
/// ```text
/// "MSPB" (4) | 0x01 (1) | longueur du manifeste u32 BE (4) | manifeste JSON | paquet signé (.msp « MSPS »)
/// ```
///
/// La requête entière est signée par la clé d'éditeur (en-têtes `X-Miyu-*`,
/// usage [`USAGE_PUBLICATION`]) et le paquet porte son propre sceau d'éditeur :
/// Origin exige que les deux signataires soient la même clé officielle épinglée.
pub fn encoder_corps_publication(manifest: &ServiceManifest, paquet_signe: &[u8]) -> Result<Vec<u8>, String> {
    let json = serde_json::to_vec(manifest).map_err(|e| format!("manifeste : {e}"))?;
    if json.len() > TAILLE_MAX_MANIFESTE_PUBLICATION {
        return Err("manifeste trop volumineux".to_string());
    }
    let longueur = u32::try_from(json.len()).map_err(|_| "manifeste trop volumineux".to_string())?;
    let mut corps = Vec::with_capacity(TAILLE_ENTETE_CORPS + json.len() + paquet_signe.len());
    corps.extend_from_slice(MAGIC_CORPS_PUBLICATION);
    corps.push(VERSION_CORPS_PUBLICATION);
    corps.extend_from_slice(&longueur.to_be_bytes());
    corps.extend_from_slice(&json);
    corps.extend_from_slice(paquet_signe);
    Ok(corps)
}

/// Décode strictement un corps de publication → `(manifeste, paquet signé)`.
pub fn decoder_corps_publication(corps: &[u8]) -> Result<(ServiceManifest, &[u8]), String> {
    if corps.len() < TAILLE_ENTETE_CORPS || &corps[..4] != MAGIC_CORPS_PUBLICATION {
        return Err("corps de publication invalide (entête)".to_string());
    }
    if corps[4] != VERSION_CORPS_PUBLICATION {
        return Err("version du corps de publication inconnue".to_string());
    }
    let longueur = u32::from_be_bytes([corps[5], corps[6], corps[7], corps[8]]) as usize;
    if longueur == 0 || longueur > TAILLE_MAX_MANIFESTE_PUBLICATION {
        return Err("longueur de manifeste hors bornes".to_string());
    }
    let fin = TAILLE_ENTETE_CORPS + longueur;
    let json = corps.get(TAILLE_ENTETE_CORPS..fin).ok_or_else(|| "manifeste tronqué".to_string())?;
    let manifest: ServiceManifest = serde_json::from_slice(json).map_err(|e| format!("manifeste JSON invalide : {e}"))?;
    let paquet = &corps[fin..];
    if paquet.is_empty() {
        return Err("paquet signé absent".to_string());
    }
    Ok((manifest, paquet))
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

#[cfg(test)]
mod tests {
    use super::*;

    fn manifeste() -> ServiceManifest {
        serde_json::from_value(serde_json::json!({
            "id": "svc", "name": "Svc", "description": "", "icon": "", "version": "1.0.0",
            "service_type": "interne_cog", "source": "officiel", "developer": "Miyukini",
            "min_central_version": "0.2.0"
        }))
        .unwrap()
    }

    #[test]
    fn corps_publication_aller_retour() {
        let corps = encoder_corps_publication(&manifeste(), b"MSPS paquet").unwrap();
        let (m, p) = decoder_corps_publication(&corps).unwrap();
        assert_eq!(m.id, "svc");
        assert_eq!(p, b"MSPS paquet");
    }

    #[test]
    fn corps_publication_malforme_refuse() {
        let corps = encoder_corps_publication(&manifeste(), b"x").unwrap();
        assert!(decoder_corps_publication(&corps[..corps.len() - 1]).is_err());
        assert!(decoder_corps_publication(b"MSPB").is_err());
        let mut v = corps.clone();
        v[4] = 2;
        assert!(decoder_corps_publication(&v).is_err());
        let mut v = corps;
        v[5] = 0xff;
        assert!(decoder_corps_publication(&v).is_err());
        assert!(decoder_corps_publication(b"{\"manifest\":{}}").is_err());
    }
}
