// @id: MGE-ARPG-Necro @do: necromancer-skills @role: back-end @layer: 3 @human: miyuk
//! Necromancer skill tree content definitions.
//!
//! Provides factory functions for the three core Necromancer skills:
//! - **Bone Spear** — Magic-damage projectile
//! - **Raise Skeleton** — Summon skill (count scales with level)
//! - **Corpse Explosion** — Fire+Physical AoE (damage = % of corpse HP)

use mge_arpg_combat::DamageType;

use crate::{SkillDef, SkillId, SkillKind, SynergyDef};

/// Necromancer skill tree identifier (tree index 0).
pub const NECRO_TREE: u8 = 0;

/// Skill ID constant for Bone Spear.
pub const BONE_SPEAR_ID: &str = "necro_bone_spear";
/// Skill ID constant for Raise Skeleton.
pub const RAISE_SKELETON_ID: &str = "necro_raise_skeleton";
/// Skill ID constant for Corpse Explosion.
pub const CORPSE_EXPLOSION_ID: &str = "necro_corpse_explosion";

/// Create the Bone Spear skill definition.
///
/// - **Type**: Magic projectile
/// - **Mana**: 7 (flat, no per-level scaling)
/// - **Damage**: 15-17 at level 1, +5 per level
/// - **Cooldown**: 0 ms (spammable)
/// - **Synergies**: Receives +8% damage per level of Raise Skeleton
pub fn bone_spear() -> SkillDef {
    SkillDef {
        id: SkillId::new(BONE_SPEAR_ID),
        name: "Bone Spear".to_string(),
        max_level: 20,
        prerequisites: Vec::new(),
        synergies: vec![SynergyDef {
            source_skill: SkillId::new(RAISE_SKELETON_ID),
            bonus_per_level: 0.08,
            stat: "damage".to_string(),
        }],
        mana_cost_base: 7.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 0,
        tree: NECRO_TREE,
        damage_type: DamageType::Magic,
        kind: SkillKind::Projectile,
        base_damage_min: 15,
        base_damage_max: 17,
        damage_per_level: 5,
        synergy_ids: vec![RAISE_SKELETON_ID.to_string()],
    }
}

/// Create the Raise Skeleton skill definition.
///
/// - **Type**: Summon (no direct damage)
/// - **Mana**: 6 (flat)
/// - **Damage**: 0 (summons deal their own damage independently)
/// - **Cooldown**: 500 ms
/// - **Summon count**: `(level / 3) + 2` — use [`raise_skeleton_max_count`]
pub fn raise_skeleton() -> SkillDef {
    SkillDef {
        id: SkillId::new(RAISE_SKELETON_ID),
        name: "Raise Skeleton".to_string(),
        max_level: 20,
        prerequisites: Vec::new(),
        synergies: Vec::new(),
        mana_cost_base: 6.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 500,
        tree: NECRO_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::Summon,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Calculate the maximum skeleton count at the given skill level.
///
/// Formula: `(level / 3) + 2`.
/// At level 0 (not learned): 2 skeletons.
/// At level 1: 2, level 3: 3, level 6: 4, ..., level 18: 8, level 20: 8.
pub fn raise_skeleton_max_count(level: u32) -> u32 {
    level / 3 + 2
}

/// Create the Corpse Explosion skill definition.
///
/// - **Type**: Fire+Physical AoE (primary type is Fire; actual damage is
///   a percentage of the corpse's max HP, applied as mixed Fire+Physical)
/// - **Mana**: 15 (flat)
/// - **Damage**: 60-100 (represents percentage of corpse HP at level 1)
/// - **Damage per level**: +5% per level (additive to both min and max)
/// - **Cooldown**: 200 ms
/// - **Synergies**: Receives +6% damage per level of Bone Spear
pub fn corpse_explosion() -> SkillDef {
    SkillDef {
        id: SkillId::new(CORPSE_EXPLOSION_ID),
        name: "Corpse Explosion".to_string(),
        max_level: 20,
        prerequisites: Vec::new(),
        synergies: vec![SynergyDef {
            source_skill: SkillId::new(BONE_SPEAR_ID),
            bonus_per_level: 0.06,
            stat: "damage".to_string(),
        }],
        mana_cost_base: 15.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 200,
        tree: NECRO_TREE,
        damage_type: DamageType::Fire,
        kind: SkillKind::AreaOfEffect,
        base_damage_min: 60,
        base_damage_max: 100,
        damage_per_level: 5,
        synergy_ids: vec![BONE_SPEAR_ID.to_string()],
    }
}

/// Register all three Necromancer skills into the given registry.
pub fn register_all(registry: &mut crate::SkillRegistry) {
    registry.register(bone_spear());
    registry.register(raise_skeleton());
    registry.register(corpse_explosion());
}
