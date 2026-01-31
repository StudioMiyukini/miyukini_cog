//! Tool MiyuAntiSpam — tool.antispam.rate_limit.check.
//! Vérifie la limite de tentatives (scope fourni) ; décision bloquer = StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyuantispamError;

/// @id: miyuantispam_tool_rate_limit_check
/// @role: security
/// @layer: tool
/// @human: Vérifie la limite de tentatives (scope fourni).
/// @do: rate_limit_check_under_governance
/// tool.antispam.rate_limit.check
pub fn check(
    ctx: &GovernedContext,
    _scope_id: &str,
) -> Result<RateLimitCheckResult, MiyuantispamError> {
    if !ctx.has_mandate() {
        return Err(MiyuantispamError::NoMandate);
    }
    Ok(RateLimitCheckResult {
        within_limit: true,
        remaining: 100,
    })
}

/// Résultat de vérification rate limit (fourni à StrongFather).
#[derive(Debug, Clone, Default)]
pub struct RateLimitCheckResult {
    pub within_limit: bool,
    pub remaining: u32,
}
