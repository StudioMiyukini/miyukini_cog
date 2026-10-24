//! @id     everbuddy_migration
//! @do     definir_contrat_migration
//! @role   orchestration
//! @layer  core
//! @human  everbuddy — contrat de migration ; aucune application réelle dans le Core.
//!
//! Module de migration de versions d'EverBuddy.
//!
//! Selon D1 / C-31, EverBuddy décide des migrations mais ne les applique pas :
//! l'application revient au Kernel et aux opérateurs. [`DefaultMigrationExecutor`]
//! refuse donc explicitement (C-14, C-12) au lieu de noter une migration comme
//! appliquée sans l'avoir faite (constat K-15).

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
    /// @id everbuddy_migration_error_non_implemente
    /// @role error
    /// @layer core
    /// @do represent_not_implemented
    //@human / @human Application de migration non implémentée dans le Core (D1) :
    //@human rien n'a été appliqué ni enregistré.
    /// @depends everbuddy_migration_error
    NonImplemente(String),
}

impl std::fmt::Display for MigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationError::Failed(msg) => write!(f, "Migration failed: {msg}"),
            MigrationError::NonImplemente(msg) => {
                write!(f, "migration non implémentée : {msg}")
            }
        }
    }
}

impl std::error::Error for MigrationError {}

/// Exécuteur par défaut : refuse toute migration (K-15, C-14).
///
/// L'application réelle sera déléguée au Kernel / aux opérateurs (D1, lot C du plan).
/// `execute` renvoie toujours [`MigrationError::NonImplemente`] et n'enregistre rien
/// dans [`DefaultMigrationExecutor::executed`].
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

    /// Retourne la liste des migrations réellement appliquées (toujours vide tant que
    /// l'application n'est pas implémentée).
    #[must_use]
    pub fn executed(&self) -> &[Migration] {
        &self.executed
    }
}

impl MigrationExecutor for DefaultMigrationExecutor {
    fn execute(&mut self, migration: &Migration) -> Result<(), MigrationError> {
        Err(MigrationError::NonImplemente(format!(
            "{} -> {} : l'application des migrations est réservée au Kernel et aux opérateurs (D1)",
            migration.from_version, migration.to_version
        )))
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

    /// @id everbuddy_migration_test_refus_explicite
    /// @role test
    /// @layer core
    /// @do verify_default_executor_refuses
    //@human / @human L'exécuteur par défaut refuse et n'enregistre rien (K-15).
    /// @depends everbuddy_migration_executor_execute
    #[test]
    fn test_default_migration_executor_refuse_explicitement() {
        let mut ex = DefaultMigrationExecutor::new();
        let m = Migration {
            from_version: "1.0".to_string(),
            to_version: "2.0".to_string(),
        };
        let err = ex.execute(&m).unwrap_err();
        assert!(matches!(err, MigrationError::NonImplemente(_)));
        assert!(err.to_string().contains("non implémentée"));
        assert!(ex.executed().is_empty());
    }
}
