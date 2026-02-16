//! Tools MWS MiyuWebwayTracker — mws.cog_list.* (get, update, merge, filter).
//! Liste locale de COGs (stub en mémoire) ; sync réelle déléguée à Origin / miyuwebway_participant.

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayTrackerError;
use std::sync::Mutex;

static COG_LIST: std::sync::OnceLock<Mutex<Vec<String>>> = std::sync::OnceLock::new();

fn cog_list() -> &'static Mutex<Vec<String>> {
    COG_LIST.get_or_init(|| Mutex::new(Vec::new()))
}

/// @id: miyuwebway_tracker_mws_cog_list_get
/// @role: accessor
/// @layer: tool
/// @human: Lit la liste locale de COGs ; lecture.
/// @do: mws_cog_list_get_under_governance
pub fn get(ctx: &GovernedContext) -> Result<Vec<String>, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    let list = cog_list();
    let guard = list.lock().map_err(|_| MiyuwebwayTrackerError::TrackerUnavailable("lock".into()))?;
    Ok(guard.clone())
}

/// @id: miyuwebway_tracker_mws_cog_list_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour une entrée dans la liste locale ; écriture liste locale.
/// @do: mws_cog_list_update_under_governance
pub fn update(ctx: &GovernedContext, entry: &str, _payload: &str) -> Result<(), MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    let list = cog_list();
    let mut guard = list.lock().map_err(|_| MiyuwebwayTrackerError::TrackerUnavailable("lock".into()))?;
    if !guard.contains(&entry.to_string()) {
        guard.push(entry.to_string());
    }
    Ok(())
}

/// @id: miyuwebway_tracker_mws_cog_list_merge
/// @role: mutator
/// @layer: tool
/// @human: Fusionne une liste reçue avec la liste locale ; règle fournie par Cores.
/// @do: mws_cog_list_merge_under_governance
pub fn merge(ctx: &GovernedContext, incoming: &str, _rule_ref: &str) -> Result<(), MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    let list = cog_list();
    let mut guard = list.lock().map_err(|_| MiyuwebwayTrackerError::TrackerUnavailable("lock".into()))?;
    let ids: Vec<String> = incoming.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    for id in ids {
        if !guard.contains(&id) {
            guard.push(id);
        }
    }
    Ok(())
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
    get(ctx)
}
