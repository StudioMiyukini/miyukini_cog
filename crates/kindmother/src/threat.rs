//! Module de détection de menaces de KindMother
//!
//! Ce module détecte la corruption, les anomalies, et les menaces potentielles
//! dans la persistance et les opérations.

use crate::state::InstanceIdentity;

/// @id kindmother_threat_level
/// @role data
/// @layer core
/// @do represent_threat_level
//@human / @human Niveau de menace détecté.
/// Niveau de menace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel {
    /// @id kindmother_threat_level_none
    /// @role data
    /// @layer core
    /// @do represent_no_threat
    //@human / @human Aucune menace détectée.
    /// @depends kindmother_threat_level
    None,
    /// @id kindmother_threat_level_low
    /// @role data
    /// @layer core
    /// @do represent_low_threat
    //@human / @human Menace faible détectée.
    /// @depends kindmother_threat_level
    Low,
    /// @id kindmother_threat_level_medium
    /// @role data
    /// @layer core
    /// @do represent_medium_threat
    //@human / @human Menace moyenne détectée.
    /// @depends kindmother_threat_level
    Medium,
    /// @id kindmother_threat_level_high
    /// @role data
    /// @layer core
    /// @do represent_high_threat
    //@human / @human Menace élevée détectée.
    /// @depends kindmother_threat_level
    High,
    /// @id kindmother_threat_level_critical
    /// @role data
    /// @layer core
    /// @do represent_critical_threat
    //@human / @human Menace critique détectée (corruption).
    /// @depends kindmother_threat_level
    Critical,
}

/// @id kindmother_threat_detector_trait
/// @role infrastructure
/// @layer core
/// @do define_threat_detection_contract
//@human / @human Trait de détection de menaces. Détecte la corruption et les anomalies.
/// Trait de détection de menaces.
pub trait ThreatDetector {
    /// @id kindmother_threat_detector_check_corruption
    /// @role infrastructure
    /// @layer core
    /// @do check_instance_corruption
    //@human / @human Vérifie la corruption dans une instance.
    /// @depends kindmother_threat_detector_trait
    /// Vérifie la corruption dans une instance.
    ///
    /// # Arguments
    ///
    /// * `instance` - Identité de l'instance à vérifier
    ///
    /// # Returns
    ///
    /// Niveau de menace détecté.
    fn check_corruption(&self, instance: &InstanceIdentity) -> ThreatLevel;

    /// @id kindmother_threat_detector_check_anomalies
    /// @role infrastructure
    /// @layer core
    /// @do check_instance_anomalies
    //@human / @human Vérifie les anomalies dans les opérations d'une instance.
    /// @depends kindmother_threat_detector_trait
    /// Vérifie les anomalies dans les opérations.
    ///
    /// # Arguments
    ///
    /// * `instance` - Identité de l'instance à vérifier
    ///
    /// # Returns
    ///
    /// Niveau de menace détecté.
    fn check_anomalies(&self, instance: &InstanceIdentity) -> ThreatLevel;
}

/// @id kindmother_threat_detector_default
/// @role infrastructure
/// @layer core
/// @do provide_default_threat_detector
//@human / @human Détecteur de menaces par défaut. Implémentation basique pour les tests.
/// Détecteur de menaces par défaut.
#[derive(Debug, Default)]
pub struct DefaultThreatDetector;

impl ThreatDetector for DefaultThreatDetector {
    fn check_corruption(&self, _instance: &InstanceIdentity) -> ThreatLevel {
        ThreatLevel::None
    }

    fn check_anomalies(&self, _instance: &InstanceIdentity) -> ThreatLevel {
        ThreatLevel::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{InstanceState, InstanceType};

    /// @id kindmother_threat_test_detector_default
    /// @role test
    /// @layer core
    /// @do verify_default_threat_detector
    //@human / @human Test du détecteur de menaces par défaut.
    /// @depends kindmother_threat_detector_default, kindmother_threat_detector_check_corruption
    #[test]
    fn test_detector_default() {
        let detector = DefaultThreatDetector;
        let instance = InstanceState::new(InstanceType::Mother);
        assert_eq!(
            detector.check_corruption(&instance.identity),
            ThreatLevel::None
        );
        assert_eq!(
            detector.check_anomalies(&instance.identity),
            ThreatLevel::None
        );
    }
}
