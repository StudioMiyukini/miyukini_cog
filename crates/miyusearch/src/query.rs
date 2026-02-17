//! Tool MiyuSearch — tool.search.query.execute.
//! Exécute une requête full-text (terme(s), filtres, options fournis) ; retourne identifiants et scores.
//! Lecture sur index en mémoire (dérivation KindMother) ; BOUND-3.

use crate::context::GovernedContext;
use crate::errors::MiyusearchError;
use crate::store;

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
    terms: &str,
    _filters: Option<&str>,
    _options: Option<&str>,
) -> Result<QueryResult, MiyusearchError> {
    if !ctx.has_mandate() {
        return Err(MiyusearchError::NoMandate);
    }
    let guard = store::index().lock().map_err(|_| MiyusearchError::InvalidInput("lock".into()))?;
    let search_terms: Vec<&str> = terms.split_whitespace().filter(|s| !s.is_empty()).collect();
    let mut ids = Vec::new();
    let mut scores = Vec::new();
    for (id, fields) in guard.iter() {
        let fields_lower = fields.to_lowercase();
        let match_count = search_terms
            .iter()
            .filter(|t| fields_lower.contains(&t.to_lowercase()))
            .count();
        if match_count == search_terms.len() {
            ids.push(id.clone());
            scores.push(1.0);
        }
    }
    Ok(QueryResult { ids, scores })
}
