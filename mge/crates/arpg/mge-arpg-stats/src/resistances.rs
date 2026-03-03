// @id: MGE-ARPG-Stats-Resistances @do: elemental-resistances @role: back-end @layer: 3 @human: miyuk
//! Elemental resistance system with difficulty penalties, absorption, and damage reduction.
//!
//! Implements D2-style resistance mechanics:
//! - Base cap at 75% (adjustable via `max_resist_bonus` from items)
//! - Difficulty penalties reduce effective resistances
//! - Monster immunity break (1/5 effectiveness rule)
//! - Absorb reduces damage after resistance

/// Player resistance cap (default 75%).
pub const RESIST_CAP_DEFAULT: i32 = 75;
/// Absolute resistance floor (−100%).
pub const RESIST_FLOOR: i32 = -100;
/// Maximum bonus above the default cap from items (e.g. "All Resist +X").
pub const RESIST_BONUS_MAX: i32 = 20;

/// Four elemental resistances for a character or monster.
#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub struct Resistances {
    /// Fire resistance (%).
    pub fire: i32,
    /// Cold resistance (%).
    pub cold: i32,
    /// Lightning resistance (%).
    pub lightning: i32,
    /// Poison resistance (%).
    pub poison: i32,
}

/// Flat absorption for fire, cold, and lightning damage.
///
/// Absorb is subtracted from damage **after** resistance is applied.
/// Poison has no absorb mechanic in the base D2 ruleset.
#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub struct Absorb {
    /// Fire absorb (flat damage reduction).
    pub fire: i32,
    /// Cold absorb (flat damage reduction).
    pub cold: i32,
    /// Lightning absorb (flat damage reduction).
    pub lightning: i32,
}

/// Returns the resistance penalty applied at a given difficulty level.
///
/// | `difficulty` | Name      | Penalty |
/// |-------------|-----------|---------|
/// | 0           | Normal    |       0 |
/// | 1           | Nightmare |     -40 |
/// | 2           | Hell      |    -100 |
/// | _           | (default) |    -100 |
#[must_use]
pub fn difficulty_resist_penalty(difficulty: u8) -> i32 {
    match difficulty {
        0 => 0,
        1 => -40,
        _ => -100,
    }
}

impl Resistances {
    /// Computes effective (clamped) resistances after applying difficulty penalty and the
    /// maximum resist cap.
    ///
    /// For each element the formula is:
    /// ```text
    /// effective = clamp(base + penalty, RESIST_FLOOR, RESIST_CAP_DEFAULT + max_resist_bonus.min(RESIST_BONUS_MAX))
    /// ```
    ///
    /// `max_resist_bonus` comes from items (e.g. "All Resist +10"). It is capped internally
    /// at [`RESIST_BONUS_MAX`] (20), so the absolute maximum effective resistance is 95.
    #[must_use]
    pub fn effective(&self, difficulty: u8, max_resist_bonus: i32) -> Resistances {
        let penalty = difficulty_resist_penalty(difficulty);
        let cap = RESIST_CAP_DEFAULT + max_resist_bonus.min(RESIST_BONUS_MAX);
        let clamp_one = |base: i32| base.saturating_add(penalty).clamp(RESIST_FLOOR, cap);
        Resistances {
            fire: clamp_one(self.fire),
            cold: clamp_one(self.cold),
            lightning: clamp_one(self.lightning),
            poison: clamp_one(self.poison),
        }
    }

    /// Returns `true` if the resistance for the named element is >= 100 (monster immunity).
    ///
    /// `element` is matched case-insensitively. Unrecognised element names return `false`.
    #[must_use]
    pub fn is_immune(&self, element: &str) -> bool {
        let value = match element.to_ascii_lowercase().as_str() {
            "fire" => self.fire,
            "cold" => self.cold,
            "lightning" => self.lightning,
            "poison" => self.poison,
            _ => return false,
        };
        value >= 100
    }

    /// Applies a resist-reduction effect to an immune monster (resist >= 100).
    ///
    /// When a monster is immune, the reduction only applies at **1/5 effectiveness**:
    /// ```text
    /// new_resist = resist - reduction / 5
    /// ```
    /// The result is clamped to a minimum of 0 (it cannot go negative via this path).
    /// If the monster is not immune (`resist < 100`) the caller should apply the full
    /// reduction elsewhere; this function clamps to 0 regardless.
    #[must_use]
    pub fn break_immunity(resist: i32, reduction: i32) -> i32 {
        (resist - reduction / 5).max(0)
    }
}

