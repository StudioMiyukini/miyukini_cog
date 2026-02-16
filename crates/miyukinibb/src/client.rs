//! Client API MiyukiniBB (Origin).
//!
//! Synchronise les profils Central vers la copie serveur pour l’auth forum.

use crate::errors::MiyukiniBbError;
use crate::types::ForumProfileSync;
use serde_json::json;

const SYNC_PATH: &str = "/api/auth/forum/sync";

/// Client pour l’API auth forum sur Origin.
#[derive(Debug, Clone)]
pub struct MiyukiniBbClient {
    /// URL de base d’Origin (ex. https://origin.miyukini.com).
    origin_base_url: String,
}

impl MiyukiniBbClient {
    /// Crée un client pointant vers l’Origin donné.
    ///
    /// `origin_base_url` ne doit pas se terminer par un `/`.
    pub fn new(origin_base_url: impl Into<String>) -> Result<Self, MiyukiniBbError> {
        let url = origin_base_url.into().trim().to_string();
        if url.is_empty() {
            return Err(MiyukiniBbError::InvalidOriginUrl);
        }
        let origin_base_url = url.trim_end_matches('/').to_string();
        Ok(Self { origin_base_url })
    }

    /// Synchronise un profil Central vers Origin (copie serveur pour le forum).
    ///
    /// À appeler après création ou mise à jour du profil (ou à la connexion)
    /// pour que l’utilisateur puisse se connecter au forum avec le même compte.
    pub fn sync_profile(&self, profile: &ForumProfileSync) -> Result<(), MiyukiniBbError> {
        if profile.central_id.is_empty() || profile.email.trim().is_empty() {
            return Err(MiyukiniBbError::InvalidProfile(
                "central_id et email requis".to_string(),
            ));
        }

        let url = format!("{}{}", self.origin_base_url, SYNC_PATH);
        let body = json!({
            "central_id": profile.central_id,
            "email": profile.email.trim(),
            "password_hash": profile.password_hash,
            "pseudonyme": profile.pseudonyme,
        });

        let response = ureq::post(&url)
            .set("Content-Type", "application/json")
            .send_json(body)
            .map_err(MiyukiniBbError::Http)?;

        let status = response.status();
        let body_str = response
            .into_string()
            .map_err(|e| MiyukiniBbError::Json(e.to_string()))?;

        if status != 200 {
            return Err(MiyukiniBbError::Api {
                status,
                body: body_str,
            });
        }

        let data: serde_json::Value =
            serde_json::from_str(&body_str).map_err(|e| MiyukiniBbError::Json(e.to_string()))?;
        if data.get("ok").and_then(|v| v.as_bool()) != Some(true) {
            return Err(MiyukiniBbError::Api {
                status: 200,
                body: body_str,
            });
        }

        Ok(())
    }
}
