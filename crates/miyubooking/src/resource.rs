//! Tools MiyuBooking — tool.booking.resource.resolve, tool.booking.resource.availability.
//! Ressources (salle, équipement) : résolution ; disponibilité sur plage donnée. Store en mémoire.

use crate::context::GovernedContext;
use crate::errors::MiyubookingError;
use std::collections::HashMap;
use std::sync::Mutex;

static RESOURCES: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();

fn resources_store() -> &'static Mutex<HashMap<String, String>> {
    RESOURCES.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Enregistre une ressource (metadata). Pour alimentation par flux / adaptateur.
pub fn register_resource(
    ctx: &GovernedContext,
    resource_id: &str,
    metadata: &str,
) -> Result<(), MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    let mut guard = resources_store()
        .lock()
        .map_err(|_| MiyubookingError::InvalidInput("lock".into()))?;
    guard.insert(resource_id.to_string(), metadata.to_string());
    Ok(())
}

/// @id: miyubooking_tool_booking_resource_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout une ressource (salle, équipement) ; contraintes fournies par KindMother.
/// @do: booking_resource_resolve_under_governance
pub fn resolve(
    ctx: &GovernedContext,
    _constraints: Option<&str>,
) -> Result<String, MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    let guard = resources_store()
        .lock()
        .map_err(|_| MiyubookingError::InvalidInput("lock".into()))?;
    if let Some(id) = guard.keys().next() {
        return Ok(id.clone());
    }
    Ok("res:default".to_string())
}

/// @id: miyubooking_tool_booking_resource_availability
/// @role: accessor
/// @layer: tool
/// @human: Retourne la disponibilité d'une ressource sur une plage donnée ; lecture gouvernée.
/// @do: booking_resource_availability_under_governance
pub fn availability(
    ctx: &GovernedContext,
    _resource_id: &str,
    _range: &str,
) -> Result<String, MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    Ok("[]".to_string())
}
