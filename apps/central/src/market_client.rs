//! Client Market — Communication avec Origin pour l'installation et la publication de services.
//!
//! Module préparé pour l'intégration UI du Service Market (Phase 2).
#![allow(dead_code)]
//!
//! # Utilisation
//!
//! ```rust,ignore
//! let client = MarketClient::new("https://origin.miyukini.com");
//! let catalog = client.fetch_catalog().await?;
//! let package = client.download_package("jaymanga", "0.1.0").await?;
//! ```

use miyumarket::manifest::ServiceManifest;
use miyumarket::protocol::{
    MarketCatalog, MarketEntry, MarketResponse, MarketSearchResult,
    PublishRequest, PublishResponse,
};

/// Client HTTP pour le Service Market d'Origin.
#[derive(Debug, Clone)]
pub struct MarketClient {
    /// URL de base du serveur Origin (ex: "http://127.0.0.1:8080").
    origin_url: String,
}

/// Erreurs du client Market.
#[derive(Debug)]
pub enum MarketError {
    /// Erreur réseau.
    Network(String),
    /// Erreur de parsing JSON.
    Parse(String),
    /// Le serveur a renvoyé une erreur.
    Server(String),
}

impl std::fmt::Display for MarketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Network(e) => write!(f, "Erreur réseau : {e}"),
            Self::Parse(e) => write!(f, "Erreur de parsing : {e}"),
            Self::Server(e) => write!(f, "Erreur serveur : {e}"),
        }
    }
}

impl MarketClient {
    /// Crée un nouveau client Market.
    pub fn new(origin_url: impl Into<String>) -> Self {
        Self { origin_url: origin_url.into() }
    }

    /// URL de base de l'Origin connecté.
    pub fn origin_url(&self) -> &str {
        &self.origin_url
    }

    /// Récupère le catalogue complet du Market.
    pub async fn fetch_catalog(&self) -> Result<MarketCatalog, MarketError> {
        let url = format!("{}/api/market/catalog", self.origin_url);
        let body = http_get(&url).await?;
        let resp: MarketResponse<MarketCatalog> =
            serde_json::from_str(&body).map_err(|e| MarketError::Parse(e.to_string()))?;

        if resp.success {
            resp.data.ok_or_else(|| MarketError::Server("Réponse vide".into()))
        } else {
            Err(MarketError::Server(resp.error.unwrap_or_default()))
        }
    }

    /// Récupère le détail d'un service.
    pub async fn fetch_service(&self, service_id: &str) -> Result<MarketEntry, MarketError> {
        let url = format!("{}/api/market/catalog/{service_id}", self.origin_url);
        let body = http_get(&url).await?;
        let resp: MarketResponse<MarketEntry> =
            serde_json::from_str(&body).map_err(|e| MarketError::Parse(e.to_string()))?;

        if resp.success {
            resp.data.ok_or_else(|| MarketError::Server("Service introuvable".into()))
        } else {
            Err(MarketError::Server(resp.error.unwrap_or_default()))
        }
    }

    /// Recherche dans le catalogue.
    pub async fn search(&self, query: &str) -> Result<MarketSearchResult, MarketError> {
        let encoded = query.replace(' ', "%20");
        let url = format!("{}/api/market/catalog/search?q={encoded}", self.origin_url);
        let body = http_get(&url).await?;
        let resp: MarketResponse<MarketSearchResult> =
            serde_json::from_str(&body).map_err(|e| MarketError::Parse(e.to_string()))?;

        if resp.success {
            resp.data.ok_or_else(|| MarketError::Server("Réponse vide".into()))
        } else {
            Err(MarketError::Server(resp.error.unwrap_or_default()))
        }
    }

    /// Télécharge le package d'un service.
    pub async fn download_package(&self, service_id: &str, version: &str) -> Result<Vec<u8>, MarketError> {
        let url = format!("{}/api/market/package/{service_id}/{version}", self.origin_url);
        http_get_bytes(&url).await
    }

