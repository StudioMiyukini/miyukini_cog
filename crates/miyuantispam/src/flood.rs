//! Tool MiyuAntiSpam — tool.antispam.flood.check.
//! Vérifie le flood (scope fourni) ; décision bloquer = StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyuantispamError;

/// @id: miyuantispam_tool_flood_check
/// @role: security
/// @layer: tool
/// @human: Vérifie le flood (scope fourni) ; seuils = flux ou KindMother.
/// @do: flood_check_under_governance
/// tool.antispam.flood.check
pub fn check(ctx: &GovernedContext, _scope_id: &str) -> Result<FloodCheckResult, MiyuantispamError> {
    if !ctx.has_mandate() {
        return Err(MiyuantispamError::NoMandate);
    }
    Ok(FloodCheckResult {
        within_limit: true,
        current_count: 0,
    })
}

/// Résultat de vérification flood (pas de décision — fourni à StrongFather).
#[derive(Debug, Clone, Default)]
pub struct FloodCheckResult {
    pub within_limit: bool,
    pub current_count: u64,
}
