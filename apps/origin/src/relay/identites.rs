//! @id     origin.relay.identites
//! @do     epingler_cle_identite_cog
//! @role   securite
//! @layer  domain
//! @human  Registre TOFU des clés d'identité COG (`cog_id` → clé publique), partagé Relay/Tracker.
//!
//! Constat O-01 : la première preuve REGISTER valide d'un `cog_id` épingle sa clé publique
//! (confiance au premier usage). Une autre clé présentée ensuite pour ce `cog_id` est refusée.
//! Le `cog_id` étant l'empreinte de la clé, l'épinglage est une défense en profondeur
//! (et la base d'une future rotation signée).

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Nombre maximal d'identités épinglées (C-13).
pub const MAX_IDENTITES: usize = 1_000_000;

/// Refus d'épinglage (C-11).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefusEpinglage {
    /// Une autre clé est déjà épinglée pour ce `cog_id`.
    #[error("une autre clé est déjà épinglée pour ce cog_id")]
    CleDifferente,
    /// Registre plein.
    #[error("registre des identités plein")]
    RegistrePlein,
}

/// Registre des clés épinglées.
#[derive(Debug, Clone, Default)]
pub struct RegistreIdentites {
    epingles: Arc<RwLock<HashMap<String, [u8; 32]>>>,
    chemin: Option<PathBuf>,
}

impl RegistreIdentites {
    /// Registre en mémoire seule.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registre persistant (JSON `{cog_id: clé_hex}`). Un fichier illisible est refusé
    /// (fail-closed) plutôt que remplacé silencieusement.
    pub fn charger(chemin: PathBuf) -> Result<Self, std::io::Error> {
        let mut epingles = HashMap::new();
        if chemin.exists() {
            let contenu = std::fs::read(&chemin)?;
            let brut: HashMap<String, String> = serde_json::from_slice(&contenu)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            for (cog_id, cle_hex) in brut {
                let cle: [u8; 32] = hex::decode(&cle_hex)
                    .ok()
                    .and_then(|v| v.try_into().ok())
                    .ok_or_else(|| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "clé épinglée invalide")
                    })?;
                epingles.insert(cog_id, cle);
            }
            info!("{} identité(s) COG épinglée(s) chargée(s)", epingles.len());
        }
        Ok(Self {
            epingles: Arc::new(RwLock::new(epingles)),
            chemin: Some(chemin),
        })
    }

    /// Clé épinglée pour `cog_id`.
    pub async fn cle_epinglee(&self, cog_id: &str) -> Option<[u8; 32]> {
        self.epingles.read().await.get(cog_id).copied()
    }

    /// Épingle `cle` pour `cog_id` au premier usage ; refuse une clé différente ensuite.
    pub async fn epingler(&self, cog_id: &str, cle: &[u8; 32]) -> Result<(), RefusEpinglage> {
        let mut epingles = self.epingles.write().await;
        match epingles.get(cog_id) {
            Some(existante) if existante == cle => Ok(()),
            Some(_) => {
                warn!("Clé différente présentée pour le cog_id épinglé {}", cog_id);
                Err(RefusEpinglage::CleDifferente)
            }
            None => {
                if epingles.len() >= MAX_IDENTITES {
                    return Err(RefusEpinglage::RegistrePlein);
                }
                epingles.insert(cog_id.to_string(), *cle);
                if let Some(chemin) = &self.chemin {
                    let brut: HashMap<&String, String> =
                        epingles.iter().map(|(k, v)| (k, hex::encode(v))).collect();
                    match serde_json::to_vec(&brut) {
                        Ok(octets) => {
                            let tmp = chemin.with_extension("tmp");
                            if let Err(e) = std::fs::write(&tmp, octets)
                                .and_then(|()| std::fs::rename(&tmp, chemin))
                            {
                                warn!("Persistance des identités épinglées impossible : {}", e);
                            }
                        }
                        Err(e) => warn!("Sérialisation des identités impossible : {}", e),
                    }
                }
                info!("Identité COG épinglée : {}", cog_id);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn premier_usage_epingle_puis_refuse_autre_cle() {
        let r = RegistreIdentites::new();
        assert!(r.epingler("cog", &[1u8; 32]).await.is_ok());
        assert!(r.epingler("cog", &[1u8; 32]).await.is_ok());
        assert_eq!(
            r.epingler("cog", &[2u8; 32]).await,
            Err(RefusEpinglage::CleDifferente)
        );
        assert_eq!(r.cle_epinglee("cog").await, Some([1u8; 32]));
    }

    #[tokio::test]
    async fn persistance_rechargee() {
        let dir = std::env::temp_dir().join(format!(
            "origin-identites-{}",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let chemin = dir.join("pins.json");
        let r = RegistreIdentites::charger(chemin.clone()).unwrap();
        r.epingler("cog", &[3u8; 32]).await.unwrap();
        let r2 = RegistreIdentites::charger(chemin).unwrap();
        assert_eq!(r2.cle_epinglee("cog").await, Some([3u8; 32]));
        assert_eq!(
            r2.epingler("cog", &[4u8; 32]).await,
            Err(RefusEpinglage::CleDifferente)
        );
        let _ = std::fs::remove_dir_all(dir);
    }
}
