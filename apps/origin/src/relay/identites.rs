//! @id     origin.relay.identites
//! @do     epingler_cle_identite_cog
//! @role   securite
//! @layer  domain
//! @human  Registre TOFU des clés d'identité COG (`cog_id` → clé publique), partagé Relay/Tracker.
//!
//! Constat O-01 : la clé publique d'un `cog_id` est épinglée (confiance au premier usage)
//! **seulement une fois le COG entièrement vérifié** : preuve REGISTER valide, phases du
//! mode strict réussies, permis délivré. Une identité non prouvée ne consomme donc jamais
//! de place dans le registre. Une autre clé présentée ensuite pour ce `cog_id` est refusée.
//! Le `cog_id` étant l'empreinte de la clé, l'épinglage est une défense en profondeur
//! (et la base d'une future rotation signée).
//!
//! Persistance en **ajout seul** : une ligne `cog_id clé_hex` par épinglage, jamais de
//! réécriture complète du fichier par épinglage. L'ancien format (objet JSON) est relu
//! puis converti une seule fois au chargement.

use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Nombre maximal d'identités épinglées (C-13). Seuls les COG prouvés (permis délivré)
/// le consomment.
pub const MAX_IDENTITES: usize = 100_000;

/// Longueur maximale d'une ligne du fichier d'épinglage (C-13).
const MAX_LIGNE: usize = 256;

/// Refus d'épinglage (C-11).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefusEpinglage {
    /// Une autre clé est déjà épinglée pour ce `cog_id`.
    #[error("une autre clé est déjà épinglée pour ce cog_id")]
    CleDifferente,
    /// Registre plein.
    #[error("registre des identités plein")]
    RegistrePlein,
    /// Persistance impossible : l'épinglage n'est pas accordé (fail-closed, C-12).
    #[error("persistance de l'épinglage impossible")]
    Persistance,
}

/// Registre des clés épinglées.
#[derive(Debug, Clone, Default)]
pub struct RegistreIdentites {
    epingles: Arc<RwLock<HashMap<String, [u8; 32]>>>,
    chemin: Option<PathBuf>,
}

fn invalide(message: &'static str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, message)
}

fn decoder_cle(cle_hex: &str) -> Result<[u8; 32], std::io::Error> {
    hex::decode(cle_hex)
        .ok()
        .and_then(|v| v.try_into().ok())
        .ok_or_else(|| invalide("clé épinglée invalide"))
}

/// Ligne d'ajout pour un épinglage.
fn ligne(cog_id: &str, cle: &[u8; 32]) -> String {
    format!("{cog_id} {}\n", hex::encode(cle))
}

/// Réécrit le fichier complet (conversion de format ou réparation), de façon atomique.
fn reecrire(chemin: &Path, epingles: &HashMap<String, [u8; 32]>) -> Result<(), std::io::Error> {
    let mut contenu = String::with_capacity(epingles.len() * 92);
    for (cog_id, cle) in epingles {
        contenu.push_str(&ligne(cog_id, cle));
    }
    let tmp = chemin.with_extension("tmp");
    std::fs::write(&tmp, contenu)?;
    std::fs::rename(&tmp, chemin)
}

