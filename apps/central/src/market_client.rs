//! @id     central_market_client
//! @do     definir_market_client
//! @role   orchestration
//! @layer  app
//! @human  central — module « market client ».

//! Client Market -- Communication HTTPS avec Origin pour le catalogue et les packages.

use miyumarket::manifest::ServiceManifest;
use sha2::{Digest, Sha256};
use miyumarket::protocol::{
    encoder_corps_publication, MarketCatalog, MarketEntry, MarketResponse, MarketSearchResult,
    PublishResponse, UnpublishResponse, USAGE_PUBLICATION, USAGE_RETRAIT,
};
use miyauth::http::signer_requete;
use miyusceau::CleSignature;

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
    /// URL Origin refusée (schéma non sécurisé).
    InsecureUrl(String),
    /// Intégrité du package non vérifiable ou incorrecte (checksum).
    Integrity(String),
    /// Package sans enveloppe signée (Sceau v1 obligatoire, CA-01).
    NonSigne,
}

impl std::fmt::Display for MarketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Network(e) => write!(f, "Erreur réseau : {e}"),
            Self::Parse(e) => write!(f, "Erreur de parsing : {e}"),
            Self::Server(e) => write!(f, "Erreur serveur : {e}"),
            Self::InsecureUrl(e) => write!(f, "URL Origin refusée (HTTPS requis) : {e}"),
            Self::Integrity(e) => write!(f, "Intégrité du package refusée : {e}"),
            Self::NonSigne => write!(f, "Package non signé : signature d'éditeur obligatoire"),
        }
    }
}

/// Valide l'URL Origin : `https://` obligatoire. En build debug uniquement,
/// `http://127.0.0.1` et `http://localhost` sont tolérés pour le développement.
pub fn validate_origin_url(url: &str) -> Result<(), MarketError> {
    let lower = url.trim().to_ascii_lowercase();
    if lower.starts_with("https://") {
        return Ok(());
    }
    #[cfg(debug_assertions)]
    {
        for prefix in ["http://127.0.0.1", "http://localhost"] {
            if let Some(rest) = lower.strip_prefix(prefix) {
                // Hôte exact : fin de chaîne, port ou chemin (pas `localhost.evil.com`).
                if rest.is_empty() || rest.starts_with(':') || rest.starts_with('/') {
                    return Ok(());
                }
            }
        }
    }
    Err(MarketError::InsecureUrl(url.to_string()))
}

/// Identifiant/version utilisables dans un chemin d'URL (pas de séparateur ni `..`).
fn validate_path_segment(what: &str, v: &str) -> Result<(), MarketError> {
    let ok = !v.is_empty()
        && v.len() <= 64
        && v != ".."
        && v.chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '+'));
    if ok {
        Ok(())
    } else {
        Err(MarketError::Server(format!("{what} invalide : '{v}'")))
    }
}

/// Calcule le SHA-256 (hex minuscule) d'un bloc d'octets.
pub fn sha256_hex(data: &[u8]) -> String {
    use std::fmt::Write;
    let digest = Sha256::digest(data);
    let mut out = String::with_capacity(64);
    for b in digest {
        let _ = write!(out, "{b:02x}");
    }
    out
}

/// Vérifie le checksum SHA-256 attendu (fournit par le catalogue) sur les octets reçus.
/// Fail-closed : checksum absent/vide ou différent => refus.
pub fn verify_checksum(bytes: &[u8], expected: Option<&str>) -> Result<(), MarketError> {
    let expected = expected
        .map(|e| e.trim().to_ascii_lowercase())
        .filter(|e| !e.is_empty())
        .ok_or_else(|| MarketError::Integrity("checksum absent du catalogue".into()))?;
    let actual = sha256_hex(bytes);
    if actual == expected {
        Ok(())
    } else {
        Err(MarketError::Integrity(format!(
            "checksum incorrect (attendu {expected}, obtenu {actual})"
        )))
    }
}

