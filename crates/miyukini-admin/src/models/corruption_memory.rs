//! Mémoire de corruption passée (Auth and First-Boot §3.5.4.4).
//!
//! Référence : MiyukiniAdmin - Implementation Security and Controls §10.1

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// @id: miyukiniadmin_corruption_memory
/// @role: data
/// @layer: operator
/// @human: Mémoire de corruption après destruction et réinit (vierge avec mémoire).
/// @do: represent_corruption_memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorruptionMemory {
    /// @id: miyukiniadmin_corruption_memory_reinitialised_at
    /// @role: data
    /// @layer: operator
    /// @human: Date de réinitialisation.
    /// @do: store_reinit_timestamp
    pub reinitialised_at: DateTime<Utc>,

    /// @id: miyukiniadmin_corruption_memory_reason
    /// @role: data
    /// @layer: operator
    /// @human: Raison (ex. recovery_automatic_failed).
    /// @do: store_reason
    pub reason: String,

    /// @id: miyukiniadmin_corruption_memory_previous_state
    /// @role: data
    /// @layer: operator
    /// @human: État précédent (ex. compromised).
    /// @do: store_previous_state
    #[serde(default)]
    pub previous_state: String,
}
