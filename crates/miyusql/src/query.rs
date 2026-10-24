//! Tools query : tool.query.execute, tool.query.prepare.
//!
//! Exécution et validation de requêtes sans logique métier (BOUND-1).

use crate::context::GovernedContext;
use crate::errors::MiyuSQLError;
use std::collections::HashMap;

/// @id miyusql_query_result
/// @role data
/// @layer tool
/// @do represent_query_result
//@human / @human Résultat d'une requête (lignes, count).
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// @id miyusql_query_result_rows
    /// @role data
    /// @layer tool
    /// @do store_rows
    //@human / @human Lignes retournées (liste de maps colonne -> valeur).
    pub rows: Vec<HashMap<String, String>>,
    /// @id miyusql_query_result_count
    /// @role data
    /// @layer tool
    /// @do store_count
    //@human / @human Nombre de lignes affectées (pour INSERT/UPDATE/DELETE).
    pub count: u64,
}

/// @id miyusql_tool_query_execute
/// @role mutator
/// @layer tool
/// @do execute_query_under_governance
//@human / @human Exécute une requête (lecture ou écriture) selon le contexte gouverné.
/// tool.query.execute — ne décide pas du contenu ni de l'autorisation.
pub fn execute(
    ctx: &GovernedContext,
    sql: &str,
    params: &[String],
    executor: &dyn QueryExecutor,
) -> Result<QueryResult, MiyuSQLError> {
    ctx.verifier_mandat()?;
    executor.execute(ctx, sql, params)
}

/// @id miyusql_tool_query_prepare
/// @role accessor
/// @layer tool
/// @do prepare_query_without_execute
//@human / @human Prépare ou valide une requête sans l'exécuter.
/// tool.query.prepare.
pub fn prepare(
    ctx: &GovernedContext,
    sql: &str,
    params: &[String],
    executor: &dyn QueryExecutor,
) -> Result<(), MiyuSQLError> {
    ctx.verifier_mandat()?;
    executor.prepare(ctx, sql, params)
}

/// @id miyusql_query_executor_trait
/// @role infrastructure
/// @layer tool
/// @do define_query_executor
//@human / @human Trait d'exécution de requêtes (abstraction backend).
pub trait QueryExecutor: Send + Sync {
    /// @id miyusql_executor_execute
    /// @role mutator
    /// @layer tool
    /// @do run_query
    //@human / @human Exécute la requête sous contexte gouverné.
    fn execute(
        &self,
        ctx: &GovernedContext,
        sql: &str,
        params: &[String],
    ) -> Result<QueryResult, MiyuSQLError>;

    /// @id miyusql_executor_prepare
    /// @role accessor
    /// @layer tool
    /// @do validate_query
    //@human / @human Valide la requête sans exécution. Par défaut : refus explicite
    //@human (C-14) — un exécuteur doit fournir sa propre validation, sinon aucune
    //@human requête n'est déclarée valide sans avoir été analysée.
    fn prepare(
        &self,
        _ctx: &GovernedContext,
        sql: &str,
        _params: &[String],
    ) -> Result<(), MiyuSQLError> {
        if sql.trim().is_empty() {
            return Err(MiyuSQLError::Syntax("empty query".into()));
        }
        Err(MiyuSQLError::NonImplemente(
            "validation de requête non fournie par cet exécuteur".into(),
        ))
    }
}
