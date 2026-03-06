//! Client Market -- Communication HTTPS avec Origin pour le catalogue et les packages.

use miyumarket::manifest::ServiceManifest;
use miyumarket::protocol::{
    MarketCatalog, MarketEntry, MarketResponse, MarketSearchResult, PublishRequest, PublishResponse,
};

/// Client HTTP pour le Service Market d'Origin.
#[derive(Debug, Clone)]
pub struct MarketClient {
    /// URL de base du serveur Origin (ex: "https://miyukini.com").
    origin_url: String,
    client: reqwest::Client,
}

/// Erreurs du client Market.
#[derive(Debug)]
pub enum MarketError {
    /// Erreur réseau / HTTP.
    Network(String),
    /// Erreur de parsing JSON.
    Parse(String),
    /// Le serveur a renvoyé une erreur métier.
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
        Self {
            origin_url: origin_url.into().trim_end_matches('/').to_string(),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .expect("reqwest client for Market"),
        }
    }

    /// URL de base de l'Origin connecté.
    pub fn origin_url(&self) -> &str {
        &self.origin_url
    }

    /// Récupère le catalogue complet du Market.
    pub async fn fetch_catalog(&self) -> Result<MarketCatalog, MarketError> {
        let url = format!("{}/api/market/catalog", self.origin_url);
        let body = self.http_get_text(&url).await?;
        let resp: MarketResponse<MarketCatalog> =
            serde_json::from_str(&body).map_err(|e| MarketError::Parse(e.to_string()))?;
        unwrap_response(resp, "Réponse vide du catalogue")
    }

    /// Récupère le détail d'un service.
    pub async fn fetch_service(&self, service_id: &str) -> Result<MarketEntry, MarketError> {
        let url = format!("{}/api/market/catalog/{service_id}", self.origin_url);
        let body = self.http_get_text(&url).await?;
        let resp: MarketResponse<MarketEntry> =
            serde_json::from_str(&body).map_err(|e| MarketError::Parse(e.to_string()))?;
        unwrap_response(resp, "Service introuvable")
    }

    /// Recherche dans le catalogue.
    pub async fn search(&self, query: &str) -> Result<MarketSearchResult, MarketError> {
        let encoded = urlencoding::encode(query);
        let url = format!("{}/api/market/catalog/search?q={encoded}", self.origin_url);
        let body = self.http_get_text(&url).await?;
        let resp: MarketResponse<MarketSearchResult> =
            serde_json::from_str(&body).map_err(|e| MarketError::Parse(e.to_string()))?;
        unwrap_response(resp, "Réponse vide de recherche")
    }

    /// Télécharge le package d'un service.
    pub async fn download_package(
        &self,
        service_id: &str,
        version: &str,
    ) -> Result<Vec<u8>, MarketError> {
        let url = format!(
            "{}/api/market/package/{service_id}/{version}",
            self.origin_url
        );
        self.http_get_bytes(&url).await
    }

    /// Publie un service sur le Market (pour les développeurs).
    pub async fn publish(
        &self,
        manifest: ServiceManifest,
        developer_token: &str,
    ) -> Result<PublishResponse, MarketError> {
        let url = format!("{}/api/market/publish", self.origin_url);
        let req = PublishRequest {
            manifest,
            developer_token: developer_token.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .map_err(|e| MarketError::Network(format!("POST {url}: {e}")))?;

        let response = response
            .error_for_status()
            .map_err(|e| MarketError::Network(format!("POST {url}: {e}")))?;
        let body = response
            .text()
            .await
            .map_err(|e| MarketError::Network(format!("Lecture réponse {url}: {e}")))?;

        let resp: MarketResponse<PublishResponse> =
            serde_json::from_str(&body).map_err(|e| MarketError::Parse(e.to_string()))?;
        unwrap_response(resp, "Réponse vide de publication")
    }

    async fn http_get_text(&self, url: &str) -> Result<String, MarketError> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| MarketError::Network(format!("GET {url}: {e}")))?;
        let response = response
            .error_for_status()
            .map_err(|e| MarketError::Network(format!("GET {url}: {e}")))?;
        response
            .text()
            .await
            .map_err(|e| MarketError::Network(format!("Lecture réponse {url}: {e}")))
    }

    async fn http_get_bytes(&self, url: &str) -> Result<Vec<u8>, MarketError> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| MarketError::Network(format!("GET {url}: {e}")))?;
        let response = response
            .error_for_status()
            .map_err(|e| MarketError::Network(format!("GET {url}: {e}")))?;
        let bytes = response
            .bytes()
            .await
            .map_err(|e| MarketError::Network(format!("Lecture binaire {url}: {e}")))?;
        Ok(bytes.to_vec())
    }
}

fn unwrap_response<T>(resp: MarketResponse<T>, empty_message: &str) -> Result<T, MarketError> {
    if resp.success {
        resp.data
            .ok_or_else(|| MarketError::Server(empty_message.to_string()))
    } else {
        Err(MarketError::Server(resp.error.unwrap_or_default()))
    }
}
