//! Module de migration de versions d'EverBuddy

/// @id everbuddy_migration
/// @role data
/// @layer core
/// @do represent_migration
//@human / @human Migration d'une version vers une autre.
#[derive(Debug, Clone)]
pub struct Migration {
    /// @id everbuddy_migration_from_version
    /// @role data
    /// @layer core
    /// @do store_from_version
    //@human / @human Version source.
    /// @depends everbuddy_migration
    pub from_version: String,
    /// @id everbuddy_migration_to_version
    /// @role data
    /// @layer core
    /// @do store_to_version
    //@human / @human Version cible.
    /// @depends everbuddy_migration
    pub to_version: String,
}

/// @id everbuddy_migration_executor_trait
/// @role infrastructure
/// @layer core
/// @do define_migration_executor_contract
//@human / @human Trait d'exécution de migration.
pub trait MigrationExecutor {
    /// @id everbuddy_migration_executor_execute
    /// @role mutator
    /// @layer core
    /// @do execute_migration
    //@human / @human Exécute une migration.
    /// @depends everbuddy_migration_executor_trait
    fn execute(&mut self, migration: &Migration) -> Result<(), MigrationError>;
}

/// @id everbuddy_migration_error
/// @role error
/// @layer core
/// @do represent_migration_error
//@human / @human Erreur de migration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationError {
    /// @id everbuddy_migration_error_failed
    /// @role error
    /// @layer core
    /// @do represent_failed_error
    //@human / @human Échec de migration.
    /// @depends everbuddy_migration_error
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

/// Exécuteur par défaut : enregistre les migrations exécutées, exécution no-op (délégation à KindMother côté produit).
#[derive(Debug, Default)]
pub struct DefaultMigrationExecutor {
    executed: Vec<Migration>,
}

impl DefaultMigrationExecutor {
    /// Crée un exécuteur vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Retourne la liste des migrations déjà exécutées.
    #[must_use]
    pub fn executed(&self) -> &[Migration] {
        &self.executed
    }
}

impl MigrationExecutor for DefaultMigrationExecutor {
    fn execute(&mut self, migration: &Migration) -> Result<(), MigrationError> {
        self.executed.push(migration.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id everbuddy_migration_test_creation
    /// @role test
    /// @layer core
    /// @do verify_migration_creation
    //@human / @human Test de création d'une migration.
    /// @depends everbuddy_migration
    #[test]
    fn test_migration_creation() {
        let migration = Migration {
            from_version: "1.0".to_string(),
            to_version: "2.0".to_string(),
        };
        assert_eq!(migration.from_version, "1.0");
    }

    #[test]
    fn test_default_migration_executor() {
        let mut ex = DefaultMigrationExecutor::new();
        let m = Migration {
            from_version: "1.0".to_string(),
            to_version: "2.0".to_string(),
        };
        assert!(ex.execute(&m).is_ok());
        assert_eq!(ex.executed().len(), 1);
        assert_eq!(ex.executed()[0].to_version, "2.0");
    }
}
