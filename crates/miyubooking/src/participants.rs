//! Tool MiyuBooking — tool.booking.participants.compute.
//! Calcul places restantes / participants pour un créneau donné.

use crate::context::GovernedContext;
use crate::errors::MiyubookingError;

/// @id: miyubooking_tool_booking_participants_compute
/// @role: accessor
/// @layer: tool
/// @human: Calcule places restantes / participants pour un créneau donné.
/// @do: booking_participants_compute_under_governance
pub fn compute(ctx: &GovernedContext, _slot_id: &str) -> Result<String, MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    Ok("0".to_string())
}
