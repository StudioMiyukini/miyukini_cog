//! Tools MiyuComptaLedger — tool.company.structure.*, tool.company.siret.resolve.
//! Structure juridique : résolution ; enregistrement (WriteIntent) ; SIRET/INSEE (lecture).

use crate::context::GovernedContext;
use crate::errors::MiyucptaledgerError;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyucptaledger_tool_company_structure_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout la structure juridique courante (micro, EURL, etc.) pour le contexte ; lecture.
/// @do: company_structure_resolve_under_governance
pub fn structure_resolve(
    ctx: &GovernedContext,
    context_ref: Option<&str>,
) -> Result<String, MiyucptaledgerError> {
    if !ctx.has_mandate() {
        return Err(MiyucptaledgerError::NoMandate);
    }
    let key = context_ref.unwrap_or("default");
    let guard = store::structures()
        .lock()
        .map_err(|e| MiyucptaledgerError::InvalidInput(e.to_string()))?;
    Ok(guard
        .get(key)
        .cloned()
        .unwrap_or_else(|| "micro".to_string()))
}

/// @id: miyucptaledger_tool_company_structure_register
/// @role: mutator
/// @layer: tool
/// @human: Enregistre une structure ; WriteIntent KindMother.
/// @do: company_structure_register_under_governance
pub fn structure_register(
    ctx: &GovernedContext,
    payload: &str,
) -> Result<String, MiyucptaledgerError> {
    if !ctx.has_mandate() {
        return Err(MiyucptaledgerError::NoMandate);
    }
    let id = format!("struct:{}", UuidIdGenerator.generate());
    let mut guard = store::structures()
        .lock()
        .map_err(|e| MiyucptaledgerError::InvalidInput(e.to_string()))?;
    guard.insert(id.clone(), payload.to_string());
    Ok(id)
}

/// @id: miyucptaledger_tool_company_siret_resolve
/// @role: accessor
/// @layer: tool
/// @human: Récupère les informations depuis SIRET/INSEE ; lecture seule.
/// @do: company_siret_resolve_under_governance
pub fn siret_resolve(ctx: &GovernedContext, siret: &str) -> Result<String, MiyucptaledgerError> {
    if !ctx.has_mandate() {
        return Err(MiyucptaledgerError::NoMandate);
    }
    let guard = store::siret_cache()
        .lock()
        .map_err(|e| MiyucptaledgerError::InvalidInput(e.to_string()))?;
    Ok(guard
        .get(siret)
        .cloned()
        .unwrap_or_else(|| format!("{{\"siret\":\"{siret}\"}}")))
}
