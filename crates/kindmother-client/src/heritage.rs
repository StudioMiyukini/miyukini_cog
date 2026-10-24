//! @id     kindmother_client_heritage
//! @do     refuser_base_heritee_non_importee
//! @role   securite
//! @layer  core
//! @human  Garde de bascule K-13 : un service passé sous KindMother refuse de démarrer
//!         tant que sa base SQLite héritée (non vide) n'a pas été importée.

//! Garde de bascule vers KindMother (K-13).
//!
//! - Les services passés en `kindmother-only` ignorent le chemin de leur
//!   ancienne base SQLite. Sans import, ils démarreraient sur une base vide
//!   alors que les données de l'utilisateur sont encore dans l'ancien fichier.
//! - Refus explicite (fail-closed) si le fichier hérité existe et n'est pas
//!   vide, sauf marqueur `<fichier>.importe-kindmother` posé par l'outil
//!   d'import (ou par l'administrateur après import ou abandon volontaire).
//! - Le fichier hérité n'est jamais modifié ni supprimé.

use std::path::{Path, PathBuf};

/// Suffixe du marqueur « base héritée importée ».
pub const SUFFIXE_MARQUEUR_IMPORT: &str = "importe-kindmother";

/// Chemin du marqueur d'import associé à une base héritée.
#[must_use]
pub fn chemin_marqueur_import(base_heritee: &Path) -> PathBuf {
    let mut nom = base_heritee.as_os_str().to_os_string();
    nom.push(".");
    nom.push(SUFFIXE_MARQUEUR_IMPORT);
    PathBuf::from(nom)
}

/// Refuse si `base_heritee` est un fichier non vide sans marqueur d'import.
///
/// # Errors
/// Message explicite à remonter tel quel à l'utilisateur.
pub fn refuser_si_base_heritee(base_heritee: &Path) -> Result<(), String> {
    let non_vide = std::fs::metadata(base_heritee).is_ok_and(|m| m.is_file() && m.len() > 0);
    if !non_vide || chemin_marqueur_import(base_heritee).exists() {
        return Ok(());
    }
    Err(format!(
        "base héritée {} présente et non importée dans KindMother (K-13) : \
         démarrage refusé pour ne pas présenter une base vide. Importer les données, \
         ou créer {} pour confirmer l'abandon volontaire (le fichier hérité n'est jamais touché).",
        base_heritee.display(),
        chemin_marqueur_import(base_heritee).display()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn garde_de_bascule() {
        let d = std::env::temp_dir().join(format!("km-heritage-{}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        let base = d.join("service.db");
        // Absente ou vide : accepté.
        assert!(refuser_si_base_heritee(&base).is_ok());
        std::fs::write(&base, b"").unwrap();
        assert!(refuser_si_base_heritee(&base).is_ok());
        // Non vide sans marqueur : refusé.
        std::fs::write(&base, b"SQLite format 3\0").unwrap();
        assert!(refuser_si_base_heritee(&base).is_err());
        // Marqueur posé : accepté, fichier hérité intact.
        std::fs::write(chemin_marqueur_import(&base), b"").unwrap();
        assert!(refuser_si_base_heritee(&base).is_ok());
        assert_eq!(std::fs::read(&base).unwrap(), b"SQLite format 3\0");
        let _ = std::fs::remove_dir_all(&d);
    }
}
