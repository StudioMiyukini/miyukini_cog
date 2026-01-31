//! Tools MiyuComptaLedger — tool.company.structure.*, tool.company.siret.resolve.
//! Structure juridique : résolution ; enregistrement (WriteIntent) ; SIRET/INSEE (lecture).

use crate::context::GovernedContext;
use crate::errors::MiyucptaledgerError;

/// @id: miyucptaledger_tool_company_structure_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout la structure juridique courante (micro, EURL, etc.) pour le contexte ; lecture.
/// @do: company_structure_resolve_under_governance
pub fn structure_resolve(ctx: &GovernedContext, _context_ref: Option<&str>) -> Result<String, MiyucptaledgerError> {
    if !ctx.has_mandate() {
        return Err(MiyucptaledgerError::NoMandate);
    }
    Err(MiyucptaledgerError::Unimplemented)
}

/// @id: miyucptaledger_tool_company_structure_register
/// @role: mutator
/// @layer: tool
/// @human: Enregistre une structure ; WriteIntent KindMother.
/// @do: company_structure_register_under_governance
pub fn structure_register(ctx: &GovernedContext, _payload: &str) -> Result<String, MiyucptaledgerError> {
    if !ctx.has_mandate() {
        return Err(MiyucptaledgerError::NoMandate);
    }
    Err(MiyucptaledgerError::Unimplemented)
}

/// @id: miyucptaledger_tool_company_siret_resolve
/// @role: accessor
/// @layer: tool
/// @human: Récupère les informations depuis SIRET/INSEE ; lecture seule.
/// @do: company_siret_resolve_under_governance
pub fn siret_resolve(ctx: &GovernedContext, _siret: &str) -> Result<String, MiyucptaledgerError> {
    if !ctx.has_mandate() {
        return Err(MiyucptaledgerError::NoMandate);
    }
    Err(MiyucptaledgerError::Unimplemented)
}