    /// Publie un service sur le Market (pour les développeurs).
    pub async fn publish(&self, manifest: ServiceManifest, developer_token: &str) -> Result<PublishResponse, MarketError> {
        let url = format!("{}/api/market/publish", self.origin_url);
        let req = PublishRequest {
            manifest,
            developer_token: developer_token.to_string(),
        };
        let req_body = serde_json::to_string(&req)
            .map_err(|e| MarketError::Parse(e.to_string()))?;

        let body = http_post(&url, &req_body).await?;
        let resp: MarketResponse<PublishResponse> =
            serde_json::from_str(&body).map_err(|e| MarketError::Parse(e.to_string()))?;

        if resp.success {
            resp.data.ok_or_else(|| MarketError::Server("Réponse vide".into()))
        } else {
            Err(MarketError::Server(resp.error.unwrap_or_default()))
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HTTP minimal (sans dépendance lourde type reqwest)
// ═══════════════════════════════════════════════════════════════════════════
//
// Phase 1 : on utilise une implémentation TCP brute avec tokio.
// Phase 2 : migrer vers reqwest ou hyper si nécessaire.

/// GET HTTP basique — retourne le body en String.
async fn http_get(url: &str) -> Result<String, MarketError> {
    let bytes = http_get_bytes(url).await?;
    String::from_utf8(bytes).map_err(|e| MarketError::Parse(e.to_string()))
}

/// GET HTTP basique — retourne le body en bytes.
async fn http_get_bytes(url: &str) -> Result<Vec<u8>, MarketError> {
    let (host, port, path) = parse_url(url)?;
    let response = tcp_request(&host, port, "GET", &path, None).await?;
    extract_body(response)
}

/// POST HTTP basique — retourne le body en String.
async fn http_post(url: &str, body: &str) -> Result<String, MarketError> {
    let (host, port, path) = parse_url(url)?;
    let response = tcp_request(&host, port, "POST", &path, Some(body)).await?;
    let bytes = extract_body(response)?;
    String::from_utf8(bytes).map_err(|e| MarketError::Parse(e.to_string()))
}

/// Parse une URL en (host, port, path).
fn parse_url(url: &str) -> Result<(String, u16, String), MarketError> {
    let url = url.strip_prefix("http://").unwrap_or(url);
    let url = url.strip_prefix("https://").unwrap_or(url);

    let (host_port, path) = if let Some(idx) = url.find('/') {
        (&url[..idx], &url[idx..])
    } else {
        (url, "/")
    };

    let (host, port) = if let Some(idx) = host_port.rfind(':') {
        let port: u16 = host_port[idx + 1..].parse()
            .map_err(|_| MarketError::Network(format!("Port invalide dans URL: {host_port}")))?;
        (&host_port[..idx], port)
    } else {
        (host_port, 80u16)
    };

    Ok((host.to_string(), port, path.to_string()))
}

/// Envoie une requête TCP brute et retourne la réponse complète.
async fn tcp_request(host: &str, port: u16, method: &str, path: &str, body: Option<&str>) -> Result<Vec<u8>, MarketError> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let addr = format!("{host}:{port}");
    let mut stream = tokio::net::TcpStream::connect(&addr)
        .await
        .map_err(|e| MarketError::Network(format!("Connexion à {addr} impossible : {e}")))?;

    let mut request = format!("{method} {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n");
    if let Some(b) = body {
        request.push_str(&format!("Content-Type: application/json\r\nContent-Length: {}\r\n", b.len()));
    }
    request.push_str("\r\n");
    if let Some(b) = body {
        request.push_str(b);
    }

    stream.write_all(request.as_bytes())
        .await
        .map_err(|e| MarketError::Network(format!("Envoi requête : {e}")))?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)
        .await
        .map_err(|e| MarketError::Network(format!("Lecture réponse : {e}")))?;

    Ok(response)
}

/// Extrait le body d'une réponse HTTP brute (après \r\n\r\n).
fn extract_body(response: Vec<u8>) -> Result<Vec<u8>, MarketError> {
    let separator = b"\r\n\r\n";
    if let Some(pos) = response.windows(4).position(|w| w == separator) {
        // Vérifier le status code
        let header = String::from_utf8_lossy(&response[..pos]);
        if let Some(status_line) = header.lines().next() {
            if !status_line.contains("200") {
                return Err(MarketError::Server(format!("HTTP {status_line}")));
            }
        }
        Ok(response[pos + 4..].to_vec())
    } else {
        Err(MarketError::Parse("Réponse HTTP invalide (séparateur header/body manquant)".into()))
    }
}
