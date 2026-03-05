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

/// Détecteur par défaut : agrège des signaux fournis au constructeur, retourne le maximum (immuable, INV-WS-4).
#[derive(Debug)]
pub struct DefaultThreatDetector {
    levels: std::collections::VecDeque<ThreatLevel>,
}

impl DefaultThreatDetector {
    /// Crée un détecteur vide ; `detect()` retourne `ThreatLevel::None`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            levels: std::collections::VecDeque::new(),
        }
    }

    /// Crée un détecteur avec les signaux donnés (vue déclarative fournie par l'adaptateur au construction).
    #[must_use]
    pub fn with_signals(levels: impl IntoIterator<Item = ThreatLevel>) -> Self {
        Self {
            levels: levels.into_iter().collect(),
        }
    }

    /// Retourne le niveau maximal parmi les signaux, ou `ThreatLevel::None` si aucun.
    #[must_use]
    pub fn max_signal(&self) -> ThreatLevel {
        self.levels
            .iter()
            .copied()
            .max()
            .unwrap_or(ThreatLevel::None)
    }
}

impl Default for DefaultThreatDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl ThreatDetector for DefaultThreatDetector {
    fn detect(&self) -> ThreatLevel {
        self.max_signal()
    }
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

    #[test]
    fn test_default_threat_detector() {
        let d = DefaultThreatDetector::new();
        assert_eq!(d.detect(), ThreatLevel::None);
        let d = DefaultThreatDetector::with_signals([ThreatLevel::Low, ThreatLevel::Medium]);
        assert_eq!(d.detect(), ThreatLevel::Medium);
    }
}
