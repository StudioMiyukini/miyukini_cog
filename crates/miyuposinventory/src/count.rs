//! Tools MiyuPosInventory — tool.inventory.count.start, record, reconcile.

use crate::context::GovernedContext;
use crate::errors::MiyuposinventoryError;

/// @id: miyuposinventory_tool_count_start
/// @role: mutator
/// @layer: tool
/// @human: Démarre une session d'inventaire physique ; WriteIntent KindMother.
/// @do: count_start_under_governance
/// tool.inventory.count.start
pub fn start(ctx: &GovernedContext, _site_id: &str) -> Result<String, MiyuposinventoryError> {
    if !ctx.has_mandate() {
        return Err(MiyuposinventoryError::NoMandate);
    }
    Err(MiyuposinventoryError::Unimplemented)
}

/// @id: miyuposinventory_tool_count_record
/// @role: mutator
/// @layer: tool
/// @human: Enregistre un comptage (article, quantité) pour une session ; WriteIntent KindMother.
/// @do: count_record_under_governance
/// tool.inventory.count.record
pub fn record(
    ctx: &GovernedContext,
    _session_id: &str,
    _article_id: &str,
    _quantity: i64,
) -> Result<(), MiyuposinventoryError> {
    if !ctx.has_mandate() {
        return Err(MiyuposinventoryError::NoMandate);
    }
    Err(MiyuposinventoryError::Unimplemented)
}

/// @id: miyuposinventory_tool_count_reconcile
/// @role: mutator
/// @layer: tool
/// @human: Clôture un comptage et propose/applique les écarts ; décision = StrongFather.
/// @do: count_reconcile_under_governance
/// tool.inventory.count.reconcile
pub fn reconcile(
    ctx: &GovernedContext,
    _session_id: &str,
) -> Result<ReconcileResult, MiyuposinventoryError> {
    if !ctx.has_mandate() {
        return Err(MiyuposinventoryError::NoMandate);
    }
    Err(MiyuposinventoryError::Unimplemented)
}

/// Résultat réconciliation.
#[derive(Debug, Clone, Default)]
pub struct ReconcileResult {
    pub discrepancies: Vec<(String, i64, i64)>,
}
