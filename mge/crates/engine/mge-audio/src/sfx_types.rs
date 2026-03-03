// @id: MGE-Audio-SFX @do: sfx-bank @role: back-end @layer: 2 @human: miyuk
//! Typed SFX identifiers and asset bank for Sodomight.
//!
//! `SfxId` enumerates every distinct sound effect in the game.
//! `SfxBank` maps each `SfxId` to a filesystem path without touching
//! any audio backend — all structures are hardware-free and fully testable.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Typed identifier for every sound effect used in Sodomight.
///
/// Prefer this over raw `&str` keys to get compile-time exhaustiveness
/// checking when all SFX entries must be handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SfxId {
    /// Melee weapon impact (sword, axe, blunt…).
    HitMelee,
    /// Spell or projectile impact.
    HitSpell,
    /// Death/death-rattle of any entity.
    Death,
    /// Item pick-up clink.
    Pickup,
    /// Health or mana potion gulp.
    Potion,
    /// Player level-up fanfare.
    LevelUp,
}

/// Registry that maps `SfxId` values to filesystem asset paths.
///
/// `SfxBank` is intentionally hardware-free; actual sample loading is
/// delegated to the `kira` backend at runtime.
#[derive(Debug, Clone, Default)]
pub struct SfxBank {
    entries: HashMap<SfxId, PathBuf>,
}

impl SfxBank {
    /// Creates an empty bank.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers (or overwrites) the asset path for `id`.
    pub fn register(&mut self, id: SfxId, path: impl Into<PathBuf>) {
        self.entries.insert(id, path.into());
    }

    /// Returns the path associated with `id`, or `None` if not registered.
    #[must_use]
    pub fn get(&self, id: SfxId) -> Option<&Path> {
        self.entries.get(&id).map(PathBuf::as_path)
    }

    /// Number of registered entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` when the bank contains no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
