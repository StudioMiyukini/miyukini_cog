// @id: MGE-ARPG-PoisonBone @do: poison-bone-skills @role: back-end @layer: 3 @human: miyuk
//! Necromancer Poison & Bone skill tree content definitions.
//!
//! Provides factory functions for the ten core Poison & Bone tree skills:
//! - **Teeth** — Multi-projectile magic attack
//! - **Bone Armor** — Passive shield that absorbs damage (absorb 20 base, +10/lvl)
//! - **Poison Dagger** — Poison projectile requiring Teeth
//! - **Corpse Explosion** — Fire AoE triggered on a corpse (% of corpse HP)
//! - **Bone Wall** — Summon a bone wall barrier (HP = 50 + 10*lvl)
//! - **Poison Explosion** — Poison AoE cloud over a corpse
//! - **Bone Spear** — Magic projectile requiring Bone Wall + Teeth
//! - **Bone Prison** — Summon a circular bone cage (HP = 80 + 15*lvl)
//! - **Poison Nova** — Poison AoE nova emanating from the caster
//! - **Bone Spirit** — Homing magic projectile requiring Bone Spear

use mge_arpg_combat::DamageType;

use crate::{SkillDef, SkillId, SkillKind, SynergyDef};

/// Necromancer Poison & Bone skill tree identifier (tree index 1).
pub const POISON_BONE_TREE: u8 = 1;

/// Skill ID constant for Teeth.
pub const TEETH_ID: &str = "necro_teeth";
/// Skill ID constant for Bone Armor.
pub const BONE_ARMOR_ID: &str = "necro_bone_armor";
/// Skill ID constant for Poison Dagger.
pub const POISON_DAGGER_ID: &str = "necro_poison_dagger";
/// Skill ID constant for Corpse Explosion (Poison & Bone tree variant).
pub const PB_CORPSE_EXPLOSION_ID: &str = "necro_pb_corpse_explosion";
/// Skill ID constant for Bone Wall.
pub const BONE_WALL_ID: &str = "necro_bone_wall";
/// Skill ID constant for Poison Explosion.
pub const POISON_EXPLOSION_ID: &str = "necro_poison_explosion";
/// Skill ID constant for Bone Spear (Poison & Bone tree variant).
pub const PB_BONE_SPEAR_ID: &str = "necro_pb_bone_spear";
/// Skill ID constant for Bone Prison.
pub const BONE_PRISON_ID: &str = "necro_bone_prison";
/// Skill ID constant for Poison Nova.
pub const POISON_NOVA_ID: &str = "necro_poison_nova";
/// Skill ID constant for Bone Spirit.
pub const BONE_SPIRIT_ID: &str = "necro_bone_spirit";

/// Create the Teeth skill definition.
///
/// - **Type**: Projectile — fires multiple magic bone teeth in a fan
/// - **Mana**: 4 (flat)
/// - **Damage**: 2-3 at level 1, +1 per level
/// - **Cooldown**: 0 ms (spammable)
/// - **Req Level**: 1 — no prerequisites
pub fn teeth() -> SkillDef {
    SkillDef {
        id: SkillId::new(TEETH_ID),
        name: "Teeth".to_string(),
        max_level: 20,
        prerequisites: Vec::new(),
        synergies: Vec::new(),
        mana_cost_base: 4.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 0,
        tree: POISON_BONE_TREE,
        damage_type: DamageType::Magic,
        kind: SkillKind::Projectile,
        base_damage_min: 2,
        base_damage_max: 3,
        damage_per_level: 1,
        synergy_ids: Vec::new(),
    }
}

