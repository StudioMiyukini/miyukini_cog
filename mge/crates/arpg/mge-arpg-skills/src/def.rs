// @id: MGE-ARPG-SkillDef @do: definition @role: back-end @layer: 3 @human: miyuk
//! Static skill definitions and synergy descriptors.

use mge_arpg_combat::DamageType;

use crate::SkillId;

/// Describes a passive synergy bonus that a skill receives from another skill.
///
/// When the source skill is levelled up, the target skill gains
/// `bonus_per_level * source_level` to the specified stat.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SynergyDef {
    /// The skill whose invested levels provide the bonus.
    pub source_skill: SkillId,
    /// Bonus multiplier per level of the source skill (e.g. 0.06 = +6%).
    pub bonus_per_level: f32,
    /// The stat affected by this synergy (e.g. "damage", "duration", "radius").
    pub stat: String,
}

/// The delivery mechanism of a skill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SkillKind {
    /// Direct projectile (e.g. Bone Spear).
    Projectile,
    /// Summons a creature or object.
    Summon,
    /// Area-of-effect explosion or zone.
    AreaOfEffect,
    /// A passive buff or aura.
    Passive,
}

/// Static definition of a skill — its costs, prerequisites, synergies and tree slot.
///
/// This struct describes the *template* of a skill, not a character's
/// investment in it. See [`crate::SkillBook`] for per-character state.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SkillDef {
    /// Unique identifier for this skill.
    pub id: SkillId,
    /// Human-readable name.
    pub name: String,
    /// Maximum investable level (typically 20 in D2-style games).
    pub max_level: u32,
    /// Skills that must be at level >= 1 before this skill can be invested.
    pub prerequisites: Vec<SkillId>,
    /// Passive synergy bonuses this skill receives from other skills.
    pub synergies: Vec<SynergyDef>,
    /// Base mana cost at level 0 (before per-level scaling).
    pub mana_cost_base: f32,
    /// Additional mana cost added per invested level.
    pub mana_cost_per_level: f32,
    /// Cooldown duration in milliseconds. 0 means no cooldown.
    pub cooldown_ms: u32,
    /// Skill tree index within a class (0, 1, or 2).
    pub tree: u8,
    /// Primary damage type dealt by this skill.
    pub damage_type: DamageType,
    /// Delivery mechanism of the skill (projectile, summon, aoe, passive).
    pub kind: SkillKind,
    /// Minimum base damage at level 1.
    pub base_damage_min: i32,
    /// Maximum base damage at level 1.
    pub base_damage_max: i32,
    /// Additional flat damage per invested level.
    pub damage_per_level: i32,
    /// IDs of skills that provide synergy bonuses to this skill (quick lookup).
    pub synergy_ids: Vec<String>,
}

impl SkillDef {
    /// Calculate the mana cost at the given invested level.
    ///
    /// Formula: `mana_cost_base + mana_cost_per_level * level`
    pub fn mana_cost(&self, level: u32) -> f32 {
        self.mana_cost_base + self.mana_cost_per_level * level as f32
    }

    /// Calculate the damage range at a given skill level.
    ///
    /// Returns `(min_damage, max_damage)` after per-level scaling.
    pub fn damage_range(&self, level: u32) -> (i32, i32) {
        #[allow(clippy::cast_possible_wrap)]
        let bonus = self.damage_per_level * level as i32;
        (self.base_damage_min + bonus, self.base_damage_max + bonus)
    }
}
