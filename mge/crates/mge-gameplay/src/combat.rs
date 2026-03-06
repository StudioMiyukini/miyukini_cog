use serde::{Deserialize, Serialize};

use crate::actions::{EntityId, WorldPos};

/// Damage type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    Physical,
    Fire,
    Cold,
    Lightning,
    Poison,
    Magic,
}

/// A damage roll result.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DamageResult {
    pub amount: f32,
    pub damage_type: DamageType,
    pub is_critical: bool,
}

/// Attacker stats needed to compute damage.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AttackerProfile {
    pub attack_rating: i32,
    pub damage_min: i32,
    pub damage_max: i32,
    pub crit_chance_pct: i32,
    pub crit_multiplier: f32,
}

/// Defender stats needed to resolve a hit.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DefenderProfile {
    pub defense: i32,
    pub block_chance_pct: i32,
    pub fire_resist: i32,
    pub cold_resist: i32,
    pub lightning_resist: i32,
    pub poison_resist: i32,
    pub life: f32,
}

// ─── Melee pipeline (T07) ──────────────────────────────────────────────────

/// D2-like hit chance formula: attacker / (attacker + defender) * 100.
#[allow(clippy::cast_precision_loss)]
pub fn hit_chance(attack_rating: i32, defense: i32) -> f32 {
    let ar = attack_rating.max(0) as f32;
    let def = defense.max(0) as f32;
    if ar + def <= 0.0 {
        return 95.0;
    }
    (ar / (ar + def) * 100.0).clamp(5.0, 95.0)
}

/// Returns true if the hit connects given a `[0,1)` roll.
pub fn melee_hits(attack_rating: i32, defense: i32, roll: f32) -> bool {
    roll * 100.0 < hit_chance(attack_rating, defense)
}

/// Returns true if the defender blocks (given a roll in `[0,1)`).
#[allow(clippy::cast_precision_loss)]
pub fn defender_blocks(block_chance_pct: i32, roll: f32) -> bool {
    roll * 100.0 < block_chance_pct as f32
}

/// Melee damage roll: picks a value in `[damage_min, damage_max]` using `roll in [0,1)`.
#[allow(clippy::cast_precision_loss)]
pub fn melee_damage(attacker: &AttackerProfile, crit_roll: f32, damage_roll: f32) -> DamageResult {
    let base_min = attacker.damage_min.max(0) as f32;
    let base_max = attacker.damage_max.max(attacker.damage_min).max(0) as f32;
    let raw = base_min + (base_max - base_min) * damage_roll;
    let is_crit = crit_roll * 100.0 < attacker.crit_chance_pct as f32;
    let amount = if is_crit { raw * attacker.crit_multiplier } else { raw };
    DamageResult { amount, damage_type: DamageType::Physical, is_critical: is_crit }
}

/// Apply physical damage after defense reduction (simple formula, not DR).
#[allow(clippy::cast_precision_loss)]
pub fn apply_physical_damage(damage: f32, defender_defense: i32) -> f32 {
    let reduction = (defender_defense as f32 / (defender_defense as f32 + 100.0)).clamp(0.0, 0.85);
    damage * (1.0 - reduction)
}

// ─── Ranged pipeline (T08) ────────────────────────────────────────────────

/// Projectile state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projectile {
    pub id: u64,
    pub owner: EntityId,
    pub position: WorldPos,
    pub velocity: (f32, f32),
    pub speed: f32,
    pub range: f32,
    pub distance_traveled: f32,
    pub pierce_remaining: u8,
    pub skill_id: u32,
}

impl Projectile {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: u64,
        owner: EntityId,
        start: WorldPos,
        target: WorldPos,
        speed: f32,
        range: f32,
        pierce: u8,
        skill_id: u32,
    ) -> Self {
        let dx = target.x - start.x;
        let dy = target.y - start.y;
        let len = (dx * dx + dy * dy).sqrt().max(f32::EPSILON);
        Self {
            id,
            owner,
            position: start,
            velocity: (dx / len, dy / len),
            speed,
            range,
            distance_traveled: 0.0,
            pierce_remaining: pierce,
            skill_id,
        }
    }

    /// Advance the projectile by `dt` seconds. Returns `true` if still alive.
    pub fn tick(&mut self, dt: f32) -> bool {
        let step = self.speed * dt;
        self.position.x += self.velocity.0 * step;
        self.position.y += self.velocity.1 * step;
        self.distance_traveled += step;
        self.distance_traveled < self.range
    }

    /// Called when the projectile hits a target. Returns `true` if it pierces.
    pub fn on_hit(&mut self) -> bool {
        if self.pierce_remaining > 0 {
            self.pierce_remaining -= 1;
            true
        } else {
            false
        }
    }

    pub fn is_in_range(&self, pos: WorldPos, radius: f32) -> bool {
        self.position.distance_sq(pos) <= radius * radius
    }
}

// ─── Spell pipeline (T09) ────────────────────────────────────────────────

/// Phase of a spell cast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CastPhase {
    /// Resource check + animation wind-up.
    Charging,
    /// Spell effect is released.
    Released,
    /// Cast finished.
    Completed,
}

