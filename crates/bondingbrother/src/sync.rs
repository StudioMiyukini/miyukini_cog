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
}

impl std::fmt::Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncError::Failed(msg) => write!(f, "Sync failed: {msg}"),
        }
    }
}

impl std::error::Error for SyncError {}

/// Implémentation par défaut : exécution de la stratégie sans persistance (no-op ou marquage).
#[derive(Debug, Default)]
pub struct DefaultSyncManager;

impl DefaultSyncManager {
    /// Crée un gestionnaire de sync.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl SyncManager for DefaultSyncManager {
    fn sync(&mut self, _strategy: SyncStrategy) -> Result<(), SyncError> {
        Ok(())
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

    #[test]
    fn test_default_sync_manager() {
        use super::*;
        let mut mgr = DefaultSyncManager::new();
        assert!(mgr.sync(SyncStrategy::Immediate).is_ok());
        assert!(mgr.sync(SyncStrategy::Deferred).is_ok());
    }
}
