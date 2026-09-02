//! Pont vers BondingBrother de MiyukiniAdmin

/// @id miyukiniadmin_bonding_brother_bridge_trait
/// @role infrastructure
/// @layer operator
/// @do define_bonding_brother_bridge_contract
//@human / @human Trait de pont vers BondingBrother pour les connexions.
pub trait BondingBrotherBridge {
    /// @id miyukiniadmin_bonding_brother_bridge_connect
    /// @role mutator
    /// @layer operator
    /// @do establish_bonding_brother_connection
    //@human / @human Établit une connexion via BondingBrother.
    /// @depends miyukiniadmin_bonding_brother_bridge_trait, bondingbrother_connection_manager_connect
    /// Établit une connexion via BondingBrother.
    fn connect(&mut self, operator_id: &str) -> Result<(), BridgeError>;
}

/// @id miyukiniadmin_bonding_brother_bridge_error
/// @role error
/// @layer operator
/// @do represent_bridge_error
//@human / @human Erreur du pont BondingBrother.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeError {
    /// @id miyukiniadmin_bonding_brother_bridge_error_connection_failed
    /// @role error
    /// @layer operator
    /// @do represent_connection_failed_error
    //@human / @human Échec de connexion.
    /// @depends miyukiniadmin_bonding_brother_bridge_error
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

    /// @id miyukiniadmin_bonding_brother_bridge_test_error
    /// @role test
    /// @layer operator
    /// @do verify_bridge_error_creation
    //@human / @human Test de création d'erreur de pont.
    /// @depends miyukiniadmin_bonding_brother_bridge_error
    #[test]
    fn test_bridge_error() {
        let error = BridgeError::ConnectionFailed("test".to_string());
        assert!(matches!(error, BridgeError::ConnectionFailed(_)));
    }
}
