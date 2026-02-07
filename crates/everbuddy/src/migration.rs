//! Module de migration de versions d'EverBuddy

/// @id: everbuddy_migration
/// @role: data
/// @layer: core
/// @human: Migration d'une version vers une autre.
/// @do: represent_migration
#[derive(Debug, Clone)]
pub struct Migration {
    /// @id: everbuddy_migration_from_version
    /// @role: data
    /// @layer: core
    /// @human: Version source.
    /// @do: store_from_version
    /// @depends: everbuddy_migration
    pub from_version: String,
    /// @id: everbuddy_migration_to_version
    /// @role: data
    /// @layer: core
    /// @human: Version cible.
    /// @do: store_to_version
    /// @depends: everbuddy_migration
    pub to_version: String,
}

/// @id: everbuddy_migration_executor_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait d'exécution de migration.
/// @do: define_migration_executor_contract
pub trait MigrationExecutor {
    /// @id: everbuddy_migration_executor_execute
    /// @role: mutator
    /// @layer: core
    /// @human: Exécute une migration.
    /// @do: execute_migration
    /// @depends: everbuddy_migration_executor_trait
    fn execute(&mut self, migration: &Migration) -> Result<(), MigrationError>;
}

/// @id: everbuddy_migration_error
/// @role: error
/// @layer: core
/// @human: Erreur de migration.
/// @do: represent_migration_error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationError {
    /// @id: everbuddy_migration_error_failed
    /// @role: error
    /// @layer: core
    /// @human: Échec de migration.
    /// @do: represent_failed_error
    /// @depends: everbuddy_migration_error
    Failed(String),
}

impl std::fmt::Display for MigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationError::Failed(msg) => write!(f, "Migration failed: {msg}"),
        }
    }
}

impl std::error::Error for MigrationError {}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: everbuddy_migration_test_creation
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une migration.
    /// @do: verify_migration_creation
    /// @depends: everbuddy_migration
    #[test]
    fn test_migration_creation() {
        let migration = Migration {
            from_version: "1.0".to_string(),
            to_version: "2.0".to_string(),
        };
        assert_eq!(migration.from_version, "1.0");
    }
}
