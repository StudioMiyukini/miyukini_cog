#![allow(missing_docs)]
//! # MiyuJobs — toolkit.jobs.miyujobs
//!
//! Kit d'outils de planification et file d'attente (schedule.at, schedule.cron, queue.enqueue, queue.process). Décision de planifier = StrongFather.
//! Alignement MIP : domaine `jobs`, layer tool/toolkit.

// @id: toolkit.jobs.miyujobs
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuJobs ; expose les modules tools.
/// @do: expose_miyujobs_toolkit

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod queue;
pub mod schedule;

pub use admin_cell::{
    miyujobs_admin_cell, MiyuJobsAdminCell, MiyuJobsIdentification, MiyuJobsIntegrity,
    MiyuJobsTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyuJobsError;
pub use queue::{
    enqueue as queue_enqueue, get_task as queue_get_task, process as queue_process, EnqueueOptions,
    ProcessResult, Task, TaskStatus,
};
pub use schedule::{schedule_at as jobs_schedule_at, schedule_cron as jobs_schedule_cron};