impl RegistreIdentites {
    /// Registre en mémoire seule.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registre persistant. Un fichier illisible est refusé (fail-closed) plutôt que
    /// remplacé silencieusement. Seule une dernière ligne incomplète (arrêt brutal pendant
    /// un ajout) est ignorée, et le fichier est alors réparé.
    pub fn charger(chemin: PathBuf) -> Result<Self, std::io::Error> {
        let mut epingles = HashMap::new();
        if chemin.exists() {
            let octets = std::fs::read(&chemin)?;
            let contenu = std::str::from_utf8(&octets).map_err(|_| invalide("fichier non UTF-8"))?;
            let a_reecrire = if contenu.trim_start().starts_with('{') {
                // Ancien format : objet JSON `{cog_id: clé_hex}`.
                let brut: HashMap<String, String> = serde_json::from_str(contenu)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                for (cog_id, cle_hex) in brut {
                    epingles.insert(cog_id, decoder_cle(&cle_hex)?);
                }
                true
            } else {
                let complet = contenu.is_empty() || contenu.ends_with('\n');
                let lignes: Vec<&str> = contenu.lines().collect();
                for (i, l) in lignes.iter().enumerate() {
                    let derniere_incomplete = !complet && i + 1 == lignes.len();
                    let analyse = (|| {
                        if l.len() > MAX_LIGNE {
                            return Err(invalide("ligne d'épinglage trop longue"));
                        }
                        let (cog_id, cle_hex) =
                            l.split_once(' ').ok_or_else(|| invalide("ligne d'épinglage invalide"))?;
                        Ok((cog_id.to_string(), decoder_cle(cle_hex)?))
                    })();
                    match analyse {
                        Ok((cog_id, cle)) => {
                            // Premier épinglage gagnant : une ligne ultérieure ne remplace rien.
                            epingles.entry(cog_id).or_insert(cle);
                        }
                        Err(_) if derniere_incomplete => {
                            warn!("Dernière ligne d'épinglage incomplète ignorée (arrêt brutal ?)");
                        }
                        Err(e) => return Err(e),
                    }
                }
                !complet
            };
            if epingles.len() > MAX_IDENTITES {
                return Err(invalide("trop d'identités épinglées"));
            }
            if a_reecrire {
                reecrire(&chemin, &epingles)?;
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

    /// Nombre d'identités épinglées.
    pub async fn nombre(&self) -> usize {
        self.epingles.read().await.len()
    }

    /// Vérifie, sans rien épingler, qu'une clé est compatible avec l'épinglage existant.
    pub async fn verifier_compatible(&self, cog_id: &str, cle: &[u8; 32]) -> Result<(), RefusEpinglage> {
        match self.epingles.read().await.get(cog_id) {
            Some(existante) if existante != cle => {
                warn!("Clé différente présentée pour le cog_id épinglé {}", cog_id);
                Err(RefusEpinglage::CleDifferente)
            }
            _ => Ok(()),
        }
    }

    /// Épingle `cle` pour `cog_id` au premier usage ; refuse une clé différente ensuite.
    ///
    /// À n'appeler qu'une fois l'identité entièrement vérifiée (permis délivré).
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
                    warn!("Registre des identités plein ({} entrées)", MAX_IDENTITES);
                    return Err(RefusEpinglage::RegistrePlein);
                }
                if let Some(chemin) = &self.chemin {
                    // Ajout seul d'une ligne : coût constant, indépendant de la taille du registre.
                    let resultat = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(chemin)
                        .and_then(|mut f| f.write_all(ligne(cog_id, cle).as_bytes()));
                    if let Err(e) = resultat {
                        warn!("Persistance de l'identité épinglée impossible : {}", e);
                        return Err(RefusEpinglage::Persistance);
                    }
                }
                epingles.insert(cog_id.to_string(), *cle);
                info!("Identité COG épinglée : {}", cog_id);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dossier_temp(nom: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "origin-identites-{nom}-{}",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

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
        assert_eq!(
            r.verifier_compatible("cog", &[2u8; 32]).await,
            Err(RefusEpinglage::CleDifferente)
        );
        assert!(r.verifier_compatible("autre", &[2u8; 32]).await.is_ok());
        // La vérification seule n'épingle rien.
        assert_eq!(r.cle_epinglee("autre").await, None);
    }

    #[tokio::test]
    async fn persistance_en_ajout_seul_rechargee() {
        let dir = dossier_temp("ajout");
        let chemin = dir.join("pins.txt");
        let r = RegistreIdentites::charger(chemin.clone()).unwrap();
        r.epingler("cog", &[3u8; 32]).await.unwrap();
        r.epingler("cog2", &[5u8; 32]).await.unwrap();
        let contenu = std::fs::read_to_string(&chemin).unwrap();
        assert_eq!(contenu.lines().count(), 2);
        assert!(contenu.starts_with(&format!("cog {}\n", hex::encode([3u8; 32]))));
        let r2 = RegistreIdentites::charger(chemin).unwrap();
        assert_eq!(r2.cle_epinglee("cog").await, Some([3u8; 32]));
        assert_eq!(r2.cle_epinglee("cog2").await, Some([5u8; 32]));
        assert_eq!(
            r2.epingler("cog", &[4u8; 32]).await,
            Err(RefusEpinglage::CleDifferente)
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[tokio::test]
    async fn ancien_format_json_converti() {
        let dir = dossier_temp("json");
        let chemin = dir.join("pins.json");
        std::fs::write(&chemin, format!("{{\"cog\":\"{}\"}}", hex::encode([6u8; 32]))).unwrap();
        let r = RegistreIdentites::charger(chemin.clone()).unwrap();
        assert_eq!(r.cle_epinglee("cog").await, Some([6u8; 32]));
        r.epingler("cog2", &[7u8; 32]).await.unwrap();
        let r2 = RegistreIdentites::charger(chemin).unwrap();
        assert_eq!(r2.cle_epinglee("cog").await, Some([6u8; 32]));
        assert_eq!(r2.cle_epinglee("cog2").await, Some([7u8; 32]));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[tokio::test]
    async fn derniere_ligne_tronquee_ignoree_mais_corruption_refusee() {
        let dir = dossier_temp("tronque");
        let chemin = dir.join("pins.txt");
        std::fs::write(
            &chemin,
            format!("cog {}\ncog2 abcd", hex::encode([8u8; 32])),
        )
        .unwrap();
        let r = RegistreIdentites::charger(chemin.clone()).unwrap();
        assert_eq!(r.cle_epinglee("cog").await, Some([8u8; 32]));
        assert_eq!(r.cle_epinglee("cog2").await, None);
        // Fichier réparé : un ajout ultérieur reste lisible.
        r.epingler("cog3", &[9u8; 32]).await.unwrap();
        let r2 = RegistreIdentites::charger(chemin.clone()).unwrap();
        assert_eq!(r2.cle_epinglee("cog3").await, Some([9u8; 32]));

        // Une ligne corrompue au milieu : refus (fail-closed).
        std::fs::write(&chemin, format!("bad\ncog {}\n", hex::encode([8u8; 32]))).unwrap();
        assert!(RegistreIdentites::charger(chemin).is_err());
        let _ = std::fs::remove_dir_all(dir);
    }
}
