//! Tool MiyuPosSales — tool.pos.cash.movement.record.

use crate::context::GovernedContext;
use crate::errors::MiyupossalesError;

/// @id: miyupossales_tool_cash_movement_record
/// @role: mutator
/// @layer: tool
/// @human: Enregistre un mouvement espèces (entrée/sortie) ; WriteIntent KindMother.
/// @do: cash_movement_record_under_governance
/// tool.pos.cash.movement.record
pub fn record(
    ctx: &GovernedContext,
    _session_id: &str,
    _direction: &str,
    _amount: f64,
    _reason: &str,
) -> Result<String, MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Err(MiyupossalesError::Unimplemented)
}