/// Create the Bone Armor skill definition.
///
/// - **Type**: Passive — creates an absorbing bone shield around the caster
/// - **Mana**: 11 (flat)
/// - **Cooldown**: 0 ms
/// - **Req Level**: 1 — no prerequisites
/// - **Absorb**: 20 damage base, +10 per level (managed by the combat system)
/// - **Damage**: 0 (purely defensive utility)
pub fn bone_armor() -> SkillDef {
    SkillDef {
        id: SkillId::new(BONE_ARMOR_ID),
        name: "Bone Armor".to_string(),
        max_level: 20,
        prerequisites: Vec::new(),
        synergies: Vec::new(),
        mana_cost_base: 11.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 0,
        tree: POISON_BONE_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::Passive,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Poison Dagger skill definition.
///
/// - **Type**: Projectile — coats the weapon with poison before throwing
/// - **Mana**: 5 (flat)
/// - **Damage**: 8-12 at level 1, +4 per level
/// - **Cooldown**: 0 ms (spammable)
/// - **Req Level**: 6 — requires Teeth
pub fn poison_dagger() -> SkillDef {
    SkillDef {
        id: SkillId::new(POISON_DAGGER_ID),
        name: "Poison Dagger".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(TEETH_ID)],
        synergies: Vec::new(),
        mana_cost_base: 5.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 0,
        tree: POISON_BONE_TREE,
        damage_type: DamageType::Poison,
        kind: SkillKind::Projectile,
        base_damage_min: 8,
        base_damage_max: 12,
        damage_per_level: 4,
        synergy_ids: Vec::new(),
    }
}

/// Create the Corpse Explosion skill definition (Poison & Bone tree variant).
///
/// - **Type**: Fire AoE — detonates a corpse for fire+physical splash damage
/// - **Mana**: 15 (flat)
/// - **Damage**: 60-100 at level 1 (represents % of corpse HP), +5 per level
/// - **Cooldown**: 200 ms
/// - **Req Level**: 6 — requires Teeth
/// - **Synergies**: Receives +6% damage per level of Bone Spear (P&B variant)
///
/// Note: this is the Poison & Bone tree variant (`PB_CORPSE_EXPLOSION_ID`).
/// The standalone legacy version lives in `necro.rs` (`CORPSE_EXPLOSION_ID`).
pub fn pb_corpse_explosion() -> SkillDef {
    SkillDef {
        id: SkillId::new(PB_CORPSE_EXPLOSION_ID),
        name: "Corpse Explosion".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(TEETH_ID)],
        synergies: vec![SynergyDef {
            source_skill: SkillId::new(PB_BONE_SPEAR_ID),
            bonus_per_level: 0.06,
            stat: "damage".to_string(),
        }],
        mana_cost_base: 15.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 200,
        tree: POISON_BONE_TREE,
        damage_type: DamageType::Fire,
        kind: SkillKind::AreaOfEffect,
        base_damage_min: 60,
        base_damage_max: 100,
        damage_per_level: 5,
        synergy_ids: vec![PB_BONE_SPEAR_ID.to_string()],
    }
}

/// Create the Bone Wall skill definition.
///
/// - **Type**: Summon — raises a wall of bones that blocks enemy pathing
/// - **Mana**: 17 (flat)
/// - **Cooldown**: 1000 ms
/// - **Req Level**: 12 — requires Bone Armor
/// - **HP**: 50 base, +10 per level (HP = 50 + 10*lvl, managed by the AI system)
/// - **Damage**: 0 (purely defensive barrier)
pub fn bone_wall() -> SkillDef {
    SkillDef {
        id: SkillId::new(BONE_WALL_ID),
        name: "Bone Wall".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(BONE_ARMOR_ID)],
        synergies: Vec::new(),
        mana_cost_base: 17.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 1000,
        tree: POISON_BONE_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::Summon,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Poison Explosion skill definition.
///
/// - **Type**: Poison AoE — detonates a corpse releasing a toxic cloud
/// - **Mana**: 18 (flat)
/// - **Damage**: 20-30 at level 1, +7 per level
/// - **Cooldown**: 300 ms
/// - **Req Level**: 18 — requires Corpse Explosion (P&B variant)
pub fn poison_explosion() -> SkillDef {
    SkillDef {
        id: SkillId::new(POISON_EXPLOSION_ID),
        name: "Poison Explosion".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(PB_CORPSE_EXPLOSION_ID)],
        synergies: Vec::new(),
        mana_cost_base: 18.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 300,
        tree: POISON_BONE_TREE,
        damage_type: DamageType::Poison,
        kind: SkillKind::AreaOfEffect,
        base_damage_min: 20,
        base_damage_max: 30,
        damage_per_level: 7,
        synergy_ids: Vec::new(),
    }
}

/// Create the Bone Spear skill definition (Poison & Bone tree variant).
///
/// - **Type**: Projectile — hurls a piercing magic projectile through enemies
/// - **Mana**: 7 (flat)
/// - **Damage**: 15-17 at level 1, +5 per level
/// - **Cooldown**: 0 ms (spammable)
/// - **Req Level**: 18 — requires Bone Wall and Teeth
/// - **Synergies**: Receives +7% damage per level of Teeth
///
/// Note: this is the Poison & Bone tree variant (`PB_BONE_SPEAR_ID`).
/// The standalone legacy version lives in `necro.rs` (`BONE_SPEAR_ID`).
pub fn pb_bone_spear() -> SkillDef {
    SkillDef {
        id: SkillId::new(PB_BONE_SPEAR_ID),
        name: "Bone Spear".to_string(),
        max_level: 20,
        prerequisites: vec![
            SkillId::new(BONE_WALL_ID),
            SkillId::new(TEETH_ID),
        ],
        synergies: vec![SynergyDef {
            source_skill: SkillId::new(TEETH_ID),
            bonus_per_level: 0.07,
            stat: "damage".to_string(),
        }],
        mana_cost_base: 7.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 0,
        tree: POISON_BONE_TREE,
        damage_type: DamageType::Magic,
        kind: SkillKind::Projectile,
        base_damage_min: 15,
        base_damage_max: 17,
        damage_per_level: 5,
        synergy_ids: vec![TEETH_ID.to_string()],
    }
}

