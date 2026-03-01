//! # miyualicia-devices
//!
//! Registre de dispositifs domotiques pour Alicia Home Assistante.
//! Definit les types de domaine, le registre in-memory thread-safe,
//! les commandes generiques et les types d'erreur.
//!
//! ## Loi d'Autonomie
//!
//! Ce crate est 100 % local : aucune dependance reseau, aucun cloud.
//! Il est utilisable sans MQTT ni HTTP actifs.

#![forbid(unsafe_code)]

// @id: toolkit.alicia.devices
// @role: device_registry
// @layer: 6
// @human: Registre central des dispositifs domotiques Alicia ; types, etat, commandes.
// @do: expose_device_registry_and_types

pub mod admin_cell;
pub mod command;
pub mod errors;
pub mod registry;
pub mod types;

// Re-exports de l'API publique
pub use command::{CommandSource, DeviceCommand};
pub use errors::DeviceError;
pub use registry::DeviceRegistry;
pub use types::{
    Device, DeviceCapabilities, DeviceConfig, DeviceProtocol, DeviceState, DeviceType,
};
