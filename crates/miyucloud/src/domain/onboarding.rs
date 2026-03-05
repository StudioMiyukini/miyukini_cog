//! Wizard d'onboarding premiere connexion MiyuCloud.
//!
//! @id: miyucloud_domain_onboarding
//! @do: provide_onboarding_status_checks_completion
//! @role: domain
//! @layer: domain
//!
//! Gere le flux de premiere connexion : verification passphrase,
//! configuration 2FA, verification stockage, completion.

use crate::errors::MiyucloudError;

#[cfg(feature = "legacy-sqlite")]
use crate::data::MiyucloudDb;

use crate::data::types::OnboardingRow;

/// Statut de l'onboarding.
pub struct OnboardingStatus {
    /// Etape courante du wizard (0-3).
    pub current_step: u32,
    /// La passphrase a ete definie.
    pub passphrase_set: bool,
    /// Le TOTP 2FA a ete configure.
    pub totp_set: bool,
    /// Le stockage a ete verifie.
    pub storage_verified: bool,
    /// L'onboarding est termine.
    pub completed: bool,
    /// Date de completion (ISO 8601).
    pub completed_at: Option<String>,
}

impl OnboardingStatus {
    /// Retourne un statut par defaut (premiere etape, rien de fait).
    fn default_new() -> Self {
        Self {
            current_step: 1,
            passphrase_set: false,
            totp_set: false,
            storage_verified: false,
            completed: false,
            completed_at: None,
        }
    }

    /// Construit un `OnboardingStatus` depuis un `OnboardingRow`.
    fn from_row(row: &OnboardingRow) -> Self {
        Self {
            current_step: row.current_step,
            passphrase_set: row.passphrase_set,
            totp_set: row.totp_set,
            storage_verified: row.storage_verified,
            completed: row.completed,
            completed_at: row.completed_at.clone(),
        }
    }
}

/// Retourne le statut de l'onboarding pour un proprietaire.
///
/// Si aucun enregistrement n'existe en base, retourne un statut par defaut
/// (etape 1, tous les booleens a faux).
#[cfg(feature = "legacy-sqlite")]
pub fn get_status(db: &MiyucloudDb, owner_id: &str) -> Result<OnboardingStatus, MiyucloudError> {
    match db.onboarding_get(owner_id)? {
        Some(row) => Ok(OnboardingStatus::from_row(&row)),
        None => Ok(OnboardingStatus::default_new()),
    }
}

/// Verifie que la passphrase est definie et correcte.
///
/// Retourne `true` si la configuration crypto existe ET que le canary
/// chiffre est present (preuve que la passphrase a ete definie et verifiee).
#[cfg(feature = "legacy-sqlite")]
pub fn check_passphrase(db: &MiyucloudDb, owner_id: &str) -> Result<bool, MiyucloudError> {
    match db.crypto_config_get(owner_id)? {
        Some(cfg) => Ok(cfg.canary_ciphertext.is_some()),
        None => Ok(false),
    }
}

/// Verifie que le 2FA est configure.
///
/// Delegue a `crate::auth::totp::is_totp_enabled` qui verifie si un secret
/// TOTP actif existe pour le proprietaire.
#[cfg(feature = "legacy-sqlite")]
pub fn check_2fa(db: &MiyucloudDb, owner_id: &str) -> Result<bool, MiyucloudError> {
    crate::auth::totp::is_totp_enabled(db, owner_id)
}

/// Verifie que le stockage est accessible et utilisable.
///
/// Effectue trois controles :
/// 1. Le chemin existe.
/// 2. Le chemin est un repertoire.
/// 3. Le repertoire est accessible en ecriture (creation + suppression d'un fichier temporaire).
pub fn verify_storage(storage_path: &str) -> Result<bool, MiyucloudError> {
    let path = std::path::Path::new(storage_path);

    // Le chemin doit exister
    if !path.exists() {
        return Ok(false);
    }

    // Le chemin doit etre un repertoire
    if !path.is_dir() {
        return Ok(false);
    }

    // Verifier l'acces en ecriture avec un fichier temporaire
    let probe = path.join(".miyucloud_write_probe");
    match std::fs::write(&probe, b"probe") {
        Ok(()) => {
            // Nettoyage : supprimer le fichier sonde
            let _ = std::fs::remove_file(&probe);
            Ok(true)
        }
        Err(_) => Ok(false),
    }
}

