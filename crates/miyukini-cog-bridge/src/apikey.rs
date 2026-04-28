//! Mode Clé API — authentification par token Bearer.
//!
//! Le Central Desktop génère un token signé, affiché en QR code.
//! L'utilisateur scanne le QR depuis l'app Android.
//! Le token est envoyé en header `Authorization: Bearer <token>` à chaque requête.

use crate::{BridgeError, BridgeResult};

/// Établit une connexion au COG Host via clé API.
pub async fn connect(host: &str, port: u16, token: &str) -> BridgeResult<()> {
    tracing::info!("API Key: validation du token sur {host}:{port}");

    let url = format!("http://{host}:{port}/api/bridge/validate-token");

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {token}"))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .map_err(|e| BridgeError::NetworkError(format!("API Key: {e}")))?;

    if resp.status().is_success() {
        tracing::info!("API Key: token valide");
        Ok(())
    } else if resp.status().as_u16() == 401 {
        Err(BridgeError::InvalidApiKey)
    } else {
        Err(BridgeError::ConnectionRefused)
    }
}
