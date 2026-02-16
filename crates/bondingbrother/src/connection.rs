//! Module de gestion de connexions de BondingBrother

use std::collections::HashSet;
use miyukini_kernel::{IdGenerator, UuidIdGenerator};

/// @id: bondingbrother_connection
/// @role: data
/// @layer: core
/// @human: Connexion entre un Opérateur et l'écosystème.
/// @do: represent_connection
#[derive(Debug, Clone)]
pub struct Connection {
    /// @id: bondingbrother_connection_id
    /// @role: data
    /// @layer: core
    /// @human: Identifiant unique de la connexion.
    /// @do: store_connection_id
    /// @depends: bondingbrother_connection
    pub id: String,
    /// @id: bondingbrother_connection_operator_id
    /// @role: data
    /// @layer: core
    /// @human: Identifiant de l'Opérateur connecté.
    /// @do: store_operator_id
    /// @depends: bondingbrother_connection
    pub operator_id: String,
}

/// @id: bondingbrother_connection_manager_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait de gestion des connexions.
/// @do: define_connection_manager_contract
pub trait ConnectionManager {
    /// @id: bondingbrother_connection_manager_connect
    /// @role: mutator
    /// @layer: core
    /// @human: Établit une connexion avec un Opérateur.
    /// @do: establish_connection
    /// @depends: bondingbrother_connection_manager_trait
    fn connect(&mut self, operator_id: &str) -> Result<Connection, ConnectionError>;
}

/// @id: bondingbrother_connection_error
/// @role: error
/// @layer: core
/// @human: Erreur de connexion.
/// @do: represent_connection_error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionError {
    /// @id: bondingbrother_connection_error_already_connected
    /// @role: error
    /// @layer: core
    /// @human: Opérateur déjà connecté.
    /// @do: represent_already_connected_error
    /// @depends: bondingbrother_connection_error
    AlreadyConnected,
}

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::AlreadyConnected => write!(f, "Operator already connected"),
        }
    }
}

impl std::error::Error for ConnectionError {}

/// Implémentation par défaut : registre en mémoire des connexions par operator_id.
#[derive(Debug, Default)]
pub struct DefaultConnectionManager {
    connected: HashSet<String>,
}

impl DefaultConnectionManager {
    /// Crée un gestionnaire de connexions vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl ConnectionManager for DefaultConnectionManager {
    fn connect(&mut self, operator_id: &str) -> Result<Connection, ConnectionError> {
        let op = operator_id.to_string();
        if self.connected.contains(&op) {
            return Err(ConnectionError::AlreadyConnected);
        }
        let id = UuidIdGenerator.generate().to_string();
        self.connected.insert(op.clone());
        Ok(Connection {
            id,
            operator_id: op,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: bondingbrother_connection_test_creation
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une connexion.
    /// @do: verify_connection_creation
    /// @depends: bondingbrother_connection
    #[test]
    fn test_connection_creation() {
        let conn = Connection {
            id: "conn-1".to_string(),
            operator_id: "op-1".to_string(),
        };
        assert_eq!(conn.id, "conn-1");
    }

    #[test]
    fn test_default_connection_manager_connect() {
        use super::*;
        let mut mgr = DefaultConnectionManager::new();
        let c1 = mgr.connect("op-1").unwrap();
        assert!(!c1.id.is_empty());
        assert_eq!(c1.operator_id, "op-1");
        assert!(mgr.connect("op-1").is_err());
        let c2 = mgr.connect("op-2").unwrap();
        assert_ne!(c1.id, c2.id);
    }
}
