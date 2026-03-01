#![forbid(unsafe_code)]
#![allow(missing_docs)]
//! # Alicia Wakeword -- toolkit.alicia.wakeword
//!
//! Kit d'outils detection wake word "Hey Alicia" via rustpotter.
//! Alicia Home Assistante : detection du mot-cle sur flux audio continu.
//!
//! @id toolkit.alicia.wakeword
//! @role wake_word_detection
//! @layer 6
//! @do detect_hey_alicia_wake_word_on_audio_stream
//! @human Detection du mot-cle "Hey Alicia" sur flux audio continu via rustpotter

pub mod admin_cell;
pub mod context;
pub mod detector;
pub mod errors;
pub mod models;

pub use admin_cell::{
    alicia_wakeword_admin_cell, AliciaWakewordAdminCell, AliciaWakewordIdentification,
    AliciaWakewordIntegrity, AliciaWakewordTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use detector::{RustpotterDetector, ScoreMode, WakeWordConfig, WakeWordDetection, WakeWordDetector};
pub use errors::WakeWordError;
pub use models::{ModelInfo, ModelType};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_cell_creation() {
        let cell = alicia_wakeword_admin_cell("0.1.0", "test-fingerprint");
        assert_eq!(cell.identification.id, TOOLKIT_ID);
        assert_eq!(cell.identification.version, "0.1.0");
        assert_eq!(cell.identification.module_type, "toolkit");
        assert!(!cell.integrity.contracts.is_empty());
    }

    #[test]
    fn admin_cell_singleton() {
        let cell_a = admin_cell::init_admin_cell("0.1.0", "fp-1");
        let cell_b = admin_cell::get_admin_cell();
        assert!(cell_b.is_some());
        assert_eq!(cell_a.identification.id, cell_b.unwrap().identification.id);
    }

    #[test]
    fn governed_context_with_mandate() {
        let ctx = GovernedContext::new("mandate-001".to_string(), 1);
        assert!(ctx.has_mandate());
        assert_eq!(ctx.security_level, 1);
    }

    #[test]
    fn governed_context_without_mandate() {
        let ctx = GovernedContext::new(String::new(), 0);
        assert!(!ctx.has_mandate());
    }

    #[test]
    fn toolkit_id_format() {
        assert!(TOOLKIT_ID.starts_with("toolkit."));
        assert!(TOOLKIT_ID.contains("alicia"));
        assert!(TOOLKIT_ID.contains("wakeword"));
    }

    #[test]
    fn error_display() {
        let err = WakeWordError::NoMandate;
        let display = format!("{err}");
        assert!(display.contains("mandate"));

        let err = WakeWordError::ModelNotFound("test.rpw".to_string());
        let display = format!("{err}");
        assert!(display.contains("test.rpw"));

        let err = WakeWordError::Unimplemented;
        let display = format!("{err}");
        assert!(display.contains("implemented"));
    }

    #[test]
    fn model_type_validation() {
        assert_eq!(
            models::model_type_from_extension("test.rpw").unwrap(),
            ModelType::Reference
        );
        assert_eq!(
            models::model_type_from_extension("test.rpwm").unwrap(),
            ModelType::Trained
        );
        assert!(models::model_type_from_extension("test.wav").is_err());
    }
}
