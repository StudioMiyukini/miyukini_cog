//! Tools MiyuBilling — tool.billing.subscription.* (create, update, cancel, status).
//! Souscriptions : WriteIntent KindMother ; décision StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyubillingError;

/// @id: miyubilling_tool_billing_subscription_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une souscription ; WriteIntent KindMother ; décision StrongFather.
/// @do: billing_subscription_create_under_governance
pub fn create(ctx: &GovernedContext, _payload: &str) -> Result<String, MiyubillingError> {
    if !ctx.has_mandate() {
        return Err(MiyubillingError::NoMandate);
    }
    Err(MiyubillingError::Unimplemented)
}

/// @id: miyubilling_tool_billing_subscription_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour une souscription ; WriteIntent KindMother.
/// @do: billing_subscription_update_under_governance
pub fn update(ctx: &GovernedContext, _subscription_id: &str, _payload: &str) -> Result<(), MiyubillingError> {
    if !ctx.has_mandate() {
        return Err(MiyubillingError::NoMandate);
    }
    Err(MiyubillingError::Unimplemented)
}

/// @id: miyubilling_tool_billing_subscription_cancel
/// @role: mutator
/// @layer: tool
/// @human: Annule ou résilie une souscription ; décision StrongFather ; WriteIntent KindMother.
/// @do: billing_subscription_cancel_under_governance
pub fn cancel(ctx: &GovernedContext, _subscription_id: &str) -> Result<(), MiyubillingError> {
    if !ctx.has_mandate() {
        return Err(MiyubillingError::NoMandate);
    }
    Err(MiyubillingError::Unimplemented)
}

/// @id: miyubilling_tool_billing_subscription_status
/// @role: accessor
/// @layer: tool
/// @human: Retourne le statut d'une souscription ; lecture gouvernée.
/// @do: billing_subscription_status_under_governance
pub fn status(ctx: &GovernedContext, _subscription_id: &str) -> Result<String, MiyubillingError> {
    if !ctx.has_mandate() {
        return Err(MiyubillingError::NoMandate);
    }
    Err(MiyubillingError::Unimplemented)
}
