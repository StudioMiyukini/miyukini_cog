//! Tools MiyuBooking — tool.booking.slots.list, tool.booking.slots.resolve.
//! Créneaux : liste disponible ; résolution par identifiant ; contexte ressource, date, durée fourni.

use crate::context::GovernedContext;
use crate::errors::MiyubookingError;

/// @id: miyubooking_tool_booking_slots_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les créneaux disponibles ; contexte ressource, date, durée fourni.
/// @do: booking_slots_list_under_governance
pub fn list(
    ctx: &GovernedContext,
    _resource_ref: &str,
    _date_range: &str,
    _duration_minutes: Option<u32>,
) -> Result<Vec<String>, MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    Err(MiyubookingError::Unimplemented)
}

/// @id: miyubooking_tool_booking_slots_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout un créneau par identifiant ; lecture.
/// @do: booking_slots_resolve_under_governance
pub fn resolve(ctx: &GovernedContext, _slot_id: &str) -> Result<String, MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    Err(MiyubookingError::Unimplemented)
}
