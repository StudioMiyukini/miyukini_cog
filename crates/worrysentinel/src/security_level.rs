//! Module de niveaux de sécurité de WorrySentinel

/// @id: worrysentinel_security_level
/// @role: data
/// @layer: core
/// @human: Niveau de sécurité actif (0-4).
/// @do: represent_security_level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    /// @id: worrysentinel_security_level_public
    /// @role: data
    /// @layer: core
    /// @human: Niveau 0 - Public (site vitrine).
    /// @do: represent_public_level
    /// @depends: worrysentinel_security_level
    Public,
    /// @id: worrysentinel_security_level_standard
    /// @role: data
    /// @layer: core
    /// @human: Niveau 1 - Standard (CMS, backoffice).
    /// @do: represent_standard_level
    /// @depends: worrysentinel_security_level
    Standard,
    /// @id: worrysentinel_security_level_sensitive
    /// @role: data
    /// @layer: core
    /// @human: Niveau 2 - Sensible (données personnelles).
    /// @do: represent_sensitive_level
    /// @depends: worrysentinel_security_level
    Sensitive,
    /// @id: worrysentinel_security_level_critical
    /// @role: data
    /// @layer: core
    /// @human: Niveau 3 - Critique (auth, paiement).
    /// @do: represent_critical_level
    /// @depends: worrysentinel_security_level
    Critical,
    /// @id: worrysentinel_security_level_hardened
    /// @role: data
    /// @layer: core
    /// @human: Niveau 4 - Durci (environnement hostile).
    /// @do: represent_hardened_level
    /// @depends: worrysentinel_security_level
    Hardened,
}

/// @id: worrysentinel_security_level_manager_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait de gestion des niveaux de sécurité.
/// @do: define_security_level_manager_contract
pub trait SecurityLevelManager {
    /// @id: worrysentinel_security_level_manager_get_current
    /// @role: accessor
    /// @layer: core
    /// @human: Récupère le niveau de sécurité actuel.
    /// @do: get_current_security_level
    /// @depends: worrysentinel_security_level_manager_trait
    fn get_current(&self) -> SecurityLevel;
}

/// Gestionnaire par défaut : niveau fixe en mémoire (sans persistance).
#[derive(Debug)]
pub struct DefaultSecurityLevelManager {
    level: SecurityLevel,
}

impl DefaultSecurityLevelManager {
    /// Crée un gestionnaire avec le niveau donné (immuable, conformité INV-WS-4).
    #[must_use]
    pub fn new(level: SecurityLevel) -> Self {
        Self { level }
    }
}

impl Default for DefaultSecurityLevelManager {
    fn default() -> Self {
        Self {
            level: SecurityLevel::Standard,
        }
    }
}

impl SecurityLevelManager for DefaultSecurityLevelManager {
    fn get_current(&self) -> SecurityLevel {
        self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: worrysentinel_security_test_level_ordering
    /// @role: test
    /// @layer: core
    /// @human: Test de l'ordre des niveaux de sécurité.
    /// @do: verify_security_level_ordering
    /// @depends: worrysentinel_security_level
    #[test]
    fn test_security_level_ordering() {
        assert!(SecurityLevel::Hardened > SecurityLevel::Critical);
        assert!(SecurityLevel::Critical > SecurityLevel::Sensitive);
    }

    #[test]
    fn test_default_security_level_manager() {
        let mgr = DefaultSecurityLevelManager::new(SecurityLevel::Sensitive);
        assert_eq!(mgr.get_current(), SecurityLevel::Sensitive);
    }
}
