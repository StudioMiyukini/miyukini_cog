// @id: Sodomight-Boss @do: boss-andariel @role: back-end @layer: 4 @human: miyuk
//! Boss definitions for Sodomight.
//!
//! Provides static definition and runtime state for Act 1 final boss Andariel,
//! the Maiden of Anguish. Implements a two-phase fight with rage mode above 50% HP lost.

use crate::content::{MonsterDef, MonsterRank};

// ---------------------------------------------------------------------------
// Static definition
// ---------------------------------------------------------------------------

/// Static definition for the Act 1 boss Andariel, the Maiden of Anguish.
///
/// Encodes canonical D2 stats: fire weakness (-50), poison resistance (+75),
/// and a two-phase rage transition at 50% HP.
#[derive(Debug, Clone)]
pub struct AndarielDef {
    /// Base monster definition (stats, name, TC, XP).
    pub base: MonsterDef,
    /// Boss rank — always `SuperUnique`.
    pub rank: MonsterRank,
    /// Fire resistance modifier. Negative values mean weakness (extra damage taken).
    pub fire_resist: i32,
    /// Poison resistance modifier.
    pub poison_resist: i32,
    /// Cold resistance modifier.
    pub cold_resist: i32,
    /// Lightning resistance modifier.
    pub lightning_resist: i32,
    /// Fraction of max HP at which rage phase begins (e.g. `0.5` = 50 %).
    pub rage_threshold: f32,
    /// Damage multiplier applied in rage phase.
    pub rage_damage_mult: f32,
    /// Movement speed multiplier applied in rage phase.
    pub rage_speed_mult: f32,
    /// Guaranteed treasure class used for the quest kill drop.
    pub quest_drop_tc: String,
}

