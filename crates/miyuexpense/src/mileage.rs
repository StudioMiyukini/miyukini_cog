//! Tools MiyuExpense — tool.expense.mileage.calculate, tool.expense.mileage.export.
//! Indemnités kilométriques : calcul selon barème fourni ; export PDF/CSV administration.

use crate::context::GovernedContext;
use crate::errors::MiyuexpenseError;

/// @id: miyuexpense_tool_expense_mileage_calculate
/// @role: accessor
/// @layer: tool
/// @human: Calcule les indemnités kilométriques selon barème fourni ; exécution seule.
/// @do: expense_mileage_calculate_under_governance
pub fn calculate(ctx: &GovernedContext, _payload: &str, _scale_ref: Option<&str>) -> Result<String, MiyuexpenseError> {
    if !ctx.has_mandate() {
        return Err(MiyuexpenseError::NoMandate);
    }
    Err(MiyuexpenseError::Unimplemented)
}

/// @id: miyuexpense_tool_expense_mileage_export
/// @role: accessor
/// @layer: tool
/// @human: Export PDF/CSV des indemnités km pour administration ; exécution seule.
/// @do: expense_mileage_export_under_governance
pub fn export(ctx: &GovernedContext, _payload: &str, _format: &str) -> Result<Vec<u8>, MiyuexpenseError> {
    if !ctx.has_mandate() {
        return Err(MiyuexpenseError::NoMandate);
    }
    Err(MiyuexpenseError::Unimplemented)
}
