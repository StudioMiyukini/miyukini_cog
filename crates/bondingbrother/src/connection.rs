//! Module de gestion de connexions de BondingBrother

/// @id bondingbrother_connection
/// @role data
/// @layer core
/// @do represent_connection
//@human / @human Connexion entre un Opérateur et l'écosystème.
#[derive(Debug, Clone)]
pub struct Connection {
    /// @id bondingbrother_connection_id
    /// @role data
    /// @layer core
    /// @do store_connection_id
    //@human / @human Identifiant unique de la connexion.
    /// @depends bondingbrother_connection
    pub id: String,
    /// @id bondingbrother_connection_operator_id
    /// @role data
    /// @layer core
    /// @do store_operator_id
    //@human / @human Identifiant de l'Opérateur connecté.
    /// @depends bondingbrother_connection
    pub operator_id: String,
}

/// @id bondingbrother_connection_manager_trait
/// @role infrastructure
/// @layer core
/// @do define_connection_manager_contract
//@human / @human Trait de gestion des connexions.
pub trait ConnectionManager {
    /// @id bondingbrother_connection_manager_connect
    /// @role mutator
    /// @layer core
    /// @do establish_connection
    //@human / @human Établit une connexion avec un Opérateur.
    /// @depends bondingbrother_connection_manager_trait
    fn connect(&mut self, operator_id: &str) -> Result<Connection, ConnectionError>;
}

/// @id bondingbrother_connection_error
/// @role error
/// @layer core
/// @do represent_connection_error
//@human / @human Erreur de connexion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionError {
    /// @id bondingbrother_connection_error_already_connected
    /// @role error
    /// @layer core
    /// @do represent_already_connected_error
    //@human / @human Opérateur déjà connecté.
    /// @depends bondingbrother_connection_error
    AlreadyConnected,
    /// @id bondingbrother_connection_error_not_implemented
    /// @role error
    /// @layer core
    /// @do represent_not_implemented_error
    //@human / @human Fonction non implémentée : refus explicite (C-14), jamais de succès factice.
    /// @depends bondingbrother_connection_error
    NonImplemente(String),
}

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::AlreadyConnected => write!(f, "Operator already connected"),
            ConnectionError::NonImplemente(m) => write!(f, "Non implémenté : {m}"),
        }
    }
}

impl std::error::Error for ConnectionError {}

/// Implémentation par défaut : **refuse toute connexion** (C-14, K-14).
///
/// Une connexion d'Opérateur exige authentification et règles BorderGuard (INV-BG-7), non
/// encore branchées : accepter n'importe quel `operator_id` serait un succès factice.
#[derive(Debug, Default)]
pub struct DefaultConnectionManager;

impl DefaultConnectionManager {
    /// Crée un gestionnaire de connexions (refus systématique tant que non implémenté).
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl ConnectionManager for DefaultConnectionManager {
    fn connect(&mut self, operator_id: &str) -> Result<Connection, ConnectionError> {
        Err(ConnectionError::NonImplemente(format!(
            "connexion de l'Opérateur '{operator_id}' : authentification et BorderGuard non branchés"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id bondingbrother_connection_test_creation
    /// @role test
    /// @layer core
    /// @do verify_connection_creation
    //@human / @human Test de création d'une connexion.
    /// @depends bondingbrother_connection
    #[test]
    fn test_connection_creation() {
        let conn = Connection {
            id: "conn-1".to_string(),
            operator_id: "op-1".to_string(),
        };
        assert_eq!(conn.id, "conn-1");
    }

    /// @id bondingbrother_connection_test_stub_refuse
    /// @role test
    /// @layer core
    /// @do verify_stub_refuses
    //@human / @human Le gestionnaire par défaut refuse explicitement (pas de connexion factice).
    #[test]
    fn test_default_connection_manager_refuse() {
        let mut mgr = DefaultConnectionManager::new();
        assert!(matches!(
            mgr.connect("op-1"),
            Err(ConnectionError::NonImplemente(_))
        ));
        assert!(matches!(
            mgr.connect(""),
            Err(ConnectionError::NonImplemente(_))
        ));
    }
}
