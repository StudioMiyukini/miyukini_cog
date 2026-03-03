// @id: MGE-ARPG-Summoning @do: summoning-skills @role: back-end @layer: 3 @human: miyuk
//! Necromancer Summoning skill tree content definitions.
//!
//! Provides factory functions for the ten core Summoning tree skills:
//! - **Raise Skeleton** — Basic skeleton summon (count scales with level)
//! - **Skeleton Mastery** — Passive: +2 HP per level to skeletons
//! - **Clay Golem** — Slow melee golem summon
//! - **Golem Mastery** — Passive: +50 HP per level to golems
//! - **Raise Skeletal Mage** — Magic-attack skeleton variant
//! - **Blood Golem** — Life-link golem (shares damage with caster)
//! - **Summon Resist** — Passive: +5% all resistances per level to summons
//! - **Iron Golem** — Created from an item on the ground
//! - **Fire Golem** — Fire-damage golem with holy fire aura
//! - **Revive** — Temporarily animate a monster corpse as an ally

use mge_arpg_combat::DamageType;

use crate::{SkillDef, SkillId, SkillKind};

/// Necromancer Summoning skill tree identifier (tree index 0).
pub const SUMMONING_TREE: u8 = 0;

/// Skill ID constant for Raise Skeleton.
pub const RAISE_SKELETON_ID: &str = "necro_raise_skeleton";
/// Skill ID constant for Skeleton Mastery.
pub const SKELETON_MASTERY_ID: &str = "necro_skeleton_mastery";
/// Skill ID constant for Clay Golem.
pub const CLAY_GOLEM_ID: &str = "necro_clay_golem";
/// Skill ID constant for Golem Mastery.
pub const GOLEM_MASTERY_ID: &str = "necro_golem_mastery";
/// Skill ID constant for Raise Skeletal Mage.
pub const RAISE_SKELETAL_MAGE_ID: &str = "necro_raise_skeletal_mage";
/// Skill ID constant for Blood Golem.
pub const BLOOD_GOLEM_ID: &str = "necro_blood_golem";
/// Skill ID constant for Summon Resist.
pub const SUMMON_RESIST_ID: &str = "necro_summon_resist";
/// Skill ID constant for Iron Golem.
pub const IRON_GOLEM_ID: &str = "necro_iron_golem";
/// Skill ID constant for Fire Golem.
pub const FIRE_GOLEM_ID: &str = "necro_fire_golem";
/// Skill ID constant for Revive.
pub const REVIVE_ID: &str = "necro_revive";

/// Create the Raise Skeleton skill definition.
///
/// - **Type**: Summon (no direct damage — skeletons deal their own damage)
/// - **Mana**: 6 (flat)
/// - **Damage**: 0 (summons deal their own damage independently)
/// - **Cooldown**: 500 ms
/// - **Req Level**: 1 — no prerequisites
/// - **Summon count**: scales with level and Skeleton Mastery synergy
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
        tree: SUMMONING_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::Summon,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Skeleton Mastery skill definition.
