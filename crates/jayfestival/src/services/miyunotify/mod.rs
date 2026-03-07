//! Intégration Miyunotify (annonces, notifications).
//!
//! JayFestival déclenche les envois (annonces organisateur, notifications candidature, etc.).
//!
//! @id: jayfestival_svc_miyunotify_mod @do: export_miyunotify_adapter
//! @role: api @layer: service
//! @human: Module intégration Miyunotify — annonces organisateur et notifications candidature exposants.

pub mod adapter;

pub use adapter::{
    miyunotify_send_announcement, miyunotify_send_targeted, MiyunotifyAdapterError,
    MiyunotifyTargetChannel,
};
