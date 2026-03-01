//! # Alicia Capture
//!
//! Toolkit de capture audio pour Alicia Home Assistante -- toolkit.alicia.capture.
//! Capture audio locale via cpal (WASAPI natif Windows), detection d'activite vocale (VAD),
//! buffer circulaire lock-free (ringbuf SPSC), enumeration multi-device.
//!
//! ## Modules exposes
//!
//! - `capture` : Demarrage/arret des streams audio cpal, callback de capture
//! - `devices` : Enumeration des peripheriques audio, filtrage par nom/ID stable
//! - `vad` : Voice Activity Detection base sur energie RMS avec debounce
//! - `admin_cell` : Cellule Admin Miyukini (identification, manifeste, integrite)
//! - `context` : Contexte de configuration runtime
//! - `errors` : Types d'erreur explicites
//!
//! ## Contraintes
//!
//! - 100% local, aucune dependance reseau
//! - Format cible : f32 mono 16kHz
//! - Buffer : 30s (480 000 samples) SPSC lock-free
//! - 4 pieces supportees (Chambre Theresa, Chambre parentale, Chambre Eleanore, Salon)

#![forbid(unsafe_code)]

// @id: toolkit.alicia.capture
// @role: infrastructure
// @layer: toolkit
// @human: Point d'entree du toolkit Alicia Capture ; expose capture audio, VAD, devices.
// @do: expose_alicia_capture_toolkit

pub mod admin_cell;
pub mod capture;
pub mod context;
pub mod devices;
pub mod errors;
pub mod vad;

pub use admin_cell::{
    get_admin_cell, init_admin_cell, alicia_capture_admin_cell,
    AliciaCaptureAdminCell, AliciaCaptureIdentification,
    AliciaCaptureIntegrity, AliciaCaptureTestManifest, TOOLKIT_ID,
};
pub use capture::{
    AudioCapture, CaptureConfig, CaptureHandle, CaptureStats, CaptureStatsSnapshot,
    OverflowPolicy,
};
pub use context::VoiceCaptureContext;
pub use devices::{AudioDeviceInfo, DeviceEnumerator};
pub use errors::VoiceCaptureError;
pub use vad::{VadConfig, VadState, VoiceActivityDetector};
