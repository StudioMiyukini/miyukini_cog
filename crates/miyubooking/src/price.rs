//! Tool MiyuBooking — tool.booking.price.compute.
//! Calcul du prix d'une réservation ; règles fournies dans le flux.

use crate::context::GovernedContext;
use crate::errors::MiyubookingError;

/// @id: miyubooking_tool_booking_price_compute
/// @role: accessor
/// @layer: tool
/// @human: Calcule le prix d'une réservation ; règles fournies dans le flux.
/// @do: booking_price_compute_under_governance
pub fn compute(ctx: &GovernedContext, _booking_ref: &str, _rules: Option<&str>) -> Result<String, MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    Ok("0".to_string())
}
