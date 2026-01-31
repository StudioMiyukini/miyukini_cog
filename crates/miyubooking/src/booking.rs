//! Tools MiyuBooking — tool.booking.create, tool.booking.update, tool.booking.cancel.
//! Réservations : create/update/cancel ; WriteIntent KindMother ; décision StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyubookingError;

/// @id: miyubooking_tool_booking_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une réservation ; WriteIntent KindMother ; décision StrongFather.
/// @do: booking_create_under_governance
pub fn create(ctx: &GovernedContext, _payload: &str) -> Result<String, MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    Err(MiyubookingError::Unimplemented)
}

/// @id: miyubooking_tool_booking_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour une réservation (déplacement, prolongation) ; WriteIntent KindMother.
/// @do: booking_update_under_governance
pub fn update(ctx: &GovernedContext, _booking_id: &str, _payload: &str) -> Result<(), MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    Err(MiyubookingError::Unimplemented)
}

/// @id: miyubooking_tool_booking_cancel
/// @role: mutator
/// @layer: tool
/// @human: Annule une réservation ; décision StrongFather ; WriteIntent KindMother.
/// @do: booking_cancel_under_governance
pub fn cancel(ctx: &GovernedContext, _booking_id: &str) -> Result<(), MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    Err(MiyubookingError::Unimplemented)
}
