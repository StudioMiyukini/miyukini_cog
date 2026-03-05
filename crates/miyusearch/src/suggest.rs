//! Tool MiyuSearch — tool.search.suggest.
//! Produit des suggestions (autocomplete) à partir d'un préfixe et options fournis.
//! Lecture sur index en mémoire ; BOUND-3.

use crate::context::GovernedContext;
use crate::errors::MiyusearchError;
use crate::store;
use std::collections::HashSet;

/// @id: miyusearch_tool_search_suggest
/// @role: accessor
/// @layer: tool
/// @human: Produit des suggestions autocomplete à partir d'un préfixe et options fournis.
/// @do: search_suggest_under_governance
/// tool.search.suggest — ne décide pas ; préfixe fourni.
pub fn suggest(
    ctx: &GovernedContext,
    prefix: &str,
    _options: Option<&str>,
) -> Result<Vec<String>, MiyusearchError> {
    if !ctx.has_mandate() {
        return Err(MiyusearchError::NoMandate);
    }
    let guard = store::index()
        .lock()
        .map_err(|_| MiyusearchError::InvalidInput("lock".into()))?;
    let prefix_lower = prefix.to_lowercase();
    let mut suggestions: HashSet<String> = HashSet::new();
    for fields in guard.values() {
        for word in fields.split_whitespace() {
            if word.to_lowercase().starts_with(&prefix_lower) && word.len() >= prefix.len() {
                suggestions.insert(word.to_string());
            }
        }
    }
    let mut out: Vec<String> = suggestions.into_iter().collect();
    out.sort();
    Ok(out)
}
