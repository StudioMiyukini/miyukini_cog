#![allow(missing_docs)]
//! # MiyuPosPayment — toolkit.pos.payment
//!
//! Kit d'outils paiements PoS (cash, check, split, terminal). WriteIntent KindMother.
//! Alignement MIP : domaine `pos`, layer tool/toolkit.

/// @id: miyupospayment_toolkit_lib
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuPosPayment ; expose les modules tools.
/// @do: expose_miyupospayment_toolkit

pub mod admin_cell;
pub mod cash;
pub mod check;
pub mod context;
pub mod errors;
pub mod split;
pub mod terminal;

pub use admin_cell::{
    miyupospayment_admin_cell, MiyupospaymentAdminCell, MiyupospaymentIdentification,
    MiyupospaymentIntegrity, MiyupospaymentTestManifest, TOOLKIT_ID,
};
pub use cash::record as cash_record;
pub use check::record as check_record;
pub use context::GovernedContext;
pub use errors::MiyupospaymentError;
pub use split::split as payment_split;
pub use terminal::{authorize as terminal_authorize, capture as terminal_capture, TerminalAuthResult};
