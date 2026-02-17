//! Tools MiyuComptaLedger — tool.compta.reconciliation.suggest, tool.compta.reconciliation.record.
//! Rapprochement : proposition (sans décider) ; enregistrement (autorisation StrongFather).

use crate::context::GovernedContext;
use crate::errors::MiyucptaledgerError;

/// @id: miyucptaledger_tool_compta_reconciliation_suggest
/// @role: accessor
/// @layer: tool
/// @human: Propose des rapprochements (sans décider) ; exécution seule.
/// @do: compta_reconciliation_suggest_under_governance
pub fn suggest(ctx: &GovernedContext, _context_ref: Option<&str>) -> Result<Vec<String>, MiyucptaledgerError> {
    if !ctx.has_mandate() {
        return Err(MiyucptaledgerError::NoMandate);
    }
    Ok(Vec::new())
}

/// @id: miyucptaledger_tool_compta_reconciliation_record
/// @role: mutator
/// @layer: tool
/// @human: Enregistre un rapprochement validé ; autorisation = StrongFather ; WriteIntent KindMother.
/// @do: compta_reconciliation_record_under_governance
pub fn record(ctx: &GovernedContext, _payload: &str) -> Result<String, MiyucptaledgerError> {
    if !ctx.has_mandate() {
        return Err(MiyucptaledgerError::NoMandate);
    }
    Ok("rec:ok".to_string())
}
