//! Tools MiyuComptaLedger — tool.compta.transaction.categorize, tool.compta.transaction.vat.resolve.
//! Écritures : catégorisation (règles fournies) ; résolution TVA.

use crate::context::GovernedContext;
use crate::errors::MiyucptaledgerError;

/// @id: miyucptaledger_tool_compta_transaction_categorize
/// @role: mutator
/// @layer: tool
/// @human: Catégorise une écriture ; exécution ; règles fournies.
/// @do: compta_transaction_categorize_under_governance
pub fn categorize(
    ctx: &GovernedContext,
    _transaction_ref: &str,
    _rules: &str,
) -> Result<String, MiyucptaledgerError> {
    if !ctx.has_mandate() {
        return Err(MiyucptaledgerError::NoMandate);
    }
    Ok("default".to_string())
}

/// @id: miyucptaledger_tool_compta_transaction_vat_resolve
/// @role: accessor
/// @layer: tool
/// @human: Rattache un taux TVA à une écriture ; lecture/résolution.
/// @do: compta_transaction_vat_resolve_under_governance
pub fn vat_resolve(
    ctx: &GovernedContext,
    _transaction_ref: &str,
) -> Result<String, MiyucptaledgerError> {
    if !ctx.has_mandate() {
        return Err(MiyucptaledgerError::NoMandate);
    }
    Ok("20".to_string())
}
