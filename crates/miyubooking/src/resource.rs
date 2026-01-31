//! Tools MiyuBooking — tool.booking.resource.resolve, tool.booking.resource.availability.
//! Ressources (salle, équipement) : résolution ; disponibilité sur plage donnée.

use crate::context::GovernedContext;
use crate::errors::MiyubookingError;

/// @id: miyubooking_tool_booking_resource_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout une ressource (salle, équipement) ; contraintes fournies par KindMother.
/// @do: booking_resource_resolve_under_governance
pub fn resolve(ctx: &GovernedContext, _constraints: Option<&str>) -> Result<String, MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    Err(MiyubookingError::Unimplemented)
}

/// @id: miyubooking_tool_booking_resource_availability
/// @role: accessor
/// @layer: tool
/// @human: Retourne la disponibilité d'une ressource sur une plage donnée ; lecture gouvernée.
/// @do: booking_resource_availability_under_governance
pub fn availability(ctx: &GovernedContext, _resource_id: &str, _range: &str) -> Result<String, MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    Err(MiyubookingError::Unimplemented)
}
