//! Tools MiyuDeclarations — tool.compta.declaration.tva.prepare, submit.
//! Soumission = StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyudeclarationsError;

/// @id: miyudeclarations_tool_tva_prepare
/// @role: mutator
/// @layer: tool
/// @human: Prépare la déclaration TVA.
/// @do: tva_prepare_under_governance
/// tool.compta.declaration.tva.prepare
pub fn prepare(
    ctx: &GovernedContext,
    _period: &str,
) -> Result<TvaPrepareResult, MiyudeclarationsError> {
    if !ctx.has_mandate() {
        return Err(MiyudeclarationsError::NoMandate);
    }
    Err(MiyudeclarationsError::Unimplemented)
}

/// @id: miyudeclarations_tool_tva_submit
/// @role: mutator
/// @layer: tool
/// @human: Soumet la déclaration TVA ; autorisation = StrongFather.
/// @do: tva_submit_under_governance
/// tool.compta.declaration.tva.submit
pub fn submit(
    ctx: &GovernedContext,
    _declaration_id: &str,
) -> Result<String, MiyudeclarationsError> {
    if !ctx.has_mandate() {
        return Err(MiyudeclarationsError::NoMandate);
    }
    Err(MiyudeclarationsError::Unimplemented)
}

/// Résultat préparation TVA.
#[derive(Debug, Clone, Default)]
pub struct TvaPrepareResult {
    pub declaration_id: String,
    pub payload: Vec<u8>,
}
