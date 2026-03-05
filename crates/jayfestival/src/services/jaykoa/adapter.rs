//! Adapter JayFestival → JayKoa (agenda, éditions, conflits de dates).
//!
//! @id: jaykoa_publish_edition
//! @do: publie une édition vers JayKoa pour agrégation calendrier et détection conflits
//! @layer: domain
//!
//! @id: jaykoa_get_conflicts
//! @do: récupère les conflits de dates pour un exposant ou visiteur (vue agrégée JayKoa)
//! @layer: domain
//!
//! Alpha : pas de crate JayKoa ; contrat et stub (retour vide ou unimplemented).

use std::fmt;

/// Erreur d'intégration JayKoa (service indisponible, alpha stub).
#[derive(Debug, Clone)]
pub struct JayKoaError {
    /// Message d'erreur (réseau, service indisponible).
    pub message: String,
}

impl fmt::Display for JayKoaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for JayKoaError {}

/// Conflit de dates (événement A vs événement B).
#[derive(Debug, Clone)]
pub struct JayKoaConflict {
    /// Identifiant édition / événement 1.
    pub edition_id_a: String,
    /// Identifiant édition / événement 2.
    pub edition_id_b: String,
    /// Description courte du conflit.
    pub description: String,
}

/// Publie une édition vers JayKoa pour agrégation calendrier et détection de conflits.
///
/// @id: jaykoa_publish_edition
/// @do: publie une édition vers JayKoa pour agrégation calendrier et détection conflits
/// @layer: domain
pub fn jaykoa_publish_edition(_edition_id: &str, _payload: &str) -> Result<(), JayKoaError> {
    // Alpha : pas de crate JayKoa ; stub.
    Err(JayKoaError {
        message: "JayKoa non intégré (alpha stub)".to_string(),
    })
}

/// Récupère les conflits de dates pour un exposant ou visiteur (vue agrégée JayKoa).
///
/// @id: jaykoa_get_conflicts
/// @do: récupère les conflits de dates pour un exposant ou visiteur (vue agrégée JayKoa)
/// @layer: domain
pub fn jaykoa_get_conflicts(
    _user_or_exposant_id: &str,
    _date_range: Option<(&str, &str)>,
) -> Result<Vec<JayKoaConflict>, JayKoaError> {
    // Alpha : pas de crate JayKoa ; retour liste vide.
    Ok(Vec::new())
}
