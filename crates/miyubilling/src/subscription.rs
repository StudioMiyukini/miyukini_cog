//! Tools MiyuBilling — tool.billing.subscription.* (create, update, cancel, status).
//! Store en mémoire (KindMother côté produit pour persistance).

use crate::context::GovernedContext;
use crate::errors::MiyubillingError;
use std::collections::HashMap;
use std::sync::Mutex;

static SUBSCRIPTIONS: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();

fn subscriptions() -> &'static Mutex<HashMap<String, String>> {
    SUBSCRIPTIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// @id: miyubilling_tool_billing_subscription_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une souscription ; WriteIntent KindMother ; décision StrongFather.
/// @do: billing_subscription_create_under_governance
pub fn create(ctx: &GovernedContext, payload: &str) -> Result<String, MiyubillingError> {
    if !ctx.has_mandate() {
        return Err(MiyubillingError::NoMandate);
    }
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let id = format!("sub:{}", NEXT.fetch_add(1, Ordering::Relaxed));
    let mut guard = subscriptions().lock().map_err(|_| MiyubillingError::LockPoisoned)?;
    guard.insert(id.clone(), payload.to_string());
    Ok(id)
}

/// @id: miyubilling_tool_billing_subscription_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour une souscription ; WriteIntent KindMother.
/// @do: billing_subscription_update_under_governance
pub fn update(ctx: &GovernedContext, subscription_id: &str, payload: &str) -> Result<(), MiyubillingError> {
    if !ctx.has_mandate() {
        return Err(MiyubillingError::NoMandate);
    }
    let mut guard = subscriptions().lock().map_err(|_| MiyubillingError::LockPoisoned)?;
    guard.insert(subscription_id.to_string(), payload.to_string());
    Ok(())
}

/// @id: miyubilling_tool_billing_subscription_cancel
/// @role: mutator
/// @layer: tool
/// @human: Annule ou résilie une souscription ; décision StrongFather ; WriteIntent KindMother.
/// @do: billing_subscription_cancel_under_governance
pub fn cancel(ctx: &GovernedContext, subscription_id: &str) -> Result<(), MiyubillingError> {
    if !ctx.has_mandate() {
        return Err(MiyubillingError::NoMandate);
    }
    let mut guard = subscriptions().lock().map_err(|_| MiyubillingError::LockPoisoned)?;
    guard.insert(subscription_id.to_string(), "cancelled".to_string());
    Ok(())
}

/// @id: miyubilling_tool_billing_subscription_status
/// @role: accessor
/// @layer: tool
/// @human: Retourne le statut d'une souscription ; lecture gouvernée.
/// @do: billing_subscription_status_under_governance
pub fn status(ctx: &GovernedContext, subscription_id: &str) -> Result<String, MiyubillingError> {
    if !ctx.has_mandate() {
        return Err(MiyubillingError::NoMandate);
    }
    let guard = subscriptions().lock().map_err(|_| MiyubillingError::LockPoisoned)?;
    Ok(guard.get(subscription_id).cloned().unwrap_or_else(|| "active".to_string()))
}
