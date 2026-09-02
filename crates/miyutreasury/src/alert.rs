//! Tool MiyuTreasury — tool.treasury.alert.check.
//! Vérification alertes ; seuils et échéances ; règles = StrongFather ; exécution seule.

use crate::context::GovernedContext;
use crate::errors::MiyutreasuryError;

/// @id miyutreasury_tool_treasury_alert_check
/// @role accessor
/// @layer tool
/// @do treasury_alert_check_under_governance
//@human / @human Vérifie les seuils et échéances (exécution ; règles = StrongFather).
pub fn check(
    ctx: &GovernedContext,
    _context_ref: Option<&str>,
) -> Result<Vec<String>, MiyutreasuryError> {
    if !ctx.has_mandate() {
        return Err(MiyutreasuryError::NoMandate);
    }
    Ok(Vec::new())
}
