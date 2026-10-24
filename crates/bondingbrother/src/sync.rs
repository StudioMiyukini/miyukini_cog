//! Module de synchronisation de BondingBrother

/// @id bondingbrother_sync_strategy
/// @role data
/// @layer core
/// @do represent_sync_strategy
//@human / @human Stratégie de synchronisation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncStrategy {
    /// @id bondingbrother_sync_strategy_immediate
    /// @role data
    /// @layer core
    /// @do represent_immediate_strategy
    //@human / @human Synchronisation immédiate.
    /// @depends bondingbrother_sync_strategy
    Immediate,
    /// @id bondingbrother_sync_strategy_deferred
    /// @role data
    /// @layer core
    /// @do represent_deferred_strategy
    //@human / @human Synchronisation différée.
    /// @depends bondingbrother_sync_strategy
    Deferred,
}

/// @id bondingbrother_sync_manager_trait
/// @role infrastructure
/// @layer core
/// @do define_sync_manager_contract
//@human / @human Trait de gestion de la synchronisation.
pub trait SyncManager {
    /// @id bondingbrother_sync_manager_sync
    /// @role mutator
    /// @layer core
    /// @do execute_sync
    //@human / @human Synchronise selon la stratégie.
    /// @depends bondingbrother_sync_manager_trait
    fn sync(&mut self, strategy: SyncStrategy) -> Result<(), SyncError>;
}

/// @id bondingbrother_sync_error
/// @role error
/// @layer core
/// @do represent_sync_error
//@human / @human Erreur de synchronisation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncError {
    /// @id bondingbrother_sync_error_failed
    /// @role error
    /// @layer core
    /// @do represent_failed_error
    //@human / @human Échec de synchronisation.
    /// @depends bondingbrother_sync_error
    Failed(String),
    /// @id bondingbrother_sync_error_not_implemented
    /// @role error
    /// @layer core
    /// @do represent_not_implemented_error
    //@human / @human Synchronisation non implémentée : refus explicite (C-14), jamais d'`Ok` silencieux.
    /// @depends bondingbrother_sync_error
    NonImplemente(String),
}

impl std::fmt::Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncError::Failed(msg) => write!(f, "Sync failed: {msg}"),
            SyncError::NonImplemente(msg) => write!(f, "Non implémenté : {msg}"),
        }
    }
}

impl std::error::Error for SyncError {}

/// Implémentation par défaut : **refuse** (C-14, K-14). Un `Ok(())` sans rien synchroniser
/// masquerait une perte de données hors ligne.
#[derive(Debug, Default)]
pub struct DefaultSyncManager;

impl DefaultSyncManager {
    /// Crée un gestionnaire de sync (refus systématique tant que non implémenté).
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl SyncManager for DefaultSyncManager {
    fn sync(&mut self, strategy: SyncStrategy) -> Result<(), SyncError> {
        Err(SyncError::NonImplemente(format!(
            "synchronisation {strategy:?} : tampon hors ligne non branché"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id bondingbrother_sync_test_strategy
    /// @role test
    /// @layer core
    /// @do verify_sync_strategies
    //@human / @human Test des stratégies de synchronisation.
    /// @depends bondingbrother_sync_strategy
    #[test]
    fn test_sync_strategies() {
        assert_eq!(SyncStrategy::Immediate, SyncStrategy::Immediate);
        assert_ne!(SyncStrategy::Immediate, SyncStrategy::Deferred);
    }

    /// @id bondingbrother_sync_test_stub_refuse
    /// @role test
    /// @layer core
    /// @do verify_stub_refuses
    //@human / @human Le gestionnaire par défaut refuse explicitement (pas de succès silencieux).
    #[test]
    fn test_default_sync_manager_refuse() {
        let mut mgr = DefaultSyncManager::new();
        assert!(matches!(
            mgr.sync(SyncStrategy::Immediate),
            Err(SyncError::NonImplemente(_))
        ));
        assert!(matches!(
            mgr.sync(SyncStrategy::Deferred),
            Err(SyncError::NonImplemente(_))
        ));
    }
}
