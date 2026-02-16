//! Tool MWS MiyuWebwayTracker — mws.address.tracker_default.
//! Résout l'adresse Tracker (port 21000) ; exécution seule ; déterministe.

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayTrackerError;

/// @id: miyuwebway_tracker_mws_address_tracker_default
/// @role: accessor
/// @layer: tool
/// @human: Résout l'adresse Tracker (port 21000) ; exécution seule ; déterministe.
/// @do: mws_address_tracker_default_under_governance
/// Adresse par défaut du Tracker (port 21000). En production, préférer config ou env.
pub fn tracker_default(ctx: &GovernedContext, host: Option<&str>) -> Result<String, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    let h = host.unwrap_or("127.0.0.1").trim();
    if h.is_empty() {
        return Ok("127.0.0.1:21000".to_string());
    }
    Ok(format!("{h}:21000"))
}