/// Create the Bone Prison skill definition.
///
/// - **Type**: Summon — raises a circular cage of bones around a target
/// - **Mana**: 27 (flat)
/// - **Cooldown**: 2000 ms
/// - **Req Level**: 24 — requires Bone Wall
/// - **HP**: 80 base, +15 per level (HP = 80 + 15*lvl, managed by the AI system)
/// - **Damage**: 0 (purely crowd-control utility)
pub fn bone_prison() -> SkillDef {
    SkillDef {
        id: SkillId::new(BONE_PRISON_ID),
        name: "Bone Prison".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(BONE_WALL_ID)],
        synergies: Vec::new(),
        mana_cost_base: 27.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 2000,
        tree: POISON_BONE_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::Summon,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Poison Nova skill definition.
///
/// - **Type**: Poison AoE — releases a ring of poison gas from the caster
/// - **Mana**: 20 (flat)
/// - **Damage**: 35-55 at level 1, +10 per level
/// - **Cooldown**: 1000 ms
/// - **Req Level**: 30 — requires Poison Explosion
/// - **Synergies**: Receives +8% damage per level of Poison Explosion
pub fn poison_nova() -> SkillDef {
    SkillDef {
        id: SkillId::new(POISON_NOVA_ID),
        name: "Poison Nova".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(POISON_EXPLOSION_ID)],
        synergies: vec![SynergyDef {
            source_skill: SkillId::new(POISON_EXPLOSION_ID),
            bonus_per_level: 0.08,
            stat: "damage".to_string(),
        }],
        mana_cost_base: 20.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 1000,
        tree: POISON_BONE_TREE,
        damage_type: DamageType::Poison,
        kind: SkillKind::AreaOfEffect,
        base_damage_min: 35,
        base_damage_max: 55,
        damage_per_level: 10,
        synergy_ids: vec![POISON_EXPLOSION_ID.to_string()],
    }
}

/// Create the Bone Spirit skill definition.
///
/// - **Type**: Projectile — launches a homing spirit of bone that seeks the nearest enemy
/// - **Mana**: 12 (flat)
/// - **Damage**: 20-30 at level 1, +8 per level
/// - **Cooldown**: 500 ms
/// - **Req Level**: 30 — requires Bone Spear (P&B variant)
/// - **Synergies**: Receives +5% damage per level of Bone Spear (P&B variant)
pub fn bone_spirit() -> SkillDef {
    SkillDef {
        id: SkillId::new(BONE_SPIRIT_ID),
        name: "Bone Spirit".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(PB_BONE_SPEAR_ID)],
        synergies: vec![SynergyDef {
            source_skill: SkillId::new(PB_BONE_SPEAR_ID),
            bonus_per_level: 0.05,
            stat: "damage".to_string(),
        }],
        mana_cost_base: 12.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 500,
        tree: POISON_BONE_TREE,
        damage_type: DamageType::Magic,
        kind: SkillKind::Projectile,
        base_damage_min: 20,
        base_damage_max: 30,
        damage_per_level: 8,
        synergy_ids: vec![PB_BONE_SPEAR_ID.to_string()],
    }
}

/// Register all ten Necromancer Poison & Bone skills into the given registry.
pub fn register_all(registry: &mut crate::SkillRegistry) {
    registry.register(teeth());
    registry.register(bone_armor());
    registry.register(poison_dagger());
    registry.register(pb_corpse_explosion());
    registry.register(bone_wall());
    registry.register(poison_explosion());
    registry.register(pb_bone_spear());
    registry.register(bone_prison());
    registry.register(poison_nova());
    registry.register(bone_spirit());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SkillRegistry;

    #[test]
    fn poison_bone_tree_10_skills() {
        let mut registry = SkillRegistry::new();
        register_all(&mut registry);
        assert!(
            registry.len() >= 10,
            "Expected at least 10 poison & bone skills, got {}",
            registry.len()
        );
    }

    #[test]
    fn bone_spear_synergy_teeth() {
        let spear = pb_bone_spear();
        let synergy = spear
            .synergies
            .iter()
            .find(|s| s.source_skill.as_str() == TEETH_ID);
        assert!(synergy.is_some(), "Bone Spear must have Teeth as a synergy source");
        let synergy = synergy.expect("synergy is Some per assertion above");
        assert!(
            (synergy.bonus_per_level - 0.07).abs() < f32::EPSILON,
            "Bone Spear synergy bonus_per_level must be 0.07, got {}",
            synergy.bonus_per_level
        );
    }
}
