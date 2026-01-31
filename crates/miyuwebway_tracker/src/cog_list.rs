//! Tools MWS MiyuWebwayTracker — mws.cog_list.* (get, update, merge, filter).
//! Liste locale de COGs ; lecture ; mise à jour ; fusion ; filtre (critère fourni par Cores).

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayTrackerError;

/// @id: miyuwebway_tracker_mws_cog_list_get
/// @role: accessor
/// @layer: tool
/// @human: Lit la liste locale de COGs ; lecture.
/// @do: mws_cog_list_get_under_governance
pub fn get(ctx: &GovernedContext) -> Result<Vec<String>, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::Unimplemented)
}

/// @id: miyuwebway_tracker_mws_cog_list_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour une entrée dans la liste locale ; écriture liste locale.
/// @do: mws_cog_list_update_under_governance
pub fn update(ctx: &GovernedContext, _entry: &str, _payload: &str) -> Result<(), MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::Unimplemented)
}

/// @id: miyuwebway_tracker_mws_cog_list_merge
/// @role: mutator
/// @layer: tool
/// @human: Fusionne une liste reçue avec la liste locale ; règle fournie par Cores.
/// @do: mws_cog_list_merge_under_governance
pub fn merge(ctx: &GovernedContext, _incoming: &str, _rule_ref: &str) -> Result<(), MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::Unimplemented)
}

/// @id: miyuwebway_tracker_mws_cog_list_filter
/// @role: accessor
/// @layer: tool
/// @human: Filtre la liste selon critère ; critère fourni par Border Guard, WorrySentinel.
/// @do: mws_cog_list_filter_under_governance
pub fn filter(ctx: &GovernedContext, _criterion_ref: &str) -> Result<Vec<String>, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::Unimplemented)
}
