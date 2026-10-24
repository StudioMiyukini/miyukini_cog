//! Tools transaction : tool.transaction.begin, commit, rollback.
//!
//! Gestion de transactions sous autorité KindMother, sans décision (BOUND-2).

use crate::context::GovernedContext;
use crate::errors::MiyuSQLError;

/// @id miyusql_tool_tx_begin
/// @role mutator
/// @layer tool
/// @do begin_transaction_under_governance
//@human / @human Démarre une transaction sur la connexion gouvernée.
/// tool.transaction.begin
pub fn begin(
    ctx: &GovernedContext,
    executor: &dyn TransactionExecutor,
) -> Result<String, MiyuSQLError> {
    ctx.verifier_mandat()?;
    executor.begin(ctx)
}

/// @id miyusql_tool_tx_commit
/// @role mutator
/// @layer tool
/// @do commit_transaction
//@human / @human Valide (commit) la transaction en cours.
/// tool.transaction.commit
pub fn commit(
    ctx: &GovernedContext,
    tx_id: &str,
    executor: &dyn TransactionExecutor,
) -> Result<(), MiyuSQLError> {
    ctx.verifier_mandat()?;
    executor.commit(ctx, tx_id)
}

/// @id miyusql_tool_tx_rollback
/// @role mutator
/// @layer tool
/// @do rollback_transaction
//@human / @human Annule (rollback) la transaction en cours.
/// tool.transaction.rollback
pub fn rollback(
    ctx: &GovernedContext,
    tx_id: &str,
    executor: &dyn TransactionExecutor,
) -> Result<(), MiyuSQLError> {
    ctx.verifier_mandat()?;
    executor.rollback(ctx, tx_id)
}

/// @id miyusql_transaction_executor_trait
/// @role infrastructure
/// @layer tool
/// @do define_transaction_executor
//@human / @human Trait d'exécution de transactions (abstraction backend).
pub trait TransactionExecutor: Send + Sync {
    /// @id miyusql_tx_begin_impl
    /// @role mutator
    /// @layer tool
    /// @do start_transaction
    //@human / @human Démarre une transaction, retourne un identifiant.
    fn begin(&self, ctx: &GovernedContext) -> Result<String, MiyuSQLError>;

    /// @id miyusql_tx_commit_impl
    /// @role mutator
    /// @layer tool
    /// @do commit_transaction
    //@human / @human Valide la transaction.
    fn commit(&self, ctx: &GovernedContext, tx_id: &str) -> Result<(), MiyuSQLError>;

    /// @id miyusql_tx_rollback_impl
    /// @role mutator
    /// @layer tool
    /// @do rollback_transaction
    //@human / @human Annule la transaction.
    fn rollback(&self, ctx: &GovernedContext, tx_id: &str) -> Result<(), MiyuSQLError>;
}
