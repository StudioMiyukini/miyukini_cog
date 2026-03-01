// @id: MGE-ARPG-Combat-Status @do: status-effects @role: back-end @layer: 3 @human: miyuk
//! Temporary status effects (buffs and debuffs) applied during combat.

use serde::{Deserialize, Serialize};

/// A temporary status effect that ticks down each game tick.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusEffect {
    /// Deals `damage_per_tick` each tick until expired.
    Poisoned {
        /// Damage dealt per tick.
        damage_per_tick: i32,
        /// Remaining ticks before the effect expires.
        ticks_remaining: u32,
    },
    /// Slows the target by `slow_factor` (e.g. `0.5` = 50% slower).
    Chilled {
        /// Movement speed multiplier reduction (0.0..1.0).
        slow_factor: f32,
    },
    /// Target cannot act for the remaining ticks.
    Frozen {
        /// Remaining ticks before the effect expires.
        ticks_remaining: u32,
    },
    /// Target cannot act for the remaining ticks (similar to frozen but
    /// from a different source).
    Stunned {
        /// Remaining ticks before the effect expires.
        ticks_remaining: u32,
    },
    /// A named curse with arbitrary semantics.
    Cursed {
        /// The name or identifier of the curse.
        name: String,
    },
}

impl StatusEffect {
    /// Advance the effect by one tick.
    ///
    /// Returns `true` if the effect expires **this** tick (i.e. its
    /// counter reached zero after decrementing).
    ///
    /// Effects without a tick counter (e.g. `Chilled`, `Cursed`) never
    /// expire from ticking alone and always return `false`.
    pub fn tick(&mut self) -> bool {
        match self {
            Self::Poisoned {
                ticks_remaining, ..
            }
            | Self::Frozen { ticks_remaining }
            | Self::Stunned { ticks_remaining } => {
                *ticks_remaining = ticks_remaining.saturating_sub(1);
                *ticks_remaining == 0
            }
            // Chilled and Cursed have no tick counter.
            Self::Chilled { .. } | Self::Cursed { .. } => false,
        }
    }

    /// Returns `true` if this effect has already expired.
    pub fn is_expired(&self) -> bool {
        match self {
            Self::Poisoned {
                ticks_remaining, ..
            }
            | Self::Frozen { ticks_remaining }
            | Self::Stunned { ticks_remaining } => *ticks_remaining == 0,
            // Permanent until explicitly removed.
            Self::Chilled { .. } | Self::Cursed { .. } => false,
        }
    }
}
