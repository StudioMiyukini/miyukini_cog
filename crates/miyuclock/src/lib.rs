#![allow(missing_docs)]
//! # MiyuClock — toolkit.kernel.miyuclock
//!
//! Kit d'outils de mesure du temps (instant présent, delta entre instants).
//! Tools : tool.time.now, tool.time.delta. Conformité LOI-4 (pas de temps global).

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod time;

pub use admin_cell::{
    miyuclock_admin_cell, MiyuclockAdminCell, MiyuclockIdentification, MiyuclockIntegrity,
    MiyuclockTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyuclockError;
pub use time::{delta as time_delta, now as time_now, now_with_clock as time_now_with_clock};