/// Marque l'onboarding comme termine.
///
/// Recupere l'etat courant (ou les valeurs par defaut) puis met a jour
/// `completed = true` et `current_step = 4` (toutes les etapes passees).
/// Les champs `passphrase_set`, `totp_set` et `storage_verified` sont
/// preserves depuis l'etat courant.
#[cfg(feature = "legacy-sqlite")]
pub fn complete(db: &MiyucloudDb, owner_id: &str) -> Result<(), MiyucloudError> {
    let status = get_status(db, owner_id)?;
    let now = chrono::Utc::now().to_rfc3339();
    let row = OnboardingRow {
        owner_id: owner_id.to_string(),
        current_step: 4,
        passphrase_set: status.passphrase_set,
        totp_set: status.totp_set,
        storage_verified: status.storage_verified,
        completed: true,
        completed_at: Some(now),
    };
    db.onboarding_upsert(&row)
}

/// Reinitialise l'onboarding (admin uniquement).
///
/// Supprime l'enregistrement d'onboarding en base. Au prochain appel a
/// `get_status`, un statut par defaut sera retourne.
#[cfg(feature = "legacy-sqlite")]
pub fn reset(db: &MiyucloudDb, owner_id: &str) -> Result<(), MiyucloudError> {
    db.onboarding_delete(owner_id)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[cfg(feature = "legacy-sqlite")]
mod tests {
    use super::*;
    use crate::data::types::CryptoConfig;
    use crate::data::MiyucloudDb;

    /// Ouvre une base en memoire pour les tests.
    fn mem_db() -> MiyucloudDb {
        MiyucloudDb::open(":memory:").expect("in-memory db")
    }

    // -- get_status -----------------------------------------------------------

    #[test]
    fn test_get_status_default_when_no_record() {
        let db = mem_db();
        let status = get_status(&db, "owner-new").expect("should not error");
        assert_eq!(status.current_step, 1);
        assert!(!status.passphrase_set);
        assert!(!status.totp_set);
        assert!(!status.storage_verified);
        assert!(!status.completed);
        assert!(status.completed_at.is_none());
    }

    #[test]
    fn test_get_status_returns_persisted_row() {
        let db = mem_db();
        let row = OnboardingRow {
            owner_id: "owner-1".to_string(),
            current_step: 2,
            passphrase_set: true,
            totp_set: false,
            storage_verified: false,
            completed: false,
            completed_at: None,
        };
        db.onboarding_upsert(&row).expect("upsert ok");

        let status = get_status(&db, "owner-1").expect("should not error");
        assert_eq!(status.current_step, 2);
        assert!(status.passphrase_set);
        assert!(!status.totp_set);
        assert!(!status.completed);
    }

    // -- check_passphrase -----------------------------------------------------

    #[test]
    fn test_check_passphrase_false_when_no_config() {
        let db = mem_db();
        let result = check_passphrase(&db, "owner-no-crypto").expect("should not error");
        assert!(!result);
    }

    #[test]
    fn test_check_passphrase_false_when_no_canary() {
        let db = mem_db();
        let now = chrono::Utc::now().to_rfc3339();
        let cfg = CryptoConfig {
            owner_id: "owner-no-canary".to_string(),
            argon2_salt: "c2FsdA==".to_string(),
            canary_ciphertext: None,
            canary_nonce: None,
            key_version: 1,
            created_at: now.clone(),
            updated_at: now,
        };
        db.crypto_config_upsert(&cfg).expect("upsert ok");

        let result = check_passphrase(&db, "owner-no-canary").expect("should not error");
        assert!(!result);
    }

    #[test]
    fn test_check_passphrase_true_when_canary_present() {
        let db = mem_db();
        let now = chrono::Utc::now().to_rfc3339();
        let cfg = CryptoConfig {
            owner_id: "owner-with-canary".to_string(),
            argon2_salt: "c2FsdA==".to_string(),
            canary_ciphertext: Some("encrypted_canary".to_string()),
            canary_nonce: Some("nonce_value".to_string()),
            key_version: 1,
            created_at: now.clone(),
            updated_at: now,
        };
        db.crypto_config_upsert(&cfg).expect("upsert ok");

        let result = check_passphrase(&db, "owner-with-canary").expect("should not error");
        assert!(result);
    }

    // -- verify_storage -------------------------------------------------------

    #[test]
    fn test_verify_storage_valid_dir() {
        let dir = tempfile::tempdir().expect("tempdir");
        let result =
            verify_storage(dir.path().to_str().expect("utf8 path")).expect("should not error");
        assert!(result);
    }

    #[test]
    fn test_verify_storage_nonexistent_path() {
        let result = verify_storage("/this/path/does/not/exist/at/all").expect("should not error");
        assert!(!result);
    }

    #[test]
    fn test_verify_storage_file_not_dir() {
        let dir = tempfile::tempdir().expect("tempdir");
        let file_path = dir.path().join("not_a_dir.txt");
        std::fs::write(&file_path, b"hello").expect("write file");

        let result =
            verify_storage(file_path.to_str().expect("utf8 path")).expect("should not error");
        assert!(!result);
    }

    // -- complete -------------------------------------------------------------

    #[test]
    fn test_complete_marks_onboarding_done() {
        let db = mem_db();
        // Pre-populate partial onboarding
        let row = OnboardingRow {
            owner_id: "owner-complete".to_string(),
            current_step: 3,
            passphrase_set: true,
            totp_set: true,
            storage_verified: true,
            completed: false,
            completed_at: None,
        };
        db.onboarding_upsert(&row).expect("upsert ok");

        complete(&db, "owner-complete").expect("complete ok");

        let status = get_status(&db, "owner-complete").expect("status ok");
        assert!(status.completed);
        assert_eq!(status.current_step, 4);
        assert!(status.completed_at.is_some());
        // Preserve existing flags
        assert!(status.passphrase_set);
        assert!(status.totp_set);
        assert!(status.storage_verified);
    }

    #[test]
    fn test_complete_from_default_state() {
        let db = mem_db();
        // No pre-existing record — complete from scratch
        complete(&db, "owner-fresh").expect("complete ok");

        let status = get_status(&db, "owner-fresh").expect("status ok");
        assert!(status.completed);
        assert_eq!(status.current_step, 4);
        assert!(status.completed_at.is_some());
        // Flags remain false since no prior steps were done
        assert!(!status.passphrase_set);
        assert!(!status.totp_set);
        assert!(!status.storage_verified);
    }

    // -- reset ----------------------------------------------------------------

    #[test]
    fn test_reset_clears_onboarding() {
        let db = mem_db();
        // Create a completed onboarding
        let row = OnboardingRow {
            owner_id: "owner-reset".to_string(),
            current_step: 4,
            passphrase_set: true,
            totp_set: true,
            storage_verified: true,
            completed: true,
            completed_at: Some("2026-03-04T00:00:00Z".to_string()),
        };
        db.onboarding_upsert(&row).expect("upsert ok");

        reset(&db, "owner-reset").expect("reset ok");

        // After reset, get_status should return defaults
        let status = get_status(&db, "owner-reset").expect("status ok");
        assert_eq!(status.current_step, 1);
        assert!(!status.passphrase_set);
        assert!(!status.totp_set);
        assert!(!status.storage_verified);
        assert!(!status.completed);
        assert!(status.completed_at.is_none());
    }

    #[test]
    fn test_reset_noop_when_no_record() {
        let db = mem_db();
        // Should not error even if no record exists
        reset(&db, "owner-nonexistent").expect("reset ok");
        let status = get_status(&db, "owner-nonexistent").expect("status ok");
        assert_eq!(status.current_step, 1);
        assert!(!status.completed);
    }

    // -- check_2fa ------------------------------------------------------------

    #[test]
    fn test_check_2fa_false_when_not_setup() {
        let db = mem_db();
        let result = check_2fa(&db, "owner-no-2fa").expect("should not error");
        assert!(!result);
    }
}