///
/// - **Type**: Passive — enhances all skeleton summons
/// - **Mana**: 0 (passive, no cost)
/// - **Cooldown**: 0 ms
/// - **Req Level**: 1 — no prerequisites
/// - **Bonus**: +2 HP per level to all active skeletons (applied by the AI system)
pub fn skeleton_mastery() -> SkillDef {
    SkillDef {
        id: SkillId::new(SKELETON_MASTERY_ID),
        name: "Skeleton Mastery".to_string(),
        max_level: 20,
        prerequisites: Vec::new(),
        synergies: Vec::new(),
        mana_cost_base: 0.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 0,
        tree: SUMMONING_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::Passive,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Clay Golem skill definition.
///
/// - **Type**: Summon — slow melee golem
/// - **Mana**: 15 (flat)
/// - **Damage**: 0 (golem deals its own damage independently)
/// - **Cooldown**: 2000 ms
/// - **Req Level**: 6 — requires Skeleton Mastery
pub fn clay_golem() -> SkillDef {
    SkillDef {
        id: SkillId::new(CLAY_GOLEM_ID),
        name: "Clay Golem".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(SKELETON_MASTERY_ID)],
        synergies: Vec::new(),
        mana_cost_base: 15.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 2000,
        tree: SUMMONING_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::Summon,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Golem Mastery skill definition.
///
/// - **Type**: Passive — enhances all golem summons
/// - **Mana**: 0 (passive, no cost)
/// - **Cooldown**: 0 ms
/// - **Req Level**: 12 — requires Skeleton Mastery
/// - **Bonus**: +50 HP per level to all active golems (applied by the AI system)
pub fn golem_mastery() -> SkillDef {
    SkillDef {
        id: SkillId::new(GOLEM_MASTERY_ID),
        name: "Golem Mastery".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(SKELETON_MASTERY_ID)],
        synergies: Vec::new(),
        mana_cost_base: 0.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 0,
        tree: SUMMONING_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::Passive,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Raise Skeletal Mage skill definition.
///
/// - **Type**: Summon — magic-attack skeleton variant
/// - **Mana**: 10 (flat)
/// - **Damage**: 0 (mages deal their own damage independently)
/// - **Cooldown**: 800 ms
/// - **Req Level**: 12 — requires Raise Skeleton
pub fn raise_skeletal_mage() -> SkillDef {
    SkillDef {
        id: SkillId::new(RAISE_SKELETAL_MAGE_ID),
        name: "Raise Skeletal Mage".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(RAISE_SKELETON_ID)],
        synergies: Vec::new(),
        mana_cost_base: 10.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 800,
        tree: SUMMONING_TREE,
        damage_type: DamageType::Magic,
        kind: SkillKind::Summon,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Blood Golem skill definition.
///
/// - **Type**: Summon — life-link golem (shares damage received with caster)
/// - **Mana**: 20 (flat)
/// - **Damage**: 0 (golem deals its own damage independently)
/// - **Cooldown**: 3000 ms
/// - **Req Level**: 18 — requires Clay Golem and Golem Mastery
pub fn blood_golem() -> SkillDef {
    SkillDef {
        id: SkillId::new(BLOOD_GOLEM_ID),
        name: "Blood Golem".to_string(),
        max_level: 20,
        prerequisites: vec![
            SkillId::new(CLAY_GOLEM_ID),
            SkillId::new(GOLEM_MASTERY_ID),
        ],
        synergies: Vec::new(),
        mana_cost_base: 20.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 3000,
        tree: SUMMONING_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::Summon,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Summon Resist skill definition.
///
/// - **Type**: Passive — increases all resistances for all summons
/// - **Mana**: 0 (passive, no cost)
/// - **Cooldown**: 0 ms
/// - **Req Level**: 24 — requires Skeleton Mastery
/// - **Bonus**: +5% to all elemental resistances per level for all active summons
pub fn summon_resist() -> SkillDef {
    SkillDef {
        id: SkillId::new(SUMMON_RESIST_ID),
        name: "Summon Resist".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(SKELETON_MASTERY_ID)],
        synergies: Vec::new(),
        mana_cost_base: 0.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 0,
        tree: SUMMONING_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::Passive,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Iron Golem skill definition.
///
/// - **Type**: Summon — created from an item on the ground (consumes the item)
/// - **Mana**: 35 (flat)
/// - **Damage**: 0 (golem deals its own damage independently)
/// - **Cooldown**: 5000 ms
/// - **Req Level**: 24 — requires Blood Golem
pub fn iron_golem() -> SkillDef {
    SkillDef {
        id: SkillId::new(IRON_GOLEM_ID),
        name: "Iron Golem".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(BLOOD_GOLEM_ID)],
        synergies: Vec::new(),
        mana_cost_base: 35.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 5000,
        tree: SUMMONING_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::Summon,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Fire Golem skill definition.
///
/// - **Type**: Summon — fire-damage golem with holy fire aura
/// - **Mana**: 40 (flat)
/// - **Damage**: 0 (golem deals its own damage independently)
/// - **Cooldown**: 5000 ms
/// - **Req Level**: 30 — requires Iron Golem
pub fn fire_golem() -> SkillDef {
    SkillDef {
        id: SkillId::new(FIRE_GOLEM_ID),
        name: "Fire Golem".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(IRON_GOLEM_ID)],
        synergies: Vec::new(),
        mana_cost_base: 40.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 5000,
        tree: SUMMONING_TREE,
        damage_type: DamageType::Fire,
        kind: SkillKind::Summon,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Revive skill definition.
///
/// - **Type**: Summon — temporarily animates a monster corpse as an ally
/// - **Mana**: 45 (flat)
/// - **Damage**: 0 (revived monster uses its original stats)
/// - **Cooldown**: 1000 ms
/// - **Req Level**: 30 — requires Raise Skeletal Mage
/// - **Duration**: `(level * 3) + 180` seconds (scales with level)
pub fn revive() -> SkillDef {
    SkillDef {
        id: SkillId::new(REVIVE_ID),
        name: "Revive".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(RAISE_SKELETAL_MAGE_ID)],
        synergies: Vec::new(),
        mana_cost_base: 45.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 1000,
        tree: SUMMONING_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::Summon,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Register all ten Necromancer Summoning skills into the given registry.
pub fn register_all(registry: &mut crate::SkillRegistry) {
    registry.register(raise_skeleton());
    registry.register(skeleton_mastery());
    registry.register(clay_golem());
    registry.register(golem_mastery());
    registry.register(raise_skeletal_mage());
    registry.register(blood_golem());
    registry.register(summon_resist());
    registry.register(iron_golem());
    registry.register(fire_golem());
    registry.register(revive());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SkillRegistry;

    #[test]
    fn summoning_tree_10_skills() {
        let mut registry = SkillRegistry::new();
        register_all(&mut registry);
        assert!(registry.len() >= 10, "Expected at least 10 summoning skills, got {}", registry.len());
    }

    #[test]
    fn summoning_prereq_chain() {
        // Fire Golem -> Iron Golem -> Blood Golem -> Clay Golem + Golem Mastery
        let fire = fire_golem();
        assert!(
            fire.prerequisites.iter().any(|p| p.as_str() == IRON_GOLEM_ID),
            "Fire Golem must require Iron Golem"
        );

        let iron = iron_golem();
        assert!(
            iron.prerequisites.iter().any(|p| p.as_str() == BLOOD_GOLEM_ID),
            "Iron Golem must require Blood Golem"
        );

        let blood = blood_golem();
        assert!(
            blood.prerequisites.iter().any(|p| p.as_str() == CLAY_GOLEM_ID),
            "Blood Golem must require Clay Golem"
        );
        assert!(
            blood.prerequisites.iter().any(|p| p.as_str() == GOLEM_MASTERY_ID),
            "Blood Golem must require Golem Mastery"
        );

        let clay = clay_golem();
        assert!(
            clay.prerequisites.iter().any(|p| p.as_str() == SKELETON_MASTERY_ID),
            "Clay Golem must require Skeleton Mastery"
        );

        let golem_mastery = golem_mastery();
        assert!(
            golem_mastery.prerequisites.iter().any(|p| p.as_str() == SKELETON_MASTERY_ID),
            "Golem Mastery must require Skeleton Mastery"
        );
    }
}