/// Construct the canonical Andariel definition using D2-accurate base stats.
///
/// Fire resist of -50 means Andariel takes 50 % *extra* fire damage, matching
/// the classic D2 fire-Nova strategy. Poison resist of +75 makes her largely
/// immune to poison.
#[must_use]
pub fn andariel() -> AndarielDef {
    AndarielDef {
        base: MonsterDef {
            id: "andariel".to_owned(),
            name: "Andariel".to_owned(),
            level: 12,
            health: 1024,
            min_damage: 60,
            max_damage: 80,
            attack_rating: 120,
            defense_rating: 40,
            speed: 1.2,
            aggro_range: 20.0,
            xp_reward: 5000,
            tc_id: "tc85_boss".to_owned(),
        },
        rank: MonsterRank::SuperUnique,
        fire_resist: -50,   // D2 canon: fire weakness
        poison_resist: 75,  // D2 canon: strong poison resist
        cold_resist: 0,
        lightning_resist: 0,
        rage_threshold: 0.5,
        rage_damage_mult: 1.5,
        rage_speed_mult: 1.3,
        quest_drop_tc: "tc85_boss_quest".to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Runtime state
// ---------------------------------------------------------------------------

/// Phase of the Andariel encounter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BossPhase {
    /// Normal phase (above `rage_threshold` HP).
    Phase1,
    /// Rage phase — boosted damage and speed (at or below `rage_threshold` HP).
    Phase2Rage,
}

/// Runtime state for an active Andariel encounter.
///
/// Tracks current HP, phase transitions, and death. All mutation goes through
/// `take_damage` which keeps the invariants consistent.
#[derive(Debug, Clone)]
pub struct BossState {
    /// Current hit points.
    pub current_hp: u32,
    /// Maximum hit points (set from `AndarielDef.base.health` at spawn).
    pub max_hp: u32,
    /// Current fight phase.
    pub phase: BossPhase,
    /// Set to `true` when `current_hp` reaches zero.
    pub is_dead: bool,
}

impl BossState {
    /// Create a fresh boss state for the given definition.
    ///
    /// HP is clamped to zero from below — negative `base.health` (editor error)
    /// becomes 0 rather than wrapping.
    #[must_use]
    pub fn new(def: &AndarielDef) -> Self {
        let max_hp = u32::try_from(def.base.health).unwrap_or(0);
        Self {
            current_hp: max_hp,
            max_hp,
            phase: BossPhase::Phase1,
            is_dead: false,
        }
    }

    /// Apply `amount` raw damage to the boss, then update phase and death flag.
    ///
    /// Uses saturating subtraction so HP never wraps below zero.
    pub fn take_damage(&mut self, amount: u32, def: &AndarielDef) {
        self.current_hp = self.current_hp.saturating_sub(amount);

        // Phase transition: check whether HP has dropped to or below the rage threshold.
        let rage_hp = (def.rage_threshold * self.max_hp as f32) as u32;
        if self.current_hp <= rage_hp && self.phase == BossPhase::Phase1 {
            self.phase = BossPhase::Phase2Rage;
        }

        if self.current_hp == 0 {
            self.is_dead = true;
        }
    }

    /// Current damage output of the boss, accounting for rage phase.
    ///
    /// Returns `base.max_damage` (the upper bound of the damage range) scaled by
    /// `rage_damage_mult` when in rage phase. Cast truncates toward zero.
    #[must_use]
    pub fn effective_damage(&self, def: &AndarielDef) -> u32 {
        let base = def.base.max_damage as f32;
        let scaled = match self.phase {
            BossPhase::Phase1 => base,
            BossPhase::Phase2Rage => base * def.rage_damage_mult,
        };
        scaled as u32
    }

    /// Current movement speed of the boss, accounting for rage phase.
    #[must_use]
    pub fn effective_speed(&self, def: &AndarielDef) -> f32 {
        match self.phase {
            BossPhase::Phase1 => def.base.speed,
            BossPhase::Phase2Rage => def.base.speed * def.rage_speed_mult,
        }
    }
}

// ---------------------------------------------------------------------------
// Damage utility functions
// ---------------------------------------------------------------------------

/// Calculate fire damage dealt to Andariel after applying her resistance.
///
/// A negative `fire_resist` (e.g. -50) means weakness: the multiplier exceeds
/// 1.0, so more damage is dealt. Formula: `raw * (1 - resist / 100)`.
#[must_use]
pub fn fire_damage_to_andariel(raw_damage: u32, def: &AndarielDef) -> u32 {
    let mult = 1.0_f32 - (def.fire_resist as f32 / 100.0_f32);
    (raw_damage as f32 * mult) as u32
}

/// Calculate poison damage dealt to Andariel after applying her resistance.
///
/// Resistance is clamped to [0, 95] so at least 5 % poison damage always
/// applies — preventing zero-damage edge cases from full immunity.
#[must_use]
pub fn poison_damage_to_andariel(raw_damage: u32, def: &AndarielDef) -> u32 {
    let resist_clamped = (def.poison_resist as f32 / 100.0_f32).clamp(0.0_f32, 0.95_f32);
    let mult = 1.0_f32 - resist_clamped;
    (raw_damage as f32 * mult) as u32
}

/// Returns `true` when the quest drop treasure class is guaranteed (non-empty TC).
///
/// An empty `quest_drop_tc` signals that no guaranteed drop is configured
/// (NoDrop = 0 equivalent).
#[must_use]
pub fn is_quest_drop_guaranteed(def: &AndarielDef) -> bool {
    !def.quest_drop_tc.is_empty()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_andariel() -> AndarielDef {
        andariel()
    }

    // 1. Fire weakness: -50 resist -> 150 % damage (1.5x multiplier)
    #[test]
    fn andariel_fire_weak() {
        let def = make_andariel();
        assert_eq!(def.fire_resist, -50);
        assert_eq!(fire_damage_to_andariel(100, &def), 150);
    }

    // 2. Poison strong: 75 % resist -> 25 % of raw damage taken
    #[test]
    fn andariel_poison_strong() {
        let def = make_andariel();
        assert_eq!(def.poison_resist, 75);
        assert_eq!(poison_damage_to_andariel(100, &def), 25);
    }

    // 3. Taking damage below 50 % HP triggers Phase2Rage
    #[test]
    fn andariel_rage_phase() {
        let def = make_andariel();
        let mut state = BossState::new(&def);
        assert_eq!(state.phase, BossPhase::Phase1);

        // Bring HP down to exactly the rage threshold boundary.
        // max_hp = 1024, threshold = 0.5 -> rage_hp = 512.
        // Deal 513 damage: current_hp = 1024 - 513 = 511 <= 512 -> Phase2Rage.
        state.take_damage(513, &def);
        assert_eq!(state.phase, BossPhase::Phase2Rage);
    }

    // 4. Effective damage in rage phase: max_damage(80) * 1.5 = 120
    #[test]
    fn andariel_rage_damage() {
        let def = make_andariel();
        let mut state = BossState::new(&def);
        // Force rage phase by dealing heavy damage.
        state.take_damage(600, &def);
        assert_eq!(state.phase, BossPhase::Phase2Rage);
        assert_eq!(state.effective_damage(&def), 120);
    }

    // 5. Effective speed in rage phase: 1.2 * 1.3 = 1.56
    #[test]
    fn andariel_rage_speed() {
        let def = make_andariel();
        let mut state = BossState::new(&def);
        state.take_damage(600, &def);
        assert_eq!(state.phase, BossPhase::Phase2Rage);
        let speed = state.effective_speed(&def);
        // Use epsilon comparison for f32 arithmetic.
        assert!((speed - 1.56_f32).abs() < 1e-4_f32, "expected ~1.56, got {speed}");
    }

    // 6. Taking more damage than total HP results in is_dead = true
    #[test]
    fn andariel_death() {
        let def = make_andariel();
        let mut state = BossState::new(&def);
        state.take_damage(2000, &def);
        assert!(state.is_dead);
        assert_eq!(state.current_hp, 0);
    }

    // 7. Quest drop TC is non-empty -> guaranteed drop
    #[test]
    fn andariel_guaranteed_drop() {
        let def = make_andariel();
        assert!(is_quest_drop_guaranteed(&def));
    }

    // 8. Base HP from factory matches D2 canon value
    #[test]
    fn andariel_hp_1024() {
        let def = make_andariel();
        assert_eq!(def.base.health, 1024);
    }

    // 9. Rank is always SuperUnique
    #[test]
    fn andariel_super_unique() {
        let def = make_andariel();
        assert_eq!(def.rank, MonsterRank::SuperUnique);
    }
}
