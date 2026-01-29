//! Module de gestion de dégradation de WorrySentinel

/// @id: worrysentinel_degradation_state
/// @role: data
/// @layer: core
/// @human: État de dégradation du système (T0-T4).
/// @do: represent_degradation_state
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DegradationState {
    /// @id: worrysentinel_degradation_state_normal
    /// @role: data
    /// @layer: core
    /// @human: T0 - Normal (toutes capacités disponibles).
    /// @do: represent_normal_state
    /// @depends: worrysentinel_degradation_state
    Normal,
    /// @id: worrysentinel_degradation_state_unstable
    /// @role: data
    /// @layer: core
    /// @human: T1 - Instable (log renforcé).
    /// @do: represent_unstable_state
    /// @depends: worrysentinel_degradation_state
    Unstable,
    /// @id: worrysentinel_degradation_state_degraded
    /// @role: data
    /// @layer: core
    /// @human: T2 - Dégradé (certaines capacités désactivées).
    /// @do: represent_degraded_state
    /// @depends: worrysentinel_degradation_state
    Degraded,
    /// @id: worrysentinel_degradation_state_restricted
    /// @role: data
    /// @layer: core
    /// @human: T3 - Restreint (gel des produits non essentiels).
    /// @do: represent_restricted_state
    /// @depends: worrysentinel_degradation_state
    Restricted,
    /// @id: worrysentinel_degradation_state_blocked
    /// @role: data
    /// @layer: core
    /// @human: T4 - Bloqué (uniquement diagnostics).
    /// @do: represent_blocked_state
    /// @depends: worrysentinel_degradation_state
    Blocked,
}

/// @id: worrysentinel_degradation
/// @role: data
/// @layer: core
/// @human: État de dégradation avec métadonnées.
/// @do: represent_degradation
#[derive(Debug, Clone)]
pub struct Degradation {
    /// @id: worrysentinel_degradation_state_field
    /// @role: data
    /// @layer: core
    /// @human: État de dégradation.
    /// @do: store_degradation_state
    /// @depends: worrysentinel_degradation
    pub state: DegradationState,
}

/// @id: worrysentinel_degradation_manager_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait de gestion de la dégradation.
/// @do: define_degradation_manager_contract
pub trait DegradationManager {
    /// @id: worrysentinel_degradation_manager_get_current
    /// @role: accessor
    /// @layer: core
    /// @human: Récupère l'état de dégradation actuel.
    /// @do: get_current_degradation
    /// @depends: worrysentinel_degradation_manager_trait
    fn get_current(&self) -> Degradation;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: worrysentinel_degradation_test_state_ordering
    /// @role: test
    /// @layer: core
    /// @human: Test de l'ordre des états de dégradation.
    /// @do: verify_degradation_state_ordering
    /// @depends: worrysentinel_degradation_state
    #[test]
    fn test_degradation_state_ordering() {
        assert!(DegradationState::Blocked > DegradationState::Restricted);
        assert!(DegradationState::Restricted > DegradationState::Degraded);
    }
}
