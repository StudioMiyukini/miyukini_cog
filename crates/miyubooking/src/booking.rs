//! Tools MiyuBooking — tool.booking.create, tool.booking.update, tool.booking.cancel.
//! Store en mémoire (KindMother côté produit pour persistance).

use crate::context::GovernedContext;
use crate::errors::MiyubookingError;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};
use std::collections::HashMap;
use std::sync::Mutex;

static BOOKINGS: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();

fn bookings() -> &'static Mutex<HashMap<String, String>> {
    BOOKINGS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// @id: miyubooking_tool_booking_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une réservation ; WriteIntent KindMother ; décision StrongFather.
/// @do: booking_create_under_governance
pub fn create(ctx: &GovernedContext, payload: &str) -> Result<String, MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    let id = format!("book:{}", UuidIdGenerator.generate());
    let mut guard = bookings()
        .lock()
        .map_err(|_| MiyubookingError::InvalidInput("lock".into()))?;
    guard.insert(id.clone(), payload.to_string());
    Ok(id)
}

/// @id: miyubooking_tool_booking_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour une réservation (déplacement, prolongation) ; WriteIntent KindMother.
/// @do: booking_update_under_governance
pub fn update(
    ctx: &GovernedContext,
    booking_id: &str,
    payload: &str,
) -> Result<(), MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    let mut guard = bookings()
        .lock()
        .map_err(|_| MiyubookingError::InvalidInput("lock".into()))?;
    guard.insert(booking_id.to_string(), payload.to_string());
    Ok(())
}

/// @id: miyubooking_tool_booking_cancel
/// @role: mutator
/// @layer: tool
/// @human: Annule une réservation ; décision StrongFather ; WriteIntent KindMother.
/// @do: booking_cancel_under_governance
pub fn cancel(ctx: &GovernedContext, booking_id: &str) -> Result<(), MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    let mut guard = bookings()
        .lock()
        .map_err(|_| MiyubookingError::InvalidInput("lock".into()))?;
    guard.remove(booking_id);
    Ok(())
}
