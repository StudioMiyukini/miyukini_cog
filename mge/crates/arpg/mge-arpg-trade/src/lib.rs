// @id: MGE-ARPG-Trade @do: trade-system @role: back-end @layer: 3 @human: miyuk
//! ARPG trade system: atomic P2P trade sessions, NPC vendors, gold wallet.
#![deny(unsafe_code)]

pub mod error;
pub mod trade_session;
pub mod vendor;
pub mod wallet;

pub use error::TradeError;
pub use trade_session::{PlayerInventory, TradeOffer, TradeSession, TradeState};
pub use vendor::{act1_vendor, NpcVendor, VendorItem};
pub use wallet::{Wallet, WalletError};
