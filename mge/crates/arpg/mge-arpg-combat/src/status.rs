// @id: MGE-ARPG-Status @do: status-effects @role: back-end @layer: 3 @human: miyuk
//! D2-style status effects applied during combat.
//!
//! Provides [`StatusType`], [`StatusEffect`], and [`StatusTracker`] for
//! tracking active debuffs on a combat entity.  All timing is expressed in
//! milliseconds to decouple the tracker from the engine tick rate.

use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------ //
// StatusType
// ------------------------------------------------------------------ //

/// Identifies the category of a status effect.
///
/// Each variant corresponds to a distinct D2-inspired debuff.  Only one
/// effect of a given type may be active at a time; applying a duplicate
/// refreshes the duration instead of stacking (see [`StatusTracker::apply`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusType {
    /// Deals damage over time (potency = damage per second).
    Poison,
    /// Reduces movement and attack speed (potency = slow percentage 0–100).
    Cold,
    /// Generic curse placeholder (potency = resist reduction in points).
    Cursed,
    /// Amplify Damage — reduces all resistances (potency = reduction in points).
    Amplify,
    /// Decrepify — reduces physical resist and slows (potency = reduction in points).
    Decrepify,
}

// ------------------------------------------------------------------ //
// StatusEffect
// ------------------------------------------------------------------ //

/// A single active status effect on an entity.
///
/// - `kind` — which debuff category this is.
/// - `remaining_ms` — milliseconds remaining before the effect expires.
/// - `potency` — effect-specific magnitude:
///   - [`StatusType::Poison`] → damage per second (DPS).
///   - [`StatusType::Cold`] → slow percentage (e.g. `30.0` = 30 % slower).
///   - [`StatusType::Cursed`] / [`StatusType::Amplify`] / [`StatusType::Decrepify`]
///     → resistance reduction in points.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffect {
    /// The category of this effect.
    pub kind: StatusType,
    /// Time remaining in milliseconds.
    pub remaining_ms: u32,
    /// Effect-specific magnitude (see struct documentation).
    pub potency: f32,
}

// ------------------------------------------------------------------ //
// StatusTracker
// ------------------------------------------------------------------ //

/// Manages the set of active [`StatusEffect`]s on a single entity.
///
/// Invariant: at most one effect of each [`StatusType`] is held at any time.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatusTracker {
    effects: Vec<StatusEffect>,
}

impl StatusTracker {
    /// Create a new empty tracker.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Apply a status effect.
    ///
    /// If an effect of the same [`StatusType`] is already active, the existing
    /// entry is refreshed with the new `remaining_ms` and `potency` (no
    /// stacking).  Otherwise the effect is appended.
    pub fn apply(&mut self, effect: StatusEffect) {
        if let Some(existing) = self.effects.iter_mut().find(|e| e.kind == effect.kind) {
            existing.remaining_ms = effect.remaining_ms;
            existing.potency = effect.potency;
        } else {
            self.effects.push(effect);
        }
    }

    /// Advance all active effects by `delta_ms` milliseconds, removing any
    /// whose `remaining_ms` reaches zero.
    pub fn tick(&mut self, delta_ms: u32) {
        for effect in &mut self.effects {
            effect.remaining_ms = effect.remaining_ms.saturating_sub(delta_ms);
        }
        self.effects.retain(|e| e.remaining_ms > 0);
    }

    /// Returns `true` if an effect of `kind` is currently active.
    #[must_use]
    pub fn is_affected(&self, kind: StatusType) -> bool {
        self.effects.iter().any(|e| e.kind == kind)
    }

    /// Returns the combined speed multiplier from all active [`StatusType::Cold`]
    /// effects.
    ///
    /// A 30 % slow means `potency = 30.0`; the multiplier returned is
    /// `1.0 - 0.30 = 0.70`.  If no cold effect is active the multiplier is
    /// `1.0` (no slow).  Clamped to `[0.0, 1.0]`.
    #[must_use]
    pub fn get_slow_factor(&self) -> f32 {
        let total_slow: f32 = self
            .effects
            .iter()
            .filter(|e| e.kind == StatusType::Cold)
            .map(|e| e.potency)
            .sum();
        (1.0_f32 - total_slow / 100.0_f32).clamp(0.0, 1.0)
    }

