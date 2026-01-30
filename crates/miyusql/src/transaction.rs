//! Tools transaction : tool.transaction.begin, commit, rollback.
//!
//! Gestion de transactions sous autorité KindMother, sans décision (BOUND-2).

use crate::context::GovernedContext;
use crate::errors::MiyuSQLError;

/// @id: miyusql_tool_tx_begin
/// @role: mutator
/// @layer: tool
/// @human: Démarre une transaction sur la connexion gouvernée.
/// @do: begin_transaction_under_governance
/// tool.transaction.begin
pub fn begin(ctx: &GovernedContext, executor: &dyn TransactionExecutor) -> Result<String, MiyuSQLError> {
    if !ctx.has_mandate() {
        return Err(MiyuSQLError::NoMandate);
    }
    executor.begin(ctx)
}

/// @id: miyusql_tool_tx_commit
/// @role: mutator
/// @layer: tool
/// @human: Valide (commit) la transaction en cours.
/// @do: commit_transaction
/// tool.transaction.commit
pub fn commit(ctx: &GovernedContext, tx_id: &str, executor: &dyn TransactionExecutor) -> Result<(), MiyuSQLError> {
    if !ctx.has_mandate() {
        return Err(MiyuSQLError::NoMandate);
    }
    executor.commit(ctx, tx_id)
}

/// @id: miyusql_tool_tx_rollback
/// @role: mutator
/// @layer: tool
/// @human: Annule (rollback) la transaction en cours.
/// @do: rollback_transaction
/// tool.transaction.rollback
pub fn rollback(ctx: &GovernedContext, tx_id: &str, executor: &dyn TransactionExecutor) -> Result<(), MiyuSQLError> {
    if !ctx.has_mandate() {
        return Err(MiyuSQLError::NoMandate);
    }
    executor.rollback(ctx, tx_id)
}

/// @id: miyusql_transaction_executor_trait
/// @role: infrastructure
/// @layer: tool
/// @human: Trait d'exécution de transactions (abstraction backend).
/// @do: define_transaction_executor
pub trait TransactionExecutor: Send + Sync {
    /// @id: miyusql_tx_begin_impl
    /// @role: mutator
    /// @layer: tool
    /// @human: Démarre une transaction, retourne un identifiant.
    /// @do: start_transaction
    fn begin(&self, ctx: &GovernedContext) -> Result<String, MiyuSQLError>;

    /// @id: miyusql_tx_commit_impl
    /// @role: mutator
    /// @layer: tool
    /// @human: Valide la transaction.
    /// @do: commit_transaction
    fn commit(&self, ctx: &GovernedContext, tx_id: &str) -> Result<(), MiyuSQLError>;

    /// @id: miyusql_tx_rollback_impl
    /// @role: mutator
    /// @layer: tool
    /// @human: Annule la transaction.
    /// @do: rollback_transaction
    fn rollback(&self, ctx: &GovernedContext, tx_id: &str) -> Result<(), MiyuSQLError>;
}
