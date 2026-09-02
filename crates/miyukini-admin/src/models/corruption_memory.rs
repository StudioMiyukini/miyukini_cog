//! Mémoire de corruption passée (Auth and First-Boot §3.5.4.4).
//!
//! Référence : MiyukiniAdmin - Implementation Security and Controls §10.1

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// @id miyukiniadmin_corruption_memory
/// @role data
/// @layer operator
/// @do represent_corruption_memory
//@human / @human Mémoire de corruption après destruction et réinit (vierge avec mémoire).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorruptionMemory {
    /// @id miyukiniadmin_corruption_memory_reinitialised_at
    /// @role data
    /// @layer operator
    /// @do store_reinit_timestamp
    //@human / @human Date de réinitialisation.
    pub reinitialised_at: DateTime<Utc>,

    /// @id miyukiniadmin_corruption_memory_reason
    /// @role data
    /// @layer operator
    /// @do store_reason
    //@human / @human Raison (ex. recovery_automatic_failed).
    pub reason: String,

    /// @id miyukiniadmin_corruption_memory_previous_state
    /// @role data
    /// @layer operator
    /// @do store_previous_state
    //@human / @human État précédent (ex. compromised).
    #[serde(default)]
    pub previous_state: String,
}