    /// Returns the total resistance reduction (in points) from all active
    /// [`StatusType::Cursed`], [`StatusType::Amplify`], and
    /// [`StatusType::Decrepify`] effects.
    #[must_use]
    pub fn get_resist_reduction(&self) -> i32 {
        self.effects
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    StatusType::Cursed | StatusType::Amplify | StatusType::Decrepify
                )
            })
            .map(|e| {
                #[allow(clippy::cast_possible_truncation)]
                { e.potency as i32 }
            })
            .sum()
    }

    /// Returns the total poison damage per second from all active
    /// [`StatusType::Poison`] effects.
    #[must_use]
    pub fn get_poison_dps(&self) -> f32 {
        self.effects
            .iter()
            .filter(|e| e.kind == StatusType::Poison)
            .map(|e| e.potency)
            .sum()
    }
}

// ------------------------------------------------------------------ //
// Tests
// ------------------------------------------------------------------ //

#[cfg(test)]
mod tests {
    use super::*;

    fn poison_effect(remaining_ms: u32, dps: f32) -> StatusEffect {
        StatusEffect {
            kind: StatusType::Poison,
            remaining_ms,
            potency: dps,
        }
    }

    fn cold_effect(remaining_ms: u32, slow_pct: f32) -> StatusEffect {
        StatusEffect {
            kind: StatusType::Cold,
            remaining_ms,
            potency: slow_pct,
        }
    }

    fn cursed_effect(remaining_ms: u32, reduction: f32) -> StatusEffect {
        StatusEffect {
            kind: StatusType::Cursed,
            remaining_ms,
            potency: reduction,
        }
    }

    // --- poison_dot_ticks ---

    #[test]
    fn poison_dot_ticks() {
        let mut tracker = StatusTracker::new();
        tracker.apply(poison_effect(3_000, 10.0));

        assert!(tracker.is_affected(StatusType::Poison));

        tracker.tick(1_000);

        // After 1 000 ms, 2 000 ms should remain.
        let remaining = tracker
            .effects
            .iter()
            .find(|e| e.kind == StatusType::Poison)
            .map(|e| e.remaining_ms)
            .expect("poison effect should still be active");
        assert_eq!(remaining, 2_000, "remaining_ms should be 2 000 after a 1 s tick");
    }

    // --- cold_slow_speed ---

    #[test]
    fn cold_slow_speed() {
        let mut tracker = StatusTracker::new();
        tracker.apply(cold_effect(5_000, 30.0)); // 30 % slow

        let factor = tracker.get_slow_factor();
        assert!(
            factor < 1.0,
            "cold effect should give a slow factor below 1.0, got {factor}"
        );
        assert!(
            (factor - 0.7_f32).abs() < f32::EPSILON,
            "30 % slow => factor 0.70, got {factor}"
        );
    }

    // --- status_expire ---

    #[test]
    fn status_expire() {
        let mut tracker = StatusTracker::new();
        tracker.apply(poison_effect(500, 5.0));

        tracker.tick(300);
        assert!(tracker.is_affected(StatusType::Poison), "effect should still be active at 200 ms remaining");

        tracker.tick(300); // total 600 ms > 500 ms duration
        assert!(
            !tracker.is_affected(StatusType::Poison),
            "effect should have expired after total delta exceeds duration"
        );
    }

    // --- same_type_refresh ---

    #[test]
    fn same_type_refresh() {
        let mut tracker = StatusTracker::new();
        tracker.apply(poison_effect(1_000, 5.0));

        // Advance so only 400 ms remain.
        tracker.tick(600);
        assert!(tracker.is_affected(StatusType::Poison));

        // Re-apply with a fresh duration.
        tracker.apply(poison_effect(3_000, 8.0));

        // Should still have exactly one Poison effect with refreshed values.
        let poison_count = tracker.effects.iter().filter(|e| e.kind == StatusType::Poison).count();
        assert_eq!(poison_count, 1, "no stacking: only one Poison entry allowed");

        let effect = tracker
            .effects
            .iter()
            .find(|e| e.kind == StatusType::Poison)
            .unwrap();
        assert_eq!(effect.remaining_ms, 3_000, "duration refreshed to 3 000 ms");
        assert!((effect.potency - 8.0).abs() < f32::EPSILON, "potency updated to 8.0");
    }

    // --- resist_reduction ---

    #[test]
    fn resist_reduction() {
        let mut tracker = StatusTracker::new();
        tracker.apply(cursed_effect(5_000, 40.0)); // Cursed: -40 resist

        let reduction = tracker.get_resist_reduction();
        assert!(reduction > 0, "Cursed should give positive resist reduction, got {reduction}");
        assert_eq!(reduction, 40, "Cursed potency 40 => resist_reduction 40, got {reduction}");
    }
}
