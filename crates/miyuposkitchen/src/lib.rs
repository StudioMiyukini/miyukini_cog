#![allow(missing_docs)]
//! # MiyuPosKitchen — toolkit.pos.kitchen
//!
//! Kit d'outils cuisine PoS (kitchen, service_type, ticket). Données fournies dans le flux.
//! Alignement MIP : domaine `pos`, layer tool/toolkit.

// @id: toolkit.pos.miyuposkitchen
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuPosKitchen ; expose les modules tools.
/// @do: expose_miyuposkitchen_toolkit

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod kitchen;
pub mod service_type;
pub mod ticket;

pub use admin_cell::{
    miyuposkitchen_admin_cell, MiyuposkitchenAdminCell, MiyuposkitchenIdentification,
    MiyuposkitchenIntegrity, MiyuposkitchenTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyuposkitchenError;
pub use kitchen::{order_push, order_update_status, print as kitchen_print};
pub use service_type::set as service_type_set;
pub use ticket::preset_assign as ticket_preset_assign;
