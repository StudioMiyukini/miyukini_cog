//! Tools MiyuPosPayment — tool.payment.terminal.authorize, capture.

use crate::context::GovernedContext;
use crate::errors::MiyupospaymentError;

/// @id: miyupospayment_tool_terminal_authorize
/// @role: mutator
/// @layer: tool
/// @human: Demande une autorisation à un terminal CB (données fournies).
/// @do: terminal_authorize_under_governance
/// tool.payment.terminal.authorize
pub fn authorize(
    ctx: &GovernedContext,
    _session_id: &str,
    _amount: f64,
    _terminal_id: &str,
) -> Result<TerminalAuthResult, MiyupospaymentError> {
    if !ctx.has_mandate() {
        return Err(MiyupospaymentError::NoMandate);
    }
    Ok(TerminalAuthResult::default())
}

/// @id: miyupospayment_tool_terminal_capture
/// @role: mutator
/// @layer: tool
/// @human: Confirme (capture) un paiement CB précédemment autorisé.
/// @do: terminal_capture_under_governance
/// tool.payment.terminal.capture
pub fn capture(ctx: &GovernedContext, _auth_id: &str) -> Result<String, MiyupospaymentError> {
    if !ctx.has_mandate() {
        return Err(MiyupospaymentError::NoMandate);
    }
    Ok("capture:stub".to_string())
}

/// Résultat autorisation terminal.
#[derive(Debug, Clone, Default)]
pub struct TerminalAuthResult {
    pub auth_id: String,
    pub approved: bool,
}
