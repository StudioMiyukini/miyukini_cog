//! Tools MWS MiyuWebwayParticipant — mws.cog_list.* (get, update, merge).
//! Liste locale de COGs ; lecture ; mise à jour ; fusion (règle fournie par Cores).

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayParticipantError;
use crate::store;

/// @id: miyuwebway_participant_mws_cog_list_get
/// @role: accessor
/// @layer: tool
/// @human: Lit la liste locale de COGs ; lecture.
/// @do: mws_cog_list_get_under_governance
pub fn get(ctx: &GovernedContext) -> Result<Vec<String>, MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    let guard = store::cog_list().lock().map_err(|e| MiyuwebwayParticipantError::ConnectionFailed(e.to_string()))?;
    Ok(guard.keys().cloned().collect())
}

/// @id: miyuwebway_participant_mws_cog_list_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour une entrée dans la liste locale ; écriture liste locale.
/// @do: mws_cog_list_update_under_governance
pub fn update(ctx: &GovernedContext, entry: &str, payload: &str) -> Result<(), MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    let mut guard = store::cog_list().lock().map_err(|e| MiyuwebwayParticipantError::ConnectionFailed(e.to_string()))?;
    guard.insert(entry.to_string(), payload.to_string());
    Ok(())
}

/// @id: miyuwebway_participant_mws_cog_list_merge
/// @role: mutator
/// @layer: tool
/// @human: Fusionne une liste reçue avec la liste locale ; règle fournie par Cores.
/// @do: mws_cog_list_merge_under_governance
pub fn merge(ctx: &GovernedContext, incoming: &str, _rule_ref: &str) -> Result<(), MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    let mut guard = store::cog_list().lock().map_err(|e| MiyuwebwayParticipantError::ConnectionFailed(e.to_string()))?;
    for line in incoming.lines() {
        let line = line.trim();
        if !line.is_empty() {
            guard.insert(line.to_string(), String::new());
        }
    }
    Ok(())
}
