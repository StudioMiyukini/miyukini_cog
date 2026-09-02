//! Tools MiyuPosInventory — tool.inventory.stock.get, adjust.

use crate::context::GovernedContext;
use crate::errors::MiyuposinventoryError;

/// @id miyuposinventory_tool_stock_get
/// @role mutator
/// @layer tool
/// @do stock_get_under_governance
//@human / @human Retourne le stock (et composants si applicable) pour un article/site.
/// tool.inventory.stock.get
pub fn get(
    ctx: &GovernedContext,
    _article_id: &str,
    _site_id: &str,
) -> Result<StockResult, MiyuposinventoryError> {
    if !ctx.has_mandate() {
        return Err(MiyuposinventoryError::NoMandate);
    }
    Ok(StockResult::default())
}

/// @id miyuposinventory_tool_stock_adjust
/// @role mutator
/// @layer tool
/// @do stock_adjust_under_governance
//@human / @human Ajuste le stock (réception, casse, perte) ; décision = StrongFather ; WriteIntent KindMother.
/// tool.inventory.stock.adjust
pub fn adjust(
    ctx: &GovernedContext,
    _article_id: &str,
    _site_id: &str,
    _delta: i64,
    _reason: &str,
) -> Result<(), MiyuposinventoryError> {
    if !ctx.has_mandate() {
        return Err(MiyuposinventoryError::NoMandate);
    }
    Ok(())
}

/// Résultat stock.
#[derive(Debug, Clone, Default)]
pub struct StockResult {
    pub quantity: i64,
    pub components: Vec<(String, i64)>,
}
