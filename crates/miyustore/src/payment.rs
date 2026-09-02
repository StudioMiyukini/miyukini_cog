//! Tools MiyuStore — tool.commerce.payment.* (capture, refund, status).
//! Store en mémoire : payment_ref -> status (captured, refunded, pending).

use crate::context::GovernedContext;
use crate::errors::MiyustoreError;
use std::collections::HashMap;
use std::sync::Mutex;

static PAYMENTS: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();

fn payments() -> &'static Mutex<HashMap<String, String>> {
    PAYMENTS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// @id miyustore_tool_commerce_payment_capture
/// @role mutator
/// @layer tool
/// @do commerce_payment_capture_under_governance
//@human / @human Capture un paiement ; autorisation StrongFather.
pub fn capture(
    ctx: &GovernedContext,
    payment_ref: &str,
    _payload: &str,
) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    let mut guard = payments()
        .lock()
        .map_err(|_| MiyustoreError::InvalidInput("lock".into()))?;
    guard.insert(payment_ref.to_string(), "captured".to_string());
    Ok(payment_ref.to_string())
}

/// @id miyustore_tool_commerce_payment_refund
/// @role mutator
/// @layer tool
/// @do commerce_payment_refund_under_governance
//@human / @human Rembourse un paiement ; décision StrongFather ; WriteIntent si état géré.
pub fn refund(
    ctx: &GovernedContext,
    payment_ref: &str,
    _payload: Option<&str>,
) -> Result<(), MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    let mut guard = payments()
        .lock()
        .map_err(|_| MiyustoreError::InvalidInput("lock".into()))?;
    guard.insert(payment_ref.to_string(), "refunded".to_string());
    Ok(())
}

/// @id miyustore_tool_commerce_payment_status
/// @role accessor
/// @layer tool
/// @do commerce_payment_status_under_governance
//@human / @human Retourne le statut d'un paiement ; lecture gouvernée.
pub fn status(ctx: &GovernedContext, payment_ref: &str) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    let guard = payments()
        .lock()
        .map_err(|_| MiyustoreError::InvalidInput("lock".into()))?;
    Ok(guard
        .get(payment_ref)
        .cloned()
        .unwrap_or_else(|| "pending".to_string()))
}
