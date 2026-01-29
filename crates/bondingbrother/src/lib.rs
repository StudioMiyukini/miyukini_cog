//! # BondingBrother
//!
//! Stratégie de liaison gouvernée du Miyukini Core System.
//!
//! BondingBrother médiatise, traduit, et filtre les interactions entre les Opérateurs
//! et l'écosystème autoritaire sans jamais décider.

pub mod connection;
pub mod sync;
pub mod translation;

pub use connection::{Connection, ConnectionManager};
pub use sync::{SyncManager, SyncStrategy};
pub use translation::{Translation, Translator};
