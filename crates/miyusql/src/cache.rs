//! Tools cache : tool.cache.get, set, invalidate.
//!
//! Cache gouverné sans décision de politique (BOUND-2).

use crate::context::GovernedContext;
use crate::errors::MiyuSQLError;

/// @id: miyusql_tool_cache_get
/// @role: accessor
/// @layer: tool
/// @human: Récupère une entrée depuis le cache gouverné (clé fournie).
/// @do: get_cache_entry
/// tool.cache.get
pub fn get(ctx: &GovernedContext, key: &str, executor: &dyn CacheExecutor) -> Result<Option<Vec<u8>>, MiyuSQLError> {
    if !ctx.has_mandate() {
        return Err(MiyuSQLError::NoMandate);
    }
    executor.get(ctx, key)
}

/// @id: miyusql_tool_cache_set
/// @role: mutator
/// @layer: tool
/// @human: Enregistre une entrée dans le cache gouverné (clé, valeur, TTL optionnel).
/// @do: set_cache_entry
/// tool.cache.set
pub fn set(
    ctx: &GovernedContext,
    key: &str,
    value: &[u8],
    ttl_secs: Option<u64>,
    executor: &dyn CacheExecutor,
) -> Result<(), MiyuSQLError> {
    if !ctx.has_mandate() {
        return Err(MiyuSQLError::NoMandate);
    }
    executor.set(ctx, key, value, ttl_secs)
}

/// @id: miyusql_tool_cache_invalidate
/// @role: mutator
/// @layer: tool
/// @human: Invalide une ou plusieurs entrées du cache (par clé ou motif).
/// @do: invalidate_cache_entries
/// tool.cache.invalidate
pub fn invalidate(ctx: &GovernedContext, key_or_pattern: &str, executor: &dyn CacheExecutor) -> Result<u64, MiyuSQLError> {
    if !ctx.has_mandate() {
        return Err(MiyuSQLError::NoMandate);
    }
    executor.invalidate(ctx, key_or_pattern)
}

/// @id: miyusql_cache_executor_trait
/// @role: infrastructure
/// @layer: tool
/// @human: Trait d'exécution cache (abstraction backend).
/// @do: define_cache_executor
pub trait CacheExecutor: Send + Sync {
    /// @id: miyusql_cache_get_impl
    /// @role: accessor
    /// @layer: tool
    /// @human: Récupère la valeur pour la clé.
    /// @do: get_value
    fn get(&self, ctx: &GovernedContext, key: &str) -> Result<Option<Vec<u8>>, MiyuSQLError>;

    /// @id: miyusql_cache_set_impl
    /// @role: mutator
    /// @layer: tool
    /// @human: Enregistre la valeur avec TTL optionnel.
    /// @do: set_value
    fn set(
        &self,
        ctx: &GovernedContext,
        key: &str,
        value: &[u8],
        ttl_secs: Option<u64>,
    ) -> Result<(), MiyuSQLError>;

    /// @id: miyusql_cache_invalidate_impl
    /// @role: mutator
    /// @layer: tool
    /// @human: Invalide les entrées correspondant à la clé ou au motif.
    /// @do: invalidate_entries
    fn invalidate(&self, ctx: &GovernedContext, key_or_pattern: &str) -> Result<u64, MiyuSQLError>;
}
