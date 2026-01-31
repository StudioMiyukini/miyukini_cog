//! Tools MiyuStore — tool.commerce.payment.* (capture, refund, status).
//! Paiement : capture/rembours = autorisation StrongFather ; statut = lecture gouvernée.

use crate::context::GovernedContext;
use crate::errors::MiyustoreError;

/// @id: miyustore_tool_commerce_payment_capture
/// @role: mutator
/// @layer: tool
/// @human: Capture un paiement ; autorisation StrongFather.
/// @do: commerce_payment_capture_under_governance
pub fn capture(ctx: &GovernedContext, _payment_ref: &str, _payload: &str) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
}

/// @id: miyustore_tool_commerce_payment_refund
/// @role: mutator
/// @layer: tool
/// @human: Rembourse un paiement ; décision StrongFather ; WriteIntent si état géré.
/// @do: commerce_payment_refund_under_governance
pub fn refund(ctx: &GovernedContext, _payment_ref: &str, _payload: Option<&str>) -> Result<(), MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
}

/// @id: miyustore_tool_commerce_payment_status
/// @role: accessor
/// @layer: tool
/// @human: Retourne le statut d'un paiement ; lecture gouvernée.
/// @do: commerce_payment_status_under_governance
pub fn status(ctx: &GovernedContext, _payment_ref: &str) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
}
