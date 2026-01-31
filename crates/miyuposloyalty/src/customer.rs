//! Tools MiyuPosLoyalty — tool.crm.customer.get, list, create, update, address.get, note.add, note.list.

use crate::context::GovernedContext;
use crate::errors::MiyuposloyaltyError;
use std::collections::HashMap;

/// @id: miyuposloyalty_tool_customer_get
/// @role: mutator
/// @layer: tool
/// @human: Retourne un client par identifiant.
/// @do: customer_get_under_governance
/// tool.crm.customer.get
pub fn get(ctx: &GovernedContext, _customer_id: &str) -> Result<CustomerItem, MiyuposloyaltyError> {
    if !ctx.has_mandate() {
        return Err(MiyuposloyaltyError::NoMandate);
    }
    Err(MiyuposloyaltyError::Unimplemented)
}

/// @id: miyuposloyalty_tool_customer_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les clients (filtres, recherche).
/// @do: customer_list_under_governance
/// tool.crm.customer.list
pub fn list(
    ctx: &GovernedContext,
    _filters: &CustomerFilters,
) -> Result<Vec<CustomerItem>, MiyuposloyaltyError> {
    if !ctx.has_mandate() {
        return Err(MiyuposloyaltyError::NoMandate);
    }
    Err(MiyuposloyaltyError::Unimplemented)
}

/// @id: miyuposloyalty_tool_customer_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un client à partir de données fournies ; WriteIntent KindMother.
/// @do: customer_create_under_governance
/// tool.crm.customer.create
pub fn create(
    ctx: &GovernedContext,
    _data: &HashMap<String, String>,
) -> Result<String, MiyuposloyaltyError> {
    if !ctx.has_mandate() {
        return Err(MiyuposloyaltyError::NoMandate);
    }
    Err(MiyuposloyaltyError::Unimplemented)
}

/// @id: miyuposloyalty_tool_customer_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour un client ; WriteIntent KindMother.
/// @do: customer_update_under_governance
/// tool.crm.customer.update
pub fn update(
    ctx: &GovernedContext,
    _customer_id: &str,
    _data: &HashMap<String, String>,
) -> Result<(), MiyuposloyaltyError> {
    if !ctx.has_mandate() {
        return Err(MiyuposloyaltyError::NoMandate);
    }
    Err(MiyuposloyaltyError::Unimplemented)
}

/// @id: miyuposloyalty_tool_customer_address_get
/// @role: mutator
/// @layer: tool
/// @human: Retourne l'adresse (livraison) du client.
/// @do: customer_address_get_under_governance
/// tool.crm.customer.address.get
pub fn address_get(
    ctx: &GovernedContext,
    _customer_id: &str,
) -> Result<AddressItem, MiyuposloyaltyError> {
    if !ctx.has_mandate() {
        return Err(MiyuposloyaltyError::NoMandate);
    }
    Err(MiyuposloyaltyError::Unimplemented)
}

/// @id: miyuposloyalty_tool_customer_note_add
/// @role: mutator
/// @layer: tool
/// @human: Ajoute une note à un client ; WriteIntent KindMother.
/// @do: customer_note_add_under_governance
/// tool.crm.customer.note.add
pub fn note_add(
    ctx: &GovernedContext,
    _customer_id: &str,
    _content: &str,
) -> Result<String, MiyuposloyaltyError> {
    if !ctx.has_mandate() {
        return Err(MiyuposloyaltyError::NoMandate);
    }
    Err(MiyuposloyaltyError::Unimplemented)
}

/// @id: miyuposloyalty_tool_customer_note_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les notes d'un client.
/// @do: customer_note_list_under_governance
/// tool.crm.customer.note.list
pub fn note_list(
    ctx: &GovernedContext,
    _customer_id: &str,
) -> Result<Vec<NoteItem>, MiyuposloyaltyError> {
    if !ctx.has_mandate() {
        return Err(MiyuposloyaltyError::NoMandate);
    }
    Err(MiyuposloyaltyError::Unimplemented)
}

/// Filtres client.
#[derive(Debug, Clone, Default)]
pub struct CustomerFilters {
    pub search: Option<String>,
    pub limit: Option<u32>,
}

/// Élément client.
#[derive(Debug, Clone)]
pub struct CustomerItem {
    pub id: String,
    pub name: String,
}

/// Adresse.
#[derive(Debug, Clone, Default)]
pub struct AddressItem {
    pub line1: String,
    pub city: String,
    pub postal_code: String,
}

/// Note.
#[derive(Debug, Clone)]
pub struct NoteItem {
    pub id: String,
    pub content: String,
}
