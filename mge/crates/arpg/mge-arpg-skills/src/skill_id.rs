// @id: MGE-ARPG-SkillId @do: identifier @role: back-end @layer: 3 @human: miyuk
//! Newtype wrapper for skill identifiers.

/// Unique identifier for a skill definition.
///
/// Wraps a `String` to provide type safety and prevent accidental
/// misuse of raw strings as skill identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SkillId(pub String);

impl SkillId {
    /// Create a new `SkillId` from anything that converts into a `String`.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Return the inner string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SkillId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
