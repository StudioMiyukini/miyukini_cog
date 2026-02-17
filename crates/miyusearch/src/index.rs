//! Tool MiyuSearch — tool.search.index.update.
//! Mise à jour index (document, champs, identifiant fournis) ; flux gouverné / WriteIntent.
//! Store en mémoire (dérivation ; persistance réelle = KindMother en amont) ; BOUND-3.

use crate::context::GovernedContext;
use crate::errors::MiyusearchError;
use crate::store;

/// @id: miyusearch_tool_search_index_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour l'index ; document, champs, identifiant fournis dans le flux.
/// @do: search_index_update_under_governance
/// tool.search.index.update — ne décide pas ; critères fournis.
pub fn update(
    ctx: &GovernedContext,
    document_id: &str,
    fields: &str,
) -> Result<(), MiyusearchError> {
    if !ctx.has_mandate() {
        return Err(MiyusearchError::NoMandate);
    }
    let mut guard = store::index().lock().map_err(|_| MiyusearchError::InvalidInput("lock".into()))?;
    guard.insert(document_id.to_string(), fields.to_string());
    Ok(())
}
