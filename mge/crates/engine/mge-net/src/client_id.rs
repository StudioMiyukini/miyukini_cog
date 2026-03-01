// @id: MGE-Net-ClientId @do: identity @role: back-end @layer: 2 @human: miyuk
//! Unique client identifier wrapping a UUID v4.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier for a connected client.
///
/// Wraps a UUID v4. Two distinct calls to [`ClientId::new`] are guaranteed
/// to produce different values (with overwhelming probability).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClientId(pub uuid::Uuid);

impl ClientId {
    /// Generate a fresh random client identifier.
    #[must_use]
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for ClientId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ClientId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Client({})", self.0)
    }
}
