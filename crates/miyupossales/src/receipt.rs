//! Tools MiyuPosSales — tool.pos.receipt.render, print, send, list.

use crate::context::GovernedContext;
use crate::errors::MiyupossalesError;

/// @id: miyupossales_tool_receipt_render
/// @role: mutator
/// @layer: tool
/// @human: Produit le contenu du reçu à partir des données de vente.
/// @do: receipt_render_under_governance
/// tool.pos.receipt.render
pub fn render(ctx: &GovernedContext, _sale_id: &str) -> Result<Vec<u8>, MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Err(MiyupossalesError::Unimplemented)
}

/// @id: miyupossales_tool_receipt_print
/// @role: mutator
/// @layer: tool
/// @human: Envoie le reçu à l'imprimante (données fournies).
/// @do: receipt_print_under_governance
/// tool.pos.receipt.print
pub fn print(ctx: &GovernedContext, _sale_id: &str) -> Result<(), MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Err(MiyupossalesError::Unimplemented)
}

/// @id: miyupossales_tool_receipt_send
/// @role: mutator
/// @layer: tool
/// @human: Envoie le reçu par email (données fournies).
/// @do: receipt_send_under_governance
/// tool.pos.receipt.send
pub fn send(
    ctx: &GovernedContext,
    _sale_id: &str,
    _email: &str,
) -> Result<(), MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Err(MiyupossalesError::Unimplemented)
}

/// @id: miyupossales_tool_receipt_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les reçus (filtres fournis).
/// @do: receipt_list_under_governance
/// tool.pos.receipt.list
pub fn list(
    ctx: &GovernedContext,
    _filters: &ReceiptFilters,
) -> Result<Vec<ReceiptItem>, MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Err(MiyupossalesError::Unimplemented)
}

/// Filtres reçu.
#[derive(Debug, Clone, Default)]
pub struct ReceiptFilters {
    pub sale_id: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

/// Élément reçu.
#[derive(Debug, Clone)]
pub struct ReceiptItem {
    pub id: String,
    pub sale_id: String,
}
