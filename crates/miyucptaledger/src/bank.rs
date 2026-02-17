//! Tool MiyuComptaLedger — tool.compta.bank.sync.
//! Synchro bancaire ; déclenche ou enregistre (API/EBICS/agrégateur) ; WriteIntent KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyucptaledgerError;

/// @id: miyucptaledger_tool_compta_bank_sync
/// @role: mutator
/// @layer: tool
/// @human: Déclenche ou enregistre une synchronisation bancaire (API/EBICS/agrégateur).
/// @do: compta_bank_sync_under_governance
pub fn sync(ctx: &GovernedContext, _payload: &str) -> Result<String, MiyucptaledgerError> {
    if !ctx.has_mandate() {
        return Err(MiyucptaledgerError::NoMandate);
    }
    Ok("sync:ok".to_string())
}
