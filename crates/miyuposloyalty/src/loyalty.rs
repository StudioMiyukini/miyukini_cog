//! Tools MiyuPosLoyalty — tool.loyalty.points.grant, redeem, balance.get, card.resolve.

use crate::context::GovernedContext;
use crate::errors::MiyuposloyaltyError;

/// @id: miyuposloyalty_tool_points_grant
/// @role: mutator
/// @layer: tool
/// @human: Accorde des points (règles fournies ou gouvernées) ; WriteIntent KindMother.
/// @do: points_grant_under_governance
/// tool.loyalty.points.grant
pub fn points_grant(
    ctx: &GovernedContext,
    _customer_id: &str,
    _amount: i64,
    _reason: &str,
) -> Result<(), MiyuposloyaltyError> {
    if !ctx.has_mandate() {
        return Err(MiyuposloyaltyError::NoMandate);
    }
    Err(MiyuposloyaltyError::Unimplemented)
}

/// @id: miyuposloyalty_tool_points_redeem
/// @role: mutator
/// @layer: tool
/// @human: Déduit des points (échange) ; autorisation = StrongFather ; WriteIntent KindMother.
/// @do: points_redeem_under_governance
/// tool.loyalty.points.redeem
pub fn points_redeem(
    ctx: &GovernedContext,
    _customer_id: &str,
    _amount: i64,
    _reason: &str,
) -> Result<(), MiyuposloyaltyError> {
    if !ctx.has_mandate() {
        return Err(MiyuposloyaltyError::NoMandate);
    }
    Err(MiyuposloyaltyError::Unimplemented)
}

/// @id: miyuposloyalty_tool_balance_get
/// @role: mutator
/// @layer: tool
/// @human: Retourne le solde points d'un client.
/// @do: balance_get_under_governance
/// tool.loyalty.balance.get
pub fn balance_get(ctx: &GovernedContext, _customer_id: &str) -> Result<i64, MiyuposloyaltyError> {
    if !ctx.has_mandate() {
        return Err(MiyuposloyaltyError::NoMandate);
    }
    Err(MiyuposloyaltyError::Unimplemented)
}

/// @id: miyuposloyalty_tool_card_resolve
/// @role: mutator
/// @layer: tool
/// @human: Résout une carte fidélité (code/QR) → client + solde.
/// @do: card_resolve_under_governance
/// tool.loyalty.card.resolve
pub fn card_resolve(
    ctx: &GovernedContext,
    _card_code: &str,
) -> Result<CardResolveResult, MiyuposloyaltyError> {
    if !ctx.has_mandate() {
        return Err(MiyuposloyaltyError::NoMandate);
    }
    Err(MiyuposloyaltyError::Unimplemented)
}

/// Résultat résolution carte.
#[derive(Debug, Clone, Default)]
pub struct CardResolveResult {
    pub customer_id: String,
    pub balance: i64,
}
