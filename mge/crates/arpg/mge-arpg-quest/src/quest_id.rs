// @id: MGE-ARPG-Quest-Id @do: quest-identity @role: back-end @layer: 3 @human: miyuk
//! Quest identifier newtype.

use std::fmt;

/// Strongly-typed quest identifier wrapping a string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct QuestId(pub String);

impl QuestId {
    /// Create a new quest identifier from any string-like value.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Return the inner string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for QuestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
