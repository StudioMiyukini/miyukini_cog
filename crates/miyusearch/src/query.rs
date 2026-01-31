//! Tool MiyuSearch — tool.search.query.execute.
//! Exécute une requête full-text (terme(s), filtres, options fournis) ; retourne identifiants et scores.

use crate::context::GovernedContext;
use crate::errors::MiyusearchError;

/// Résultat de requête (identifiants et scores).
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// Identifiants des documents trouvés.
    pub ids: Vec<String>,
    /// Scores associés (ordre aligné avec ids).
    pub scores: Vec<f64>,
}

/// @id: miyusearch_tool_search_query_execute
/// @role: accessor
/// @layer: tool
/// @human: Exécute une requête full-text ; terme(s), filtres, options fournis.
/// @do: search_query_execute_under_governance
/// tool.search.query.execute — ne décide pas ; critères fournis.
pub fn execute(
    ctx: &GovernedContext,
    _terms: &str,
    _filters: Option<&str>,
    _options: Option<&str>,
) -> Result<QueryResult, MiyusearchError> {
    if !ctx.has_mandate() {
        return Err(MiyusearchError::NoMandate);
    }
    Err(MiyusearchError::Unimplemented)
}
