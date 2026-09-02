//! Tools MiyuDeclarations — tool.compta.declaration.tva.prepare, submit.
//! Soumission = StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyudeclarationsError;

/// @id miyudeclarations_tool_tva_prepare
/// @role mutator
/// @layer tool
/// @do tva_prepare_under_governance
//@human / @human Prépare la déclaration TVA.
/// tool.compta.declaration.tva.prepare
pub fn prepare(
    ctx: &GovernedContext,
    _period: &str,
) -> Result<TvaPrepareResult, MiyudeclarationsError> {
    if !ctx.has_mandate() {
        return Err(MiyudeclarationsError::NoMandate);
    }
    Ok(TvaPrepareResult::default())
}

/// @id miyudeclarations_tool_tva_submit
/// @role mutator
/// @layer tool
/// @do tva_submit_under_governance
//@human / @human Soumet la déclaration TVA ; autorisation = StrongFather.
/// tool.compta.declaration.tva.submit
pub fn submit(
    ctx: &GovernedContext,
    _declaration_id: &str,
) -> Result<String, MiyudeclarationsError> {
    if !ctx.has_mandate() {
        return Err(MiyudeclarationsError::NoMandate);
    }
    Ok("tva:submitted".to_string())
}

/// Résultat préparation TVA.
#[derive(Debug, Clone, Default)]
pub struct TvaPrepareResult {
    pub declaration_id: String,
    pub payload: Vec<u8>,
}
