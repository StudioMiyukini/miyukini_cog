#![allow(missing_docs)]
//! # MiyuHR — toolkit.hr.miyuhr
//!
//! Kit d'outils RH (time_clock, schedule). Persistance = KindMother (WriteIntent).
//! Alignement MIP : domaine `hr`, layer tool/toolkit.

// @id: toolkit.hr.miyuhr
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuHR ; expose les modules tools.
/// @do: expose_miyuhr_toolkit

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod schedule;
pub mod time_clock;

pub use admin_cell::{
    miyuhr_admin_cell, MiyuhrAdminCell, MiyuhrIdentification, MiyuhrIntegrity, MiyuhrTestManifest,
    TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyuhrError;
pub use schedule::{get as schedule_get, ScheduleResult, ShiftItem};
pub use time_clock::{clock_in, clock_out, list_clock_events, ClockEvent, ClockEventKind};
