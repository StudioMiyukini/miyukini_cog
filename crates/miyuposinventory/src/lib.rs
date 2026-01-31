//! # MiyuPosInventory — toolkit.pos.inventory
//!
//! Kit d'outils inventaire PoS (stock, import, alert, purchase_order, transfer, count, production, label, history, valuation). WriteIntent KindMother.
//! Alignement MIP : domaine `pos`, layer tool/toolkit.

/// @id: miyuposinventory_toolkit_lib
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuPosInventory ; expose les modules tools.
/// @do: expose_miyuposinventory_toolkit

pub mod admin_cell;
pub mod alert;
pub mod context;
pub mod count;
pub mod errors;
pub mod history;
pub mod import;
pub mod label;
pub mod production;
pub mod purchase_order;
pub mod stock;
pub mod transfer;
pub mod valuation;

pub use admin_cell::{
    miyuposinventory_admin_cell, MiyuposinventoryAdminCell, MiyuposinventoryIdentification,
    MiyuposinventoryIntegrity, MiyuposinventoryTestManifest, TOOLKIT_ID,
};
pub use alert::{low_evaluate as alert_low_evaluate, AlertItem};
pub use context::GovernedContext;
pub use count::{record as count_record, reconcile as count_reconcile, start as count_start, ReconcileResult};
pub use errors::MiyuposinventoryError;
pub use history::{list as history_list, MovementItem};
pub use import::{items as import_items, ImportResult};
pub use label::print as label_print;
pub use production::record as production_record;
pub use purchase_order::{create as purchase_order_create, track as purchase_order_track, update as purchase_order_update, PurchaseOrderStatus};
pub use stock::{adjust as stock_adjust, get as stock_get, StockResult};
pub use transfer::{create as transfer_create, execute as transfer_execute, list as transfer_list, TransferFilters, TransferItem};
pub use valuation::report as valuation_report;
