//! Tools MiyuExpense — tool.expense.receipt.capture, tool.expense.receipt.extract.
//! Justificatifs : enregistrement (photo/scan) WriteIntent ; extraction OCR (exécution seule).

use crate::context::GovernedContext;
use crate::errors::MiyuexpenseError;

/// @id: miyuexpense_tool_expense_receipt_capture
/// @role: mutator
/// @layer: tool
/// @human: Enregistre un justificatif (photo/scan) ; WriteIntent KindMother.
/// @do: expense_receipt_capture_under_governance
pub fn capture(ctx: &GovernedContext, _claim_ref: &str, _payload: &str) -> Result<String, MiyuexpenseError> {
    if !ctx.has_mandate() {
        return Err(MiyuexpenseError::NoMandate);
    }
    Ok("receipt:stub".to_string())
}

/// @id: miyuexpense_tool_expense_receipt_extract
/// @role: accessor
/// @layer: tool
/// @human: Extrait les données d'un justificatif par OCR ; exécution seule.
/// @do: expense_receipt_extract_under_governance
pub fn extract(ctx: &GovernedContext, _receipt_ref: &str) -> Result<String, MiyuexpenseError> {
    if !ctx.has_mandate() {
        return Err(MiyuexpenseError::NoMandate);
    }
    Ok("{}".to_string())
}
