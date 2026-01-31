//! Tools MWS MiyuWebwayParticipant — mws.cog_list.* (get, update, merge).
//! Liste locale de COGs ; lecture ; mise à jour ; fusion (règle fournie par Cores).

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayParticipantError;

/// @id: miyuwebway_participant_mws_cog_list_get
/// @role: accessor
/// @layer: tool
/// @human: Lit la liste locale de COGs ; lecture.
/// @do: mws_cog_list_get_under_governance
pub fn get(ctx: &GovernedContext) -> Result<Vec<String>, MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    Err(MiyuwebwayParticipantError::Unimplemented)
}

/// @id: miyuwebway_participant_mws_cog_list_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour une entrée dans la liste locale ; écriture liste locale.
/// @do: mws_cog_list_update_under_governance
pub fn update(ctx: &GovernedContext, _entry: &str, _payload: &str) -> Result<(), MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    Err(MiyuwebwayParticipantError::Unimplemented)
}

/// @id: miyuwebway_participant_mws_cog_list_merge
/// @role: mutator
/// @layer: tool
/// @human: Fusionne une liste reçue avec la liste locale ; règle fournie par Cores.
/// @do: mws_cog_list_merge_under_governance
pub fn merge(ctx: &GovernedContext, _incoming: &str, _rule_ref: &str) -> Result<(), MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    Err(MiyuwebwayParticipantError::Unimplemented)
}
