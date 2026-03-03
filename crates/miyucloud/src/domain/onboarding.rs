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

/// Retourne le statut de l'onboarding pour un proprietaire.
pub fn get_status(
    _db: &crate::data::MiyucloudDb,
    _owner_id: &str,
) -> Result<OnboardingStatus, MiyucloudError> {
    todo!("V3-T01: Implémenter get_status")
}

/// Verifie que la passphrase est definie et correcte.
pub fn check_passphrase(
    _db: &crate::data::MiyucloudDb,
    _owner_id: &str,
) -> Result<bool, MiyucloudError> {
    todo!("V3-T01: Implémenter check_passphrase")
}

/// Verifie que le 2FA est configure.
pub fn check_2fa(
    _db: &crate::data::MiyucloudDb,
    _owner_id: &str,
) -> Result<bool, MiyucloudError> {
    todo!("V3-T01: Implémenter check_2fa")
}

/// Verifie que le stockage est accessible et utilisable.
pub fn verify_storage(
    _storage_path: &str,
) -> Result<bool, MiyucloudError> {
    todo!("V3-T01: Implémenter verify_storage")
}

/// Marque l'onboarding comme termine.
pub fn complete(
    _db: &crate::data::MiyucloudDb,
    _owner_id: &str,
) -> Result<(), MiyucloudError> {
    todo!("V3-T01: Implémenter complete")
}

/// Reinitialise l'onboarding (admin uniquement).
pub fn reset(
    _db: &crate::data::MiyucloudDb,
    _owner_id: &str,
) -> Result<(), MiyucloudError> {
    todo!("V3-T01: Implémenter reset")
}
