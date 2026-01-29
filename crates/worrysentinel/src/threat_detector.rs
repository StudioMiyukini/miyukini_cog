//! Module de détection de menaces de WorrySentinel

/// @id: worrysentinel_threat_level
/// @role: data
/// @layer: core
/// @human: Niveau de menace détecté.
/// @do: represent_threat_level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel {
    /// @id: worrysentinel_threat_level_none
    /// @role: data
    /// @layer: core
    /// @human: Aucune menace détectée.
    /// @do: represent_no_threat
    /// @depends: worrysentinel_threat_level
    None,
    /// @id: worrysentinel_threat_level_low
    /// @role: data
    /// @layer: core
    /// @human: Menace faible.
    /// @do: represent_low_threat
    /// @depends: worrysentinel_threat_level
    Low,
    /// @id: worrysentinel_threat_level_medium
    /// @role: data
    /// @layer: core
    /// @human: Menace moyenne.
    /// @do: represent_medium_threat
    /// @depends: worrysentinel_threat_level
    Medium,
    /// @id: worrysentinel_threat_level_high
    /// @role: data
    /// @layer: core
    /// @human: Menace élevée.
    /// @do: represent_high_threat
    /// @depends: worrysentinel_threat_level
    High,
    /// @id: worrysentinel_threat_level_critical
    /// @role: data
    /// @layer: core
    /// @human: Menace critique.
    /// @do: represent_critical_threat
    /// @depends: worrysentinel_threat_level
    Critical,
}

/// @id: worrysentinel_threat_detector_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait de détection de menaces.
/// @do: define_threat_detector_contract
pub trait ThreatDetector {
    /// @id: worrysentinel_threat_detector_detect
    /// @role: infrastructure
    /// @layer: core
    /// @human: Détecte les menaces dans le système.
    /// @do: detect_threats
    /// @depends: worrysentinel_threat_detector_trait
    fn detect(&self) -> ThreatLevel;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: worrysentinel_threat_test_level_ordering
    /// @role: test
    /// @layer: core
    /// @human: Test de l'ordre des niveaux de menace.
    /// @do: verify_threat_level_ordering
    /// @depends: worrysentinel_threat_level
    #[test]
    fn test_threat_level_ordering() {
        assert!(ThreatLevel::Critical > ThreatLevel::High);
        assert!(ThreatLevel::High > ThreatLevel::Medium);
    }
}
