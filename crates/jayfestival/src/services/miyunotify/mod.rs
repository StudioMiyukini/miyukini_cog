//! Intégration Miyunotify (annonces, notifications).
//!
//! JayFestival déclenche les envois (annonces organisateur, notifications candidature, etc.).

pub mod adapter;

pub use adapter::{
    miyunotify_send_announcement, miyunotify_send_targeted, MiyunotifyAdapterError,
    MiyunotifyTargetChannel,
};
