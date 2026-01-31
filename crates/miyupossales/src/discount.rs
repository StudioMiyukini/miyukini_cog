//! Tool MiyuPosSales — tool.pos.discount.apply.

use crate::context::GovernedContext;
use crate::errors::MiyupossalesError;

/// @id: miyupossales_tool_discount_apply
/// @role: mutator
/// @layer: tool
/// @human: Applique une remise (article ou reçu) ; règles = StrongFather ; WriteIntent KindMother.
/// @do: discount_apply_under_governance
/// tool.pos.discount.apply
pub fn apply(
    ctx: &GovernedContext,
    _sale_id: &str,
    _discount_type: &str,
    _value: f64,
    _target_line_id: Option<&str>,
) -> Result<(), MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Err(MiyupossalesError::Unimplemented)
}