/// Calculates final damage dealt after applying resistance and flat absorption.
///
/// Formula:
/// ```text
/// reduced = raw_damage * (100 - resist_pct) / 100
/// final   = (reduced - absorb).max(0)
/// ```
///
/// Notes:
/// - `resist_pct` may be negative (curse / debuff), increasing damage beyond `raw_damage`.
/// - `absorb` is subtracted after resistance; it cannot produce negative damage.
#[must_use]
pub fn damage_after_resist(raw_damage: i32, resist_pct: i32, absorb: i32) -> i32 {
    let reduced = raw_damage * (100 - resist_pct) / 100;
    (reduced - absorb).max(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- difficulty_resist_penalty ----

    #[test]
    fn penalty_normal_is_zero() {
        assert_eq!(difficulty_resist_penalty(0), 0);
    }

    #[test]
    fn penalty_nightmare_is_minus_40() {
        assert_eq!(difficulty_resist_penalty(1), -40);
    }

    #[test]
    fn penalty_hell_is_minus_100() {
        assert_eq!(difficulty_resist_penalty(2), -100);
    }

    #[test]
    fn penalty_unknown_defaults_to_hell() {
        assert_eq!(difficulty_resist_penalty(99), -100);
    }

    // ---- Resistances::effective ----

    /// base fire=75, hell penalty=-100 -> effective fire = 75 + (-100) = -25
    #[test]
    fn resist_hell_penalty() {
        let r = Resistances { fire: 75, cold: 0, lightning: 0, poison: 0 };
        let eff = r.effective(2, 0);
        assert_eq!(eff.fire, -25);
    }

    /// base fire=200, normal difficulty, no bonus -> capped at 75
    #[test]
    fn resist_cap_75() {
        let r = Resistances { fire: 200, cold: 0, lightning: 0, poison: 0 };
        let eff = r.effective(0, 0);
        assert_eq!(eff.fire, 75);
    }

    /// base fire=200, normal difficulty, max_resist_bonus=20 -> capped at 95
    #[test]
    fn resist_max_cap_95() {
        let r = Resistances { fire: 200, cold: 0, lightning: 0, poison: 0 };
        let eff = r.effective(0, 20);
        assert_eq!(eff.fire, 95);
    }

    /// base fire=-200, any difficulty -> floored at -100
    #[test]
    fn resist_clamp_minus_100() {
        let r = Resistances { fire: -200, cold: 0, lightning: 0, poison: 0 };
        let eff = r.effective(0, 0);
        assert_eq!(eff.fire, -100);
    }

    /// max_resist_bonus above 20 is capped at 20 internally
    #[test]
    fn resist_bonus_capped_at_20() {
        let r = Resistances { fire: 200, cold: 0, lightning: 0, poison: 0 };
        // bonus=50 should still give cap=75+20=95, not 75+50=125
        let eff = r.effective(0, 50);
        assert_eq!(eff.fire, 95);
    }

    // ---- Resistances::is_immune ----

    #[test]
    fn immune_at_100() {
        let r = Resistances { fire: 100, cold: 80, lightning: 99, poison: 0 };
        assert!(r.is_immune("fire"));
        assert!(!r.is_immune("cold"));
        assert!(!r.is_immune("lightning"));
        assert!(!r.is_immune("poison"));
    }

    #[test]
    fn immune_above_100() {
        let r = Resistances { fire: 150, cold: 0, lightning: 0, poison: 0 };
        assert!(r.is_immune("fire"));
    }

    #[test]
    fn immune_unknown_element() {
        let r = Resistances { fire: 200, cold: 200, lightning: 200, poison: 200 };
        assert!(!r.is_immune("arcane"));
    }

    #[test]
    fn immune_case_insensitive() {
        let r = Resistances { fire: 100, cold: 0, lightning: 0, poison: 0 };
        assert!(r.is_immune("FIRE"));
        assert!(r.is_immune("Fire"));
    }

    // ---- Resistances::break_immunity ----

    /// resist=110, reduction=50 -> 110 - 50/5 = 100 (still immune but reduced)
    #[test]
    fn immunity_break() {
        assert_eq!(Resistances::break_immunity(110, 50), 100);
    }

    /// resist=100, reduction=100 -> 100 - 20 = 80 (no longer immune)
    #[test]
    fn immunity_break_removes_immunity() {
        assert_eq!(Resistances::break_immunity(100, 100), 80);
    }

    /// Result is clamped to 0 minimum
    #[test]
    fn immunity_break_clamp_zero() {
        assert_eq!(Resistances::break_immunity(10, 1000), 0);
    }

    // ---- damage_after_resist ----

    /// raw=100, resist=50% -> reduced=50, absorb=0 -> final=50
    #[test]
    fn damage_after_resist_50() {
        assert_eq!(damage_after_resist(100, 50, 0), 50);
    }

    /// raw=100, resist=-50% -> 100*(100-(-50))/100 = 100*150/100 = 150 damage
    #[test]
    fn damage_after_resist_negative() {
        assert_eq!(damage_after_resist(100, -50, 0), 150);
    }

    /// raw=100, resist=0%, absorb=30 -> reduced=100, final=100-30=70
    #[test]
    fn damage_after_absorb_reduces() {
        assert_eq!(damage_after_resist(100, 0, 30), 70);
    }

    /// Absorb cannot produce negative damage
    #[test]
    fn damage_after_absorb_no_negative() {
        assert_eq!(damage_after_resist(50, 0, 200), 0);
    }

    /// resist=100% -> zero damage
    #[test]
    fn damage_after_full_resist() {
        assert_eq!(damage_after_resist(100, 100, 0), 0);
    }
}
