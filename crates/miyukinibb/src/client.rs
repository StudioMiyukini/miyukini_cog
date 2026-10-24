//! @id     miyukinibb_client
//! @do     synchroniser_profil_signe
//! @role   orchestration
//! @layer  core
//! @human  miyukinibb — client de synchronisation des profils vers Origin, requête signée par la clé du COG.

//! Client API MiyukiniBB (Origin).
//!
//! Synchronise les profils du COG vers la copie serveur d'Origin (connexion forum par
//! mot de passe, désactivée par défaut côté Origin).
//!
//! Sortie du lot 2 (inventaire #13) : plus de secret partagé. Chaque requête est
//! **signée par la clé d'identité du COG** (`miyauth::http`, usage
//! [`USAGE_SYNC`]) : Origin vérifie que la clé est épinglée dans `[forum] sync_cles`
//! (`cog_id` = empreinte de la clé), l'horodatage (± 5 min) et le nonce (anti-rejeu).
//! Le corps signé est exactement celui envoyé.

use crate::errors::MiyukiniBbError;
use crate::types::ForumProfileSync;
use miyauth::http::signer_requete;
use miyusceau::CleSignature;
use serde_json::json;

/// Chemin de l'API de synchronisation (inclus dans la signature).
pub const SYNC_PATH: &str = "/api/auth/forum/sync";
/// Usage (séparation de domaine) de la requête signée, identique côté Origin.
pub const USAGE_SYNC: &str = "origin/forum-sync";

/// En-têtes HTTP `(nom, valeur)` d'une requête signée.
pub type EntetesSignees = [(&'static str, String); 4];

/// Client pour l'API auth forum sur Origin.
#[derive(Debug)]
pub struct MiyukiniBbClient {
    /// URL de base d'Origin (ex. https://origin.miyukini.com).
    origin_base_url: String,
    /// Clé d'identité du COG (graine de `IdentiteCog`) qui signe les requêtes.
    cle_cog: CleSignature,
}

fn maintenant() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

impl MiyukiniBbClient {
    /// Crée un client pointant vers l'Origin donné, signant avec la clé d'identité du COG.
    ///
    /// `origin_base_url` doit être en `https://` (hors `http://127.0.0.1` / `localhost`).
    pub fn new(origin_base_url: impl Into<String>, cle_cog: CleSignature) -> Result<Self, MiyukiniBbError> {
        let url = origin_base_url.into().trim().to_string();
        let minuscule = url.to_ascii_lowercase();
        let local = ["http://127.0.0.1", "http://localhost"].iter().any(|p| {
            minuscule
                .strip_prefix(p)
                .is_some_and(|reste| reste.is_empty() || reste.starts_with(':') || reste.starts_with('/'))
        });
        if url.is_empty() || !(minuscule.starts_with("https://") || local) {
            return Err(MiyukiniBbError::InvalidOriginUrl);
        }
        let origin_base_url = url.trim_end_matches('/').to_string();
        Ok(Self { origin_base_url, cle_cog })
    }

    /// Corps JSON et en-têtes signés d'une synchronisation (séparés pour les tests).
    pub fn requete_signee(
        &self,
        profile: &ForumProfileSync,
        horodatage: u64,
    ) -> Result<(Vec<u8>, EntetesSignees), MiyukiniBbError> {
        if profile.central_id.is_empty() || profile.email.trim().is_empty() {
            return Err(MiyukiniBbError::InvalidProfile("central_id et email requis".to_string()));
        }
        let corps = serde_json::to_vec(&json!({
            "central_id": profile.central_id,
            "email": profile.email.trim(),
            "password_hash": profile.password_hash,
            "pseudonyme": profile.pseudonyme,
        }))
        .map_err(|e| MiyukiniBbError::Json(e.to_string()))?;
        let entetes = signer_requete(&self.cle_cog, USAGE_SYNC, "POST", SYNC_PATH, &corps, horodatage).en_entetes();
        Ok((corps, entetes))
    }

    /// Synchronise un profil vers Origin (copie serveur pour le forum).
    pub fn sync_profile(&self, profile: &ForumProfileSync) -> Result<(), MiyukiniBbError> {
        let (corps, entetes) = self.requete_signee(profile, maintenant())?;
        let url = format!("{}{}", self.origin_base_url, SYNC_PATH);
        let mut requete = ureq::post(&url).set("Content-Type", "application/json");
        for (nom, valeur) in &entetes {
            requete = requete.set(nom, valeur);
        }
        let response = match requete.send_bytes(&corps) {
            Ok(r) => r,
            Err(ureq::Error::Status(status, r)) => {
                return Err(MiyukiniBbError::Api { status, body: r.into_string().unwrap_or_default() });
            }
            Err(e) => return Err(MiyukiniBbError::Http(Box::new(e))),
        };

        let status = response.status();
        let body_str = response.into_string().map_err(|e| MiyukiniBbError::Json(e.to_string()))?;
        if status != 200 {
            return Err(MiyukiniBbError::Api { status, body: body_str });
        }
        let data: serde_json::Value =
            serde_json::from_str(&body_str).map_err(|e| MiyukiniBbError::Json(e.to_string()))?;
        if data.get("ok").and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(MiyukiniBbError::Api { status: 200, body: body_str });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyauth::http::{verifier_requete_signee, EntetesSignees, RegistreNonces};

    fn profil() -> ForumProfileSync {
        ForumProfileSync::new("c1", "a@b.c", "$argon2id$v=19$m=19456,t=2,p=1$c2Fs$aGFzaA", Some("Pseudo"))
    }

    #[test]
    fn url_non_https_refusee() {
        for url in ["http://origin.miyukini.com", "", "http://localhost.evil.com", "ftp://x"] {
            assert!(MiyukiniBbClient::new(url, CleSignature::generer()).is_err(), "{url}");
        }
        assert!(MiyukiniBbClient::new("https://origin.miyukini.com/", CleSignature::generer()).is_ok());
        assert!(MiyukiniBbClient::new("http://127.0.0.1:8080", CleSignature::generer()).is_ok());
    }

    #[test]
    fn requete_signee_verifiable_par_origin() {
        let cle = CleSignature::generer();
        let cle_publique = cle.cle_publique();
        let client = MiyukiniBbClient::new("https://origin.miyukini.com", cle).unwrap();
        let (corps, entetes) = client.requete_signee(&profil(), 1_000).unwrap();
        let e = EntetesSignees::analyser(
            Some(&entetes[0].1),
            Some(&entetes[1].1),
            Some(&entetes[2].1),
            Some(&entetes[3].1),
        )
        .unwrap();
        assert_eq!(e.signataire, cle_publique);
        let nonces = RegistreNonces::default();
        assert!(verifier_requete_signee(&e, USAGE_SYNC, "POST", SYNC_PATH, &corps, &nonces, 1_000).is_ok());
        // Corps modifié : refus.
        let mut altere = corps.clone();
        altere[0] ^= 1;
        assert!(verifier_requete_signee(&e, USAGE_SYNC, "POST", SYNC_PATH, &altere, &nonces, 1_000).is_err());
    }

    #[test]
    fn profil_invalide_refuse() {
        let client = MiyukiniBbClient::new("https://o.example", CleSignature::generer()).unwrap();
        let mut p = profil();
        p.email = " ".into();
        assert!(client.requete_signee(&p, 0).is_err());
    }
}
