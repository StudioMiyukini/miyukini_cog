//! @id     central_service_manager_confiance
//! @do     charger_editeurs_confiance
//! @role   securite
//! @layer  app
//! @human  central — éditeurs Market de confiance épinglés et dérogation dev non signée.

//! Éditeurs de confiance du Market (CA-01, EX-09, EX-26).
//!
//! Les clés publiques Ed25519 acceptées pour signer un paquet viennent de :
//! 1. [`EDITEURS_OFFICIELS`] — clés compilées dans Central (bundle) ;
//! 2. `market-editeurs.json` dans le dossier de configuration de Central :
//!    `{ "editeurs": [ { "nom": "…", "cle_publique": "<64 hex>" } ] }`.
//!
//! Sans aucune clé, toute installation est refusée (fail-closed).
//!
//! **Dérogation de développement** : en build debug uniquement, la variable
//! `MIYUKINI_MARKET_DEV_NON_SIGNE=1` autorise un paquet non signé. Chaque usage
//! est journalisé en erreur. En release, la variable est ignorée.

use miyumarket::signature::cle_publique_depuis_hex;
use miyusceau::{AncresDeConfiance, NiveauConfiance};
use serde::Deserialize;
use std::path::Path;

/// Fichier des éditeurs épinglés (dossier de configuration Central).
pub const FICHIER_EDITEURS: &str = "market-editeurs.json";

/// Variable de dérogation (debug uniquement).
pub const VAR_DEV_NON_SIGNE: &str = "MIYUKINI_MARKET_DEV_NON_SIGNE";

/// Clés publiques (hex) des éditeurs officiels compilées dans Central.
///
/// Vide tant que la clé d'émission d'Origin (lot 2, racine SLIP-39) n'est pas
/// produite : l'installation exige alors une clé épinglée dans
/// `market-editeurs.json`.
pub const EDITEURS_OFFICIELS: &[(&str, &str)] = &[];

#[derive(Debug, Deserialize)]
struct FichierEditeurs {
    #[serde(default)]
    editeurs: Vec<EntreeEditeur>,
}

#[derive(Debug, Deserialize)]
struct EntreeEditeur {
    #[serde(default)]
    nom: String,
    cle_publique: String,
}

/// Identifiants de service réservés aux éditeurs officiels (bundle) : ils
/// reçoivent des secrets du COG. Un éditeur ajouté localement ne peut ni les
/// installer ni les lancer.
pub const IDS_RESERVES_OFFICIELS: &[&str] = &["miyucloud"];

/// Vrai si l'identifiant est réservé aux éditeurs officiels.
pub fn id_reserve_officiel(id: &str) -> bool {
    IDS_RESERVES_OFFICIELS.contains(&id)
}

/// Ajoute une clé hex aux ancres ; une clé invalide est ignorée et journalisée.
/// Éditeurs officiels (bundle) : `Souverain` ; éditeurs ajoutés localement :
/// `Reconnu`.
fn epingler(ancres: &mut AncresDeConfiance, nom: &str, hex: &str, niveau: NiveauConfiance) {
    if let Some(cle) = cle_publique_depuis_hex(hex) {
        if ancres.ajouter(&cle, niveau).is_err() {
            tracing::warn!("Éditeur Market '{nom}' : clé publique refusée (faible ou invalide)");
        }
    } else {
        tracing::warn!("Éditeur Market '{nom}' : clé publique mal formée (64 hex attendus)");
    }
}

/// Charge les ancres depuis le bundle et un fichier explicite.
pub fn charger_ancres_depuis(fichier: Option<&Path>) -> AncresDeConfiance {
    let mut ancres = AncresDeConfiance::new();
    for (nom, hex) in EDITEURS_OFFICIELS {
        epingler(&mut ancres, nom, hex, NiveauConfiance::Souverain);
    }
    if let Some(chemin) = fichier {
        match std::fs::read_to_string(chemin) {
            Ok(texte) => match serde_json::from_str::<FichierEditeurs>(&texte) {
                Ok(f) => {
                    for e in &f.editeurs {
                        epingler(&mut ancres, &e.nom, &e.cle_publique, NiveauConfiance::Reconnu);
                    }
                }
                Err(e) => tracing::warn!("{} illisible : {e}", chemin.display()),
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => tracing::warn!("{} illisible : {e}", chemin.display()),
        }
    }
    ancres
}

/// Ancres de confiance de Central (bundle + fichier de configuration).
pub fn ancres_market() -> AncresDeConfiance {
    let fichier = miyukini_central::chemins::dossier_config()
        .ok()
        .map(|d| d.join(FICHIER_EDITEURS));
    charger_ancres_depuis(fichier.as_deref())
}

/// Vrai si la dérogation « paquet non signé » est active (debug + variable = 1).
/// Toujours faux en release.
pub fn dev_non_signe_autorise() -> bool {
    #[cfg(debug_assertions)]
    {
        std::env::var(VAR_DEV_NON_SIGNE).is_ok_and(|v| v.trim() == "1")
    }
    #[cfg(not(debug_assertions))]
    {
        false
    }
}

/// Horloge Unix (secondes) pour la fenêtre de validité des sceaux.
pub fn maintenant_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyumarket::signature::hex_encoder;
    use miyusceau::CleSignature;

    #[test]
    fn fichier_absent_ou_vide_aucune_ancre() {
        let a = charger_ancres_depuis(Some(Path::new("/chemin/inexistant/market-editeurs.json")));
        assert_eq!(a.len(), EDITEURS_OFFICIELS.len());
        assert_eq!(charger_ancres_depuis(None).len(), EDITEURS_OFFICIELS.len());
    }

    #[test]
    fn fichier_editeurs_charge_et_cle_invalide_ignoree() {
        let cle = CleSignature::generer();
        let dir = std::env::temp_dir().join(format!("miyu-editeurs-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join(FICHIER_EDITEURS);
        std::fs::write(
            &f,
            format!(
                r#"{{"editeurs":[{{"nom":"dev","cle_publique":"{}"}},{{"nom":"cassee","cle_publique":"zz"}}]}}"#,
                hex_encoder(&cle.cle_publique())
            ),
        )
        .unwrap();
        let a = charger_ancres_depuis(Some(&f));
        assert_eq!(a.len(), EDITEURS_OFFICIELS.len() + 1);
        let _ = std::fs::remove_dir_all(dir);
    }
}
