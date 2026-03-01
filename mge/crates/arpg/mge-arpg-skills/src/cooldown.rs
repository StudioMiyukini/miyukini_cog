// @id: MGE-ARPG-Cooldown @do: cooldown @role: back-end @layer: 3 @human: miyuk
//! Cooldown timers for individual skills and aggregate tracking.

use std::collections::HashMap;

use crate::SkillId;

/// A single cooldown timer with a fixed duration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Cooldown {
    duration_ms: u32,
    remaining_ms: u32,
}

impl Cooldown {
    /// Create a new cooldown that starts in the ready state.
    pub fn new(duration_ms: u32) -> Self {
        Self {
            duration_ms,
            remaining_ms: 0,
        }
    }

    /// Return `true` if the cooldown has expired (remaining time is zero).
    pub fn is_ready(&self) -> bool {
        self.remaining_ms == 0
    }

    /// Start the cooldown, setting remaining time to the full duration.
    pub fn trigger(&mut self) {
        self.remaining_ms = self.duration_ms;
    }

    /// Advance the cooldown by `delta_ms` milliseconds, clamping at zero.
    pub fn tick(&mut self, delta_ms: u32) {
        self.remaining_ms = self.remaining_ms.saturating_sub(delta_ms);
    }

    /// Return the remaining cooldown time in milliseconds.
    pub fn remaining_ms(&self) -> u32 {
        self.remaining_ms
    }
}

/// Tracks cooldowns for multiple skills simultaneously.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SkillCooldownTracker {
    cooldowns: HashMap<SkillId, Cooldown>,
}

impl SkillCooldownTracker {
    /// Create an empty tracker.
    pub fn new() -> Self {
        Self {
            cooldowns: HashMap::new(),
        }
    }

    /// Register a cooldown for a skill. If the skill already has a cooldown
    /// entry, it is replaced.
    pub fn register(&mut self, id: SkillId, duration_ms: u32) {
        self.cooldowns.insert(id, Cooldown::new(duration_ms));
    }

    /// Check whether a skill's cooldown is ready. Returns `true` if the
    /// skill has no registered cooldown or if its cooldown has expired.
    pub fn is_ready(&self, id: &SkillId) -> bool {
        self.cooldowns
            .get(id)
            .is_none_or(Cooldown::is_ready)
    }

    /// Trigger the cooldown for a skill. Does nothing if the skill has no
    /// registered cooldown.
    pub fn trigger(&mut self, id: &SkillId) {
        if let Some(cd) = self.cooldowns.get_mut(id) {
            cd.trigger();
        }
    }

    /// Advance all registered cooldowns by `delta_ms` milliseconds.
    pub fn tick_all(&mut self, delta_ms: u32) {
        for cd in self.cooldowns.values_mut() {
            cd.tick(delta_ms);
        }
    }
}
