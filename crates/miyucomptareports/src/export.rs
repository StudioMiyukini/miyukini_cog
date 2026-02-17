//! Tool MiyuComptaReports — tool.compta.export.ledger.
//! Export des écritures (format fourni) ; autorisation = StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyucomptareportsError;

/// @id: miyucomptareports_tool_ledger_export
/// @role: mutator
/// @layer: tool
/// @human: Export des écritures (format fourni) ; autorisation = StrongFather.
/// @do: ledger_export_under_governance
/// tool.compta.export.ledger
pub fn ledger(
    ctx: &GovernedContext,
    _format: &str,
    _period_start: &str,
    _period_end: &str,
) -> Result<Vec<u8>, MiyucomptareportsError> {
    if !ctx.has_mandate() {
        return Err(MiyucomptareportsError::NoMandate);
    }
    Ok(Vec::new())
}
