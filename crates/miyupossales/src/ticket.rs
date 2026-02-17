//! Tools MiyuPosSales — tool.pos.ticket.open, save, close, list.

use crate::context::GovernedContext;
use crate::errors::MiyupossalesError;

/// @id: miyupossales_tool_ticket_open
/// @role: mutator
/// @layer: tool
/// @human: Ouvre un ticket (ordre) pour paiement différé ; WriteIntent KindMother.
/// @do: ticket_open_under_governance
/// tool.pos.ticket.open
pub fn open(ctx: &GovernedContext, _store_id: &str) -> Result<String, MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Ok("ticket:stub".to_string())
}

/// @id: miyupossales_tool_ticket_save
/// @role: mutator
/// @layer: tool
/// @human: Sauvegarde un ticket sans le clôturer ; WriteIntent KindMother.
/// @do: ticket_save_under_governance
/// tool.pos.ticket.save
pub fn save(ctx: &GovernedContext, _ticket_id: &str) -> Result<(), MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Ok(())
}

/// @id: miyupossales_tool_ticket_close
/// @role: mutator
/// @layer: tool
/// @human: Clôture un ticket (après paiement ou annulation) ; WriteIntent KindMother.
/// @do: ticket_close_under_governance
/// tool.pos.ticket.close
pub fn close(ctx: &GovernedContext, _ticket_id: &str) -> Result<(), MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Ok(())
}

/// @id: miyupossales_tool_ticket_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les tickets ouverts (filtres fournis).
/// @do: ticket_list_under_governance
/// tool.pos.ticket.list
pub fn list(
    ctx: &GovernedContext,
    _store_id: &str,
    _filters: &TicketFilters,
) -> Result<Vec<TicketItem>, MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Ok(Vec::new())
}

/// Filtres ticket.
#[derive(Debug, Clone, Default)]
pub struct TicketFilters {
    pub status: Option<String>,
}

/// Élément ticket.
#[derive(Debug, Clone)]
pub struct TicketItem {
    pub id: String,
    pub store_id: String,
    pub status: String,
}
