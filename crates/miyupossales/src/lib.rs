#![allow(missing_docs)]
//! # MiyuPosSales — toolkit.pos.sales
//!
//! Kit d'outils caisse et ventes PoS (sale, ticket, discount, refund, receipt, item, cash_register, cash_movement, barcode, store, display, order). WriteIntent KindMother.
//! Alignement MIP : domaine `pos`, layer tool/toolkit.

// @id: toolkit.pos.miyupossales
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuPosSales ; expose les modules tools.
/// @do: expose_miyupossales_toolkit

pub mod admin_cell;
pub mod barcode;
pub mod cash_movement;
pub mod cash_register;
pub mod context;
pub mod discount;
pub mod display;
pub mod errors;
pub mod item;
pub mod order;
pub mod receipt;
pub mod refund;
pub mod sale;
pub mod store;
pub mod ticket;

pub use admin_cell::{
    miyupossales_admin_cell, MiyupossalesAdminCell, MiyupossalesIdentification,
    MiyupossalesIntegrity, MiyupossalesTestManifest, TOOLKIT_ID,
};
pub use barcode::{parse as barcode_parse, BarcodeParseResult};
pub use cash_movement::record as cash_movement_record;
pub use cash_register::{close as cash_register_close, open as cash_register_open};
pub use context::GovernedContext;
pub use discount::apply as discount_apply;
pub use display::push as display_push;
pub use errors::MiyupossalesError;
pub use item::{modifier_apply, variant_resolve, VariantResult};
pub use order::service_type_set as order_service_type_set;
pub use receipt::{list as receipt_list, print as receipt_print, render as receipt_render, send as receipt_send, ReceiptFilters, ReceiptItem};
pub use refund::record as refund_record;
pub use sale::{add_item as sale_add_item, create as sale_create, remove_item as sale_remove_item};
pub use store::{store_resolve, StoreResult};
pub use ticket::{close as ticket_close, list as ticket_list, open as ticket_open, save as ticket_save, TicketFilters, TicketItem};

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("m".into(), 0)
    }

    #[test]
    fn stub_tools_with_mandate() {
        let c = ctx();
        assert!(sale_create(&c, "s1").unwrap().starts_with("sale:"));
        assert!(store_resolve(&c).unwrap().store_id.is_empty());
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(sale_create(&c, "").is_err());
    }
}