/// Contrôle précoce : refuse au téléchargement un package sans enveloppe signée
/// (la vérification cryptographique complète, éditeur épinglé compris, est faite
/// par `service_manager::installer::verifier_paquet_market` avant toute écriture).
/// En build debug, `MIYUKINI_MARKET_DEV_NON_SIGNE=1` lève ce contrôle précoce.
pub fn require_signed_envelope(package_bytes: &[u8]) -> Result<(), MarketError> {
    if miyumarket::signature::est_paquet_signe(package_bytes)
        || crate::service_manager::confiance::dev_non_signe_autorise()
    {
        Ok(())
    } else {
        Err(MarketError::NonSigne)
    }
}

fn maintenant() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

impl MarketClient {
    /// Crée un nouveau client Market.
    /// Refuse toute URL Origin non HTTPS (hors localhost en build debug).
    pub fn new(origin_url: impl Into<String>) -> Result<Self, MarketError> {
        let origin_url = origin_url.into();
        validate_origin_url(&origin_url)?;
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            // Pas de redirection : évite un downgrade https -> http.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|e| MarketError::Network(format!("client HTTP Market : {e}")))?;
        Ok(Self {
            origin_url: origin_url.trim_end_matches('/').to_string(),
            client,
        })
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
    ///
    /// Vérifie avant de rendre les octets : cohérence id/version du catalogue
    /// avec la demande, checksum SHA-256 du catalogue (obligatoire), présence de
    /// l'enveloppe signée. La signature est vérifiée par l'installeur (CA-01).
    pub async fn download_package(
        &self,
        service_id: &str,
        version: &str,
    ) -> Result<Vec<u8>, MarketError> {
        validate_path_segment("Identifiant de service", service_id)?;
        validate_path_segment("Version", version)?;

        // Checksum de référence : issu du catalogue (pas du package lui-même).
        let entry = self.fetch_service(service_id).await?;
        if entry.manifest.id != service_id {
            return Err(MarketError::Integrity(format!(
                "le catalogue renvoie '{}' pour le service demandé '{service_id}'",
                entry.manifest.id
            )));
        }
        if entry.manifest.version != version {
            return Err(MarketError::Integrity(format!(
                "version du catalogue '{}' différente de la version demandée '{version}'",
                entry.manifest.version
            )));
        }

        let url = format!(
            "{}/api/market/package/{service_id}/{version}",
            self.origin_url
        );
        let bytes = self.http_get_bytes(&url).await?;
        verify_checksum(&bytes, entry.manifest.checksum.as_deref())?;
        require_signed_envelope(&bytes)?;
        Ok(bytes)
    }

    /// Publie un paquet signé sur le Market (éditeur officiel uniquement).
    ///
    /// Sortie du lot 2 (inventaire #14) : plus de jeton développeur. Le corps
    /// (`encoder_corps_publication` : manifeste + paquet `MSPS`) est envoyé dans une
    /// requête **signée par la clé d'éditeur** (`miyauth::http`, usage
    /// `origin/market-publication`) ; le paquet doit être scellé par la même clé
    /// (`miyumarket-signer`), épinglée côté Origin dans `[market] editeurs_officiels`.
    pub async fn publish(
        &self,
        manifest: &ServiceManifest,
        paquet_signe: &[u8],
        cle_editeur: &CleSignature,
    ) -> Result<PublishResponse, MarketError> {
        if !miyumarket::signature::est_paquet_signe(paquet_signe) {
            return Err(MarketError::NonSigne);
        }
        let chemin = "/api/market/publish";
        let corps = encoder_corps_publication(manifest, paquet_signe).map_err(MarketError::Server)?;
        let entetes = signer_requete(cle_editeur, USAGE_PUBLICATION, "POST", chemin, &corps, maintenant());
        let url = format!("{}{chemin}", self.origin_url);
        let mut requete = self
            .client
            .post(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/octet-stream");
        for (nom, valeur) in entetes.en_entetes() {
            requete = requete.header(nom, valeur);
        }
        let body = Self::envoyer(requete.body(corps), &url).await?;
        let resp: MarketResponse<PublishResponse> =
            serde_json::from_str(&body).map_err(|e| MarketError::Parse(e.to_string()))?;
        unwrap_response(resp, "Réponse vide de publication")
    }

    /// Retire une version publiée (requête signée par la clé d'éditeur, corps vide).
    pub async fn unpublish(
        &self,
        service_id: &str,
        version: &str,
        cle_editeur: &CleSignature,
    ) -> Result<UnpublishResponse, MarketError> {
        validate_path_segment("Identifiant de service", service_id)?;
        validate_path_segment("Version", version)?;
        let chemin = format!("/api/market/package/{service_id}/{version}");
        let entetes = signer_requete(cle_editeur, USAGE_RETRAIT, "DELETE", &chemin, b"", maintenant());
        let url = format!("{}{chemin}", self.origin_url);
        let mut requete = self.client.delete(&url);
        for (nom, valeur) in entetes.en_entetes() {
            requete = requete.header(nom, valeur);
        }
        let body = Self::envoyer(requete, &url).await?;
        let resp: MarketResponse<UnpublishResponse> =
            serde_json::from_str(&body).map_err(|e| MarketError::Parse(e.to_string()))?;
        unwrap_response(resp, "Réponse vide de retrait")
    }

    /// Envoie une requête d'écriture ; le message d'erreur du serveur est conservé.
    async fn envoyer(requete: reqwest::RequestBuilder, url: &str) -> Result<String, MarketError> {
        let response = requete
            .send()
            .await
            .map_err(|e| MarketError::Network(format!("{url}: {e}")))?;
        let statut = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| MarketError::Network(format!("Lecture réponse {url}: {e}")))?;
        if !statut.is_success() {
            let message = serde_json::from_str::<MarketResponse<serde_json::Value>>(&body)
                .ok()
                .and_then(|r| r.error)
                .unwrap_or_else(|| statut.to_string());
            return Err(MarketError::Server(format!("{statut} : {message}")));
        }
        Ok(body)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_http_refusee() {
        assert!(MarketClient::new("http://miyukini.com").is_err());
        assert!(MarketClient::new("ftp://miyukini.com").is_err());
        assert!(MarketClient::new("http://localhost.evil.com").is_err());
        assert!(MarketClient::new("https://miyukini.com").is_ok());
    }

    #[cfg(debug_assertions)]
    #[test]
    fn localhost_http_tolere_en_debug() {
        assert!(MarketClient::new("http://127.0.0.1:8080").is_ok());
        assert!(MarketClient::new("http://localhost:8080").is_ok());
    }

    #[test]
    fn checksum_incorrect_ou_absent_refuse() {
        let data = b"hello world";
        let good = sha256_hex(data);
        assert_eq!(
            good,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
        assert!(verify_checksum(data, Some(&good)).is_ok());
        assert!(verify_checksum(data, Some(&good.to_uppercase())).is_ok());
        assert!(verify_checksum(data, Some("deadbeef")).is_err());
        assert!(verify_checksum(data, Some("")).is_err());
        assert!(verify_checksum(data, None).is_err());
    }

    #[test]
    fn package_sans_enveloppe_signee_refuse() {
        if crate::service_manager::confiance::dev_non_signe_autorise() {
            return; // dérogation dev active dans l'environnement de test
        }
        assert!(matches!(
            require_signed_envelope(b"PK\x03\x04 zip nu"),
            Err(MarketError::NonSigne)
        ));
        assert!(require_signed_envelope(b"MSPS\x01....").is_ok());
    }

    #[tokio::test]
    async fn publication_sans_jeton_paquet_non_signe_refuse_avant_envoi() {
        let client = MarketClient::new("https://127.0.0.1:9").unwrap();
        let manifest: ServiceManifest = serde_json::from_value(serde_json::json!({
            "id": "svc", "name": "Svc", "description": "", "icon": "", "version": "1.0.0",
            "service_type": "interne_cog", "source": "officiel", "developer": "Miyukini",
            "min_central_version": "0.2.0"
        }))
        .unwrap();
        let cle = CleSignature::generer();
        assert!(matches!(
            client.publish(&manifest, b"PK\x03\x04 zip nu", &cle).await,
            Err(MarketError::NonSigne)
        ));
        assert!(client.unpublish("../x", "1.0.0", &cle).await.is_err());
    }
}
