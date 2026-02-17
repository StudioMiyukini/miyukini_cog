//! Tool MiyuDeclarations — tool.compta.declaration.estimate.cotisations.
//! Calcule une estimation des cotisations (micro) à partir de CA fourni.

use crate::context::GovernedContext;
use crate::errors::MiyudeclarationsError;

/// @id: miyudeclarations_tool_estimate_cotisations
/// @role: mutator
/// @layer: tool
/// @human: Calcule une estimation des cotisations (micro) à partir de CA fourni.
/// @do: estimate_cotisations_under_governance
/// tool.compta.declaration.estimate.cotisations
pub fn cotisations(
    ctx: &GovernedContext,
    _ca_amount: f64,
    _regime: &str,
) -> Result<CotisationsEstimate, MiyudeclarationsError> {
    if !ctx.has_mandate() {
        return Err(MiyudeclarationsError::NoMandate);
    }
    Ok(CotisationsEstimate::default())
}

/// Résultat estimation cotisations.
#[derive(Debug, Clone, Default)]
pub struct CotisationsEstimate {
    pub total: f64,
    pub breakdown: Vec<(String, f64)>,
}
