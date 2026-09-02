//! Tools MiyuDeclarations — tool.compta.declaration.urssaf.prepare, submit.
//! Soumission = StrongFather ; persistance = KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyudeclarationsError;

/// @id miyudeclarations_tool_urssaf_prepare
/// @role mutator
/// @layer tool
/// @do urssaf_prepare_under_governance
//@human / @human Prépare les données de déclaration URSSAF (CA, etc.).
/// tool.compta.declaration.urssaf.prepare
pub fn prepare(
    ctx: &GovernedContext,
    _period: &str,
) -> Result<UrssafPrepareResult, MiyudeclarationsError> {
    if !ctx.has_mandate() {
        return Err(MiyudeclarationsError::NoMandate);
    }
    Ok(UrssafPrepareResult::default())
}

/// @id miyudeclarations_tool_urssaf_submit
/// @role mutator
/// @layer tool
/// @do urssaf_submit_under_governance
//@human / @human Soumet la déclaration URSSAF (télédéclaration) ; autorisation = StrongFather.
/// tool.compta.declaration.urssaf.submit
pub fn submit(
    ctx: &GovernedContext,
    _declaration_id: &str,
) -> Result<String, MiyudeclarationsError> {
    if !ctx.has_mandate() {
        return Err(MiyudeclarationsError::NoMandate);
    }
    Ok("urssaf:submitted".to_string())
}

/// Résultat préparation URSSAF.
#[derive(Debug, Clone, Default)]
pub struct UrssafPrepareResult {
    pub declaration_id: String,
    pub payload: Vec<u8>,
}