/// A spell being cast.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellCast {
    pub skill_id: u32,
    pub caster: EntityId,
    pub mana_cost: f32,
    pub cast_time: f32,
    pub elapsed: f32,
    pub phase: CastPhase,
    pub interrupted: bool,
}

impl SpellCast {
    pub fn new(skill_id: u32, caster: EntityId, mana_cost: f32, cast_time: f32) -> Self {
        Self {
            skill_id,
            caster,
            mana_cost,
            cast_time,
            elapsed: 0.0,
            phase: CastPhase::Charging,
            interrupted: false,
        }
    }

    /// Tick the cast. Returns the new phase.
    pub fn tick(&mut self, dt: f32) -> CastPhase {
        if self.interrupted {
            self.phase = CastPhase::Completed;
            return self.phase;
        }
        self.elapsed += dt;
        if self.elapsed >= self.cast_time && self.phase == CastPhase::Charging {
            self.phase = CastPhase::Released;
        } else if self.phase == CastPhase::Released {
            self.phase = CastPhase::Completed;
        }
        self.phase
    }

    pub fn interrupt(&mut self) {
        self.interrupted = true;
    }

    pub fn is_complete(&self) -> bool {
        self.phase == CastPhase::Completed
    }
}

/// Apply elemental resistance to damage.
#[allow(clippy::cast_precision_loss)]
pub fn apply_resistance(damage: f32, resist_pct: i32) -> f32 {
    let eff_resist = resist_pct.clamp(-100, 75) as f32;
    damage * (1.0 - eff_resist / 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Melee ───────────────────────────────────────────────────────────────

    #[test]
    fn hit_chance_clamps_at_bounds() {
        assert!((hit_chance(0, 1000) - 5.0).abs() < 0.1);
        assert!((hit_chance(1000, 0) - 95.0).abs() < 0.1);
    }

    #[test]
    fn melee_hit_misses_with_high_defense() {
        let attacker = AttackerProfile {
            attack_rating: 1,
            damage_min: 10,
            damage_max: 20,
            crit_chance_pct: 0,
            crit_multiplier: 2.0,
        };
        let hits = melee_hits(attacker.attack_rating, 99999, 0.94);
        assert!(!hits);
    }

    #[test]
    fn crit_doubles_damage() {
        let attacker = AttackerProfile {
            attack_rating: 1000,
            damage_min: 100,
            damage_max: 100,
            crit_chance_pct: 100,
            crit_multiplier: 2.0,
        };
        let result = melee_damage(&attacker, 0.0, 0.5);
        assert!(result.is_critical);
        assert!((result.amount - 200.0).abs() < 0.1);
    }

    #[test]
    fn physical_damage_reduced_by_defense() {
        let reduced = apply_physical_damage(100.0, 100);
        // defense 100: reduction = 100/200 = 50%
        assert!((reduced - 50.0).abs() < 1.0);
    }

    // ── Ranged ──────────────────────────────────────────────────────────────

    #[test]
    fn projectile_moves_toward_target() {
        let mut proj = Projectile::new(
            1,
            1,
            WorldPos::new(0.0, 0.0),
            WorldPos::new(10.0, 0.0),
            20.0,
            100.0,
            0,
            300,
        );
        proj.tick(0.1);
        assert!(proj.position.x > 0.0);
    }

    #[test]
    fn projectile_expires_at_range() {
        let mut proj = Projectile::new(
            1,
            1,
            WorldPos::new(0.0, 0.0),
            WorldPos::new(10.0, 0.0),
            20.0,
            5.0,
            0,
            300,
        );
        // tick enough to exceed range
        let alive = proj.tick(1.0);
        assert!(!alive);
    }

    #[test]
    fn projectile_pierce_allows_multiple_hits() {
        let mut proj = Projectile::new(
            1,
            1,
            WorldPos::new(0.0, 0.0),
            WorldPos::new(10.0, 0.0),
            10.0,
            100.0,
            2,
            200,
        );
        assert!(proj.on_hit()); // pierces
        assert!(proj.on_hit()); // pierces again
        assert!(!proj.on_hit()); // stops
    }

    // ── Spell ───────────────────────────────────────────────────────────────

    #[test]
    fn spell_cast_transitions_phases() {
        let mut cast = SpellCast::new(300, 1, 12.0, 0.5);
        assert_eq!(cast.tick(0.3), CastPhase::Charging);
        assert_eq!(cast.tick(0.3), CastPhase::Released);
        assert_eq!(cast.tick(0.01), CastPhase::Completed);
    }

    #[test]
    fn spell_interrupt_completes_immediately() {
        let mut cast = SpellCast::new(300, 1, 12.0, 1.0);
        cast.interrupt();
        assert_eq!(cast.tick(0.1), CastPhase::Completed);
    }

    #[test]
    fn resistance_reduces_spell_damage() {
        let damage = apply_resistance(100.0, 50);
        assert!((damage - 50.0).abs() < 0.1);
    }

    #[test]
    fn negative_resistance_increases_damage() {
        let damage = apply_resistance(100.0, -50);
        assert!((damage - 150.0).abs() < 0.1);
    }
}
