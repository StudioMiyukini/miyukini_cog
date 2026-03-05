#![allow(missing_docs)]
//! # MiyuPosLoyalty — toolkit.pos.loyalty
//!
//! Kit d'outils CRM et fidélité PoS (customer, loyalty). WriteIntent KindMother.
//! Alignement MIP : domaine `pos`, layer tool/toolkit.

// @id: toolkit.pos.miyuposloyalty
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuPosLoyalty ; expose les modules tools.
/// @do: expose_miyuposloyalty_toolkit
pub mod admin_cell;
pub mod context;
pub mod customer;
pub mod errors;
pub mod loyalty;

pub use admin_cell::{
    miyuposloyalty_admin_cell, MiyuposloyaltyAdminCell, MiyuposloyaltyIdentification,
    MiyuposloyaltyIntegrity, MiyuposloyaltyTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use customer::{
    address_get as customer_address_get, create as customer_create, get as customer_get,
    list as customer_list, note_add as customer_note_add, note_list as customer_note_list,
    update as customer_update, AddressItem, CustomerFilters, CustomerItem, NoteItem,
};
pub use errors::MiyuposloyaltyError;
pub use loyalty::{
    balance_get as loyalty_balance_get, card_resolve as loyalty_card_resolve,
    points_grant as loyalty_points_grant, points_redeem as loyalty_points_redeem,
    CardResolveResult,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("m".into(), 0)
    }

    #[test]
    fn stub_tools_with_mandate() {
        let c = ctx();
        assert_eq!(loyalty_balance_get(&c, "cust").unwrap(), 0);
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(loyalty_balance_get(&c, "").is_err());
    }
}
