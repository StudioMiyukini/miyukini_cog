//! Tool MiyuSearch — tool.search.suggest.
//! Produit des suggestions (autocomplete) à partir d'un préfixe et options fournis.
//! Implémentation minimale : liste vide sans backend (lecture = KindMother ou flux) ; BOUND-3.

use crate::context::GovernedContext;
use crate::errors::MiyusearchError;

/// @id: miyusearch_tool_search_suggest
/// @role: accessor
/// @layer: tool
/// @human: Produit des suggestions autocomplete à partir d'un préfixe et options fournis.
/// @do: search_suggest_under_governance
/// tool.search.suggest — ne décide pas ; préfixe fourni.
pub fn suggest(
    ctx: &GovernedContext,
    _prefix: &str,
    _options: Option<&str>,
) -> Result<Vec<String>, MiyusearchError> {
    if !ctx.has_mandate() {
        return Err(MiyusearchError::NoMandate);
    }
    Ok(Vec::new())
}
