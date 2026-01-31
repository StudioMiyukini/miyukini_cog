//! Tool MiyuSearch — tool.search.index.update.
//! Mise à jour index (document, champs, identifiant fournis) ; flux gouverné / WriteIntent.

use crate::context::GovernedContext;
use crate::errors::MiyusearchError;

/// @id: miyusearch_tool_search_index_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour l'index ; document, champs, identifiant fournis dans le flux.
/// @do: search_index_update_under_governance
/// tool.search.index.update — ne décide pas ; critères fournis.
pub fn update(
    ctx: &GovernedContext,
    _document_id: &str,
    _fields: &str,
) -> Result<(), MiyusearchError> {
    if !ctx.has_mandate() {
        return Err(MiyusearchError::NoMandate);
    }
    Err(MiyusearchError::Unimplemented)
}
