//! Module de gestion de dégradation de WorrySentinel

/// @id worrysentinel_degradation_state
/// @role data
/// @layer core
/// @do represent_degradation_state
//@human / @human État de dégradation du système (T0-T4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DegradationState {
    /// @id worrysentinel_degradation_state_normal
    /// @role data
    /// @layer core
    /// @do represent_normal_state
    //@human / @human T0 - Normal (toutes capacités disponibles).
    /// @depends worrysentinel_degradation_state
    Normal,
    /// @id worrysentinel_degradation_state_unstable
    /// @role data
    /// @layer core
    /// @do represent_unstable_state
    //@human / @human T1 - Instable (log renforcé).
    /// @depends worrysentinel_degradation_state
    Unstable,
    /// @id worrysentinel_degradation_state_degraded
    /// @role data
    /// @layer core
    /// @do represent_degraded_state
    //@human / @human T2 - Dégradé (certaines capacités désactivées).
    /// @depends worrysentinel_degradation_state
    Degraded,
    /// @id worrysentinel_degradation_state_restricted
    /// @role data
    /// @layer core
    /// @do represent_restricted_state
    //@human / @human T3 - Restreint (gel des produits non essentiels).
    /// @depends worrysentinel_degradation_state
    Restricted,
    /// @id worrysentinel_degradation_state_blocked
    /// @role data
    /// @layer core
    /// @do represent_blocked_state
    //@human / @human T4 - Bloqué (uniquement diagnostics).
    /// @depends worrysentinel_degradation_state
    Blocked,
}

/// @id worrysentinel_degradation
/// @role data
/// @layer core
/// @do represent_degradation
//@human / @human État de dégradation avec métadonnées.
#[derive(Debug, Clone)]
pub struct Degradation {
    /// @id worrysentinel_degradation_state_field
    /// @role data
    /// @layer core
    /// @do store_degradation_state
    //@human / @human État de dégradation.
    /// @depends worrysentinel_degradation
    pub state: DegradationState,
}

/// @id worrysentinel_degradation_manager_trait
/// @role infrastructure
/// @layer core
/// @do define_degradation_manager_contract
//@human / @human Trait de gestion de la dégradation.
pub trait DegradationManager {
    /// @id worrysentinel_degradation_manager_get_current
    /// @role accessor
    /// @layer core
    /// @do get_current_degradation
    //@human / @human Récupère l'état de dégradation actuel.
    /// @depends worrysentinel_degradation_manager_trait
    fn get_current(&self) -> Degradation;
}

/// Gestionnaire par défaut : état en mémoire (sans persistance).
#[derive(Debug)]
pub struct DefaultDegradationManager {
    degradation: Degradation,
}

impl DefaultDegradationManager {
    /// Crée un gestionnaire avec l'état donné (immuable, conformité INV-WS-4).
    #[must_use]
    pub fn new(degradation: Degradation) -> Self {
        Self { degradation }
    }
}

impl Default for DefaultDegradationManager {
    fn default() -> Self {
        Self {
            degradation: Degradation {
                state: DegradationState::Normal,
            },
        }
    }
}

impl DegradationManager for DefaultDegradationManager {
    fn get_current(&self) -> Degradation {
        self.degradation.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id worrysentinel_degradation_test_state_ordering
    /// @role test
    /// @layer core
    /// @do verify_degradation_state_ordering
    //@human / @human Test de l'ordre des états de dégradation.
    /// @depends worrysentinel_degradation_state
    #[test]
    fn test_degradation_state_ordering() {
        assert!(DegradationState::Blocked > DegradationState::Restricted);
        assert!(DegradationState::Restricted > DegradationState::Degraded);
    }

    #[test]
    fn test_default_degradation_manager() {
        let mgr = DefaultDegradationManager::default();
        assert_eq!(mgr.get_current().state, DegradationState::Normal);
    }
}
