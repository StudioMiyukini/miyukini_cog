//! Tools MiyuPosSales — tool.pos.cash.register.open, close.

use crate::context::GovernedContext;
use crate::errors::MiyupossalesError;

/// @id: miyupossales_tool_cash_register_open
/// @role: mutator
/// @layer: tool
/// @human: Ouvre une session caisse (ouverture de tiroir) ; WriteIntent KindMother.
/// @do: cash_register_open_under_governance
/// tool.pos.cash.register.open
pub fn open(
    ctx: &GovernedContext,
    _store_id: &str,
    _register_id: &str,
) -> Result<String, MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Ok("session:stub".to_string())
}

/// @id: miyupossales_tool_cash_register_close
/// @role: mutator
/// @layer: tool
/// @human: Clôture une session caisse (comptage, écart) ; WriteIntent KindMother.
/// @do: cash_register_close_under_governance
/// tool.pos.cash.register.close
pub fn close(
    ctx: &GovernedContext,
    _session_id: &str,
    _count: &std::collections::HashMap<String, f64>,
) -> Result<String, MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Ok("session:stub".to_string())
}
