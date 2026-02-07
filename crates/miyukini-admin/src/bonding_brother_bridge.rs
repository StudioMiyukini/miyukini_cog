//! Pont vers BondingBrother de MiyukiniAdmin

/// @id: miyukiniadmin_bonding_brother_bridge_trait
/// @role: infrastructure
/// @layer: operator
/// @human: Trait de pont vers BondingBrother pour les connexions.
/// @do: define_bonding_brother_bridge_contract
pub trait BondingBrotherBridge {
    /// @id: miyukiniadmin_bonding_brother_bridge_connect
    /// @role: mutator
    /// @layer: operator
    /// @human: Établit une connexion via BondingBrother.
    /// @do: establish_bonding_brother_connection
    /// @depends: miyukiniadmin_bonding_brother_bridge_trait, bondingbrother_connection_manager_connect
    /// Établit une connexion via BondingBrother.
    fn connect(&mut self, operator_id: &str) -> Result<(), BridgeError>;
}

/// @id: miyukiniadmin_bonding_brother_bridge_error
/// @role: error
/// @layer: operator
/// @human: Erreur du pont BondingBrother.
/// @do: represent_bridge_error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeError {
    /// @id: miyukiniadmin_bonding_brother_bridge_error_connection_failed
    /// @role: error
    /// @layer: operator
    /// @human: Échec de connexion.
    /// @do: represent_connection_failed_error
    /// @depends: miyukiniadmin_bonding_brother_bridge_error
    ConnectionFailed(String),
}

impl std::fmt::Display for BridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BridgeError::ConnectionFailed(msg) => write!(f, "Connection failed: {msg}"),
        }
    }
}

impl std::error::Error for BridgeError {}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: miyukiniadmin_bonding_brother_bridge_test_error
    /// @role: test
    /// @layer: operator
    /// @human: Test de création d'erreur de pont.
    /// @do: verify_bridge_error_creation
    /// @depends: miyukiniadmin_bonding_brother_bridge_error
    #[test]
    fn test_bridge_error() {
        let error = BridgeError::ConnectionFailed("test".to_string());
        assert!(matches!(error, BridgeError::ConnectionFailed(_)));
    }
}
