//! Tools query : tool.query.execute, tool.query.prepare.
//!
//! Exécution et validation de requêtes sans logique métier (BOUND-1).

use crate::context::GovernedContext;
use crate::errors::MiyuSQLError;
use std::collections::HashMap;

/// @id: miyusql_query_result
/// @role: data
/// @layer: tool
/// @human: Résultat d'une requête (lignes, count).
/// @do: represent_query_result
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// @id: miyusql_query_result_rows
    /// @role: data
    /// @layer: tool
    /// @human: Lignes retournées (liste de maps colonne -> valeur).
    /// @do: store_rows
    pub rows: Vec<HashMap<String, String>>,
    /// @id: miyusql_query_result_count
    /// @role: data
    /// @layer: tool
    /// @human: Nombre de lignes affectées (pour INSERT/UPDATE/DELETE).
    /// @do: store_count
    pub count: u64,
}

/// @id: miyusql_tool_query_execute
/// @role: mutator
/// @layer: tool
/// @human: Exécute une requête (lecture ou écriture) selon le contexte gouverné.
/// @do: execute_query_under_governance
/// tool.query.execute — ne décide pas du contenu ni de l'autorisation.
pub fn execute(
    ctx: &GovernedContext,
    sql: &str,
    params: &[String],
    executor: &dyn QueryExecutor,
) -> Result<QueryResult, MiyuSQLError> {
    if !ctx.has_mandate() {
        return Err(MiyuSQLError::NoMandate);
    }
    executor.execute(ctx, sql, params)
}

/// @id: miyusql_tool_query_prepare
/// @role: accessor
/// @layer: tool
/// @human: Prépare ou valide une requête sans l'exécuter.
/// @do: prepare_query_without_execute
/// tool.query.prepare.
pub fn prepare(
    ctx: &GovernedContext,
    sql: &str,
    params: &[String],
    executor: &dyn QueryExecutor,
) -> Result<(), MiyuSQLError> {
    if !ctx.has_mandate() {
        return Err(MiyuSQLError::NoMandate);
    }
    executor.prepare(ctx, sql, params)
}

/// @id: miyusql_query_executor_trait
/// @role: infrastructure
/// @layer: tool
/// @human: Trait d'exécution de requêtes (abstraction backend).
/// @do: define_query_executor
pub trait QueryExecutor: Send + Sync {
    /// @id: miyusql_executor_execute
    /// @role: mutator
    /// @layer: tool
    /// @human: Exécute la requête sous contexte gouverné.
    /// @do: run_query
    fn execute(
        &self,
        ctx: &GovernedContext,
        sql: &str,
        params: &[String],
    ) -> Result<QueryResult, MiyuSQLError>;

    /// @id: miyusql_executor_prepare
    /// @role: accessor
    /// @layer: tool
    /// @human: Valide la requête sans exécution.
    /// @do: validate_query
    fn prepare(
        &self,
        _ctx: &GovernedContext,
        sql: &str,
        params: &[String],
    ) -> Result<(), MiyuSQLError> {
        if sql.trim().is_empty() {
            return Err(MiyuSQLError::Syntax("empty query".into()));
        }
        let _ = params;
        Ok(())
    }
}
