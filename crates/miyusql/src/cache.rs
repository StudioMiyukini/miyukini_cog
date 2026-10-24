//! Tools cache : tool.cache.get, set, invalidate.
//!
//! Cache gouverné sans décision de politique (BOUND-2).

use crate::context::GovernedContext;
use crate::errors::MiyuSQLError;

/// @id miyusql_tool_cache_get
/// @role accessor
/// @layer tool
/// @do get_cache_entry
//@human / @human Récupère une entrée depuis le cache gouverné (clé fournie).
/// tool.cache.get
pub fn get(
    ctx: &GovernedContext,
    key: &str,
    executor: &dyn CacheExecutor,
) -> Result<Option<Vec<u8>>, MiyuSQLError> {
    ctx.verifier_mandat()?;
    executor.get(ctx, key)
}

/// @id miyusql_tool_cache_set
/// @role mutator
/// @layer tool
/// @do set_cache_entry
//@human / @human Enregistre une entrée dans le cache gouverné (clé, valeur, TTL optionnel).
/// tool.cache.set
pub fn set(
    ctx: &GovernedContext,
    key: &str,
    value: &[u8],
    ttl_secs: Option<u64>,
    executor: &dyn CacheExecutor,
) -> Result<(), MiyuSQLError> {
    ctx.verifier_mandat()?;
    executor.set(ctx, key, value, ttl_secs)
}

/// @id miyusql_tool_cache_invalidate
/// @role mutator
/// @layer tool
/// @do invalidate_cache_entries
//@human / @human Invalide une ou plusieurs entrées du cache (par clé ou motif).
/// tool.cache.invalidate
pub fn invalidate(
    ctx: &GovernedContext,
    key_or_pattern: &str,
    executor: &dyn CacheExecutor,
) -> Result<u64, MiyuSQLError> {
    ctx.verifier_mandat()?;
    executor.invalidate(ctx, key_or_pattern)
}

/// @id miyusql_cache_executor_trait
/// @role infrastructure
/// @layer tool
/// @do define_cache_executor
//@human / @human Trait d'exécution cache (abstraction backend).
pub trait CacheExecutor: Send + Sync {
    /// @id miyusql_cache_get_impl
    /// @role accessor
    /// @layer tool
    /// @do get_value
    //@human / @human Récupère la valeur pour la clé.
    fn get(&self, ctx: &GovernedContext, key: &str) -> Result<Option<Vec<u8>>, MiyuSQLError>;

    /// @id miyusql_cache_set_impl
    /// @role mutator
    /// @layer tool
    /// @do set_value
    //@human / @human Enregistre la valeur avec TTL optionnel.
    fn set(
        &self,
        ctx: &GovernedContext,
        key: &str,
        value: &[u8],
        ttl_secs: Option<u64>,
    ) -> Result<(), MiyuSQLError>;

    /// @id miyusql_cache_invalidate_impl
    /// @role mutator
    /// @layer tool
    /// @do invalidate_entries
    //@human / @human Invalide les entrées correspondant à la clé ou au motif.
    fn invalidate(&self, ctx: &GovernedContext, key_or_pattern: &str) -> Result<u64, MiyuSQLError>;
}
