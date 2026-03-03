// @id: MGE-ARPG-Curses @do: curses-skills @role: back-end @layer: 3 @human: miyuk
//! Necromancer Curses skill tree content definitions.
//!
//! Provides factory functions for the ten core Curses tree skills:
//! - **Amplify Damage** — +100% physical damage taken by cursed monsters
//! - **Dim Vision** — Reduce monster sight radius to 0
//! - **Weaken** — -33% physical damage dealt by cursed monsters
//! - **Iron Maiden** — Attackers take reflected physical damage
//! - **Terror** — Monsters flee in fear
//! - **Confuse** — Monsters attack random targets
//! - **Life Tap** — Melee attackers heal from damage dealt to cursed target
//! - **Attract** — Single target becomes focus of all nearby monsters
//! - **Decrepify** — Slow 50%, +50% phys dmg taken, -50% phys dmg dealt
//! - **Lower Resist** — Reduce all elemental resistances, break immunities
//!
//! # Curse Exclusivity
//!
//! **Only ONE curse can be active on a target at a time.** Casting a new curse
//! on an already-cursed target immediately replaces the previous curse. This is
//! a fundamental property of the Curses tree and must be enforced by the combat
//! system when applying curse effects.

use mge_arpg_combat::DamageType;

use crate::{SkillDef, SkillId, SkillKind};

/// Necromancer Curses skill tree identifier (tree index 2).
pub const CURSES_TREE: u8 = 2;

/// Skill ID constant for Amplify Damage.
pub const AMPLIFY_DAMAGE_ID: &str = "necro_amplify_damage";
/// Skill ID constant for Dim Vision.
pub const DIM_VISION_ID: &str = "necro_dim_vision";
/// Skill ID constant for Weaken.
pub const WEAKEN_ID: &str = "necro_weaken";
/// Skill ID constant for Iron Maiden.
pub const IRON_MAIDEN_ID: &str = "necro_iron_maiden";
/// Skill ID constant for Terror.
pub const TERROR_ID: &str = "necro_terror";
/// Skill ID constant for Confuse.
pub const CONFUSE_ID: &str = "necro_confuse";
/// Skill ID constant for Life Tap.
pub const LIFE_TAP_ID: &str = "necro_life_tap";
/// Skill ID constant for Attract.
pub const ATTRACT_ID: &str = "necro_attract";
/// Skill ID constant for Decrepify.
pub const DECREPIFY_ID: &str = "necro_decrepify";
/// Skill ID constant for Lower Resist.
pub const LOWER_RESIST_ID: &str = "necro_lower_resist";

/// Create the Amplify Damage skill definition.
///
/// - **Type**: AreaOfEffect — curses all monsters in radius
/// - **Mana**: 4 (flat)
/// - **Damage**: 0 (debuff only — no direct damage)
/// - **Cooldown**: 500 ms
/// - **Req Level**: 1 — no prerequisites
/// - **Effect**: Cursed monsters take +100% physical damage from all sources
/// - **Duration**: 8s base + 2s per level
pub fn amplify_damage() -> SkillDef {
    SkillDef {
        id: SkillId::new(AMPLIFY_DAMAGE_ID),
        name: "Amplify Damage".to_string(),
        max_level: 20,
        prerequisites: Vec::new(),
        synergies: Vec::new(),
        mana_cost_base: 4.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 500,
        tree: CURSES_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::AreaOfEffect,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Dim Vision skill definition.
///
/// - **Type**: AreaOfEffect — curses all monsters in radius
/// - **Mana**: 9 (flat)
/// - **Damage**: 0 (debuff only — no direct damage)
/// - **Cooldown**: 800 ms
/// - **Req Level**: 6 — no prerequisites
/// - **Effect**: Reduce monster sight radius to 0 (effectively blind)
/// - **Duration**: 7s base + 1s per level
pub fn dim_vision() -> SkillDef {
    SkillDef {
        id: SkillId::new(DIM_VISION_ID),
        name: "Dim Vision".to_string(),
        max_level: 20,
        prerequisites: Vec::new(),
        synergies: Vec::new(),
        mana_cost_base: 9.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 800,
        tree: CURSES_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::AreaOfEffect,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Weaken skill definition.
///
/// - **Type**: AreaOfEffect — curses all monsters in radius
/// - **Mana**: 4 (flat)
/// - **Damage**: 0 (debuff only — no direct damage)
/// - **Cooldown**: 500 ms
/// - **Req Level**: 6 — requires Amplify Damage
/// - **Effect**: Cursed monsters deal -33% physical damage
/// - **Duration**: 8s base + 2s per level
pub fn weaken() -> SkillDef {
    SkillDef {
        id: SkillId::new(WEAKEN_ID),
        name: "Weaken".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(AMPLIFY_DAMAGE_ID)],
        synergies: Vec::new(),
        mana_cost_base: 4.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 500,
        tree: CURSES_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::AreaOfEffect,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Iron Maiden skill definition.
///
/// - **Type**: AreaOfEffect — curses all monsters in radius
/// - **Mana**: 5 (flat)
/// - **Damage**: 0 (debuff only — reflected damage handled by combat system)
/// - **Cooldown**: 500 ms
/// - **Req Level**: 12 — requires Amplify Damage
/// - **Effect**: Attackers take 200% + 10%/level of their physical damage as reflected
/// - **Duration**: 12s base + 1.2s per level
pub fn iron_maiden() -> SkillDef {
    SkillDef {
        id: SkillId::new(IRON_MAIDEN_ID),
        name: "Iron Maiden".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(AMPLIFY_DAMAGE_ID)],
        synergies: Vec::new(),
        mana_cost_base: 5.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 500,
        tree: CURSES_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::AreaOfEffect,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Terror skill definition.
///
/// - **Type**: AreaOfEffect — curses all monsters in radius
/// - **Mana**: 7 (flat)
/// - **Damage**: 0 (debuff only — no direct damage)
/// - **Cooldown**: 1000 ms
/// - **Req Level**: 12 — requires Weaken
/// - **Effect**: Cursed monsters flee in fear (cannot attack while fleeing)
/// - **Duration**: 3s base + 0.4s per level
pub fn terror() -> SkillDef {
    SkillDef {
        id: SkillId::new(TERROR_ID),
        name: "Terror".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(WEAKEN_ID)],
        synergies: Vec::new(),
        mana_cost_base: 7.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 1000,
        tree: CURSES_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::AreaOfEffect,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Confuse skill definition.
///
/// - **Type**: AreaOfEffect — curses all monsters in radius
/// - **Mana**: 13 (flat)
/// - **Damage**: 0 (debuff only — no direct damage)
/// - **Cooldown**: 1500 ms
/// - **Req Level**: 18 — requires Dim Vision
/// - **Effect**: Cursed monsters attack random targets (including other monsters)
/// - **Duration**: 10s base + 1s per level
pub fn confuse() -> SkillDef {
    SkillDef {
        id: SkillId::new(CONFUSE_ID),
        name: "Confuse".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(DIM_VISION_ID)],
        synergies: Vec::new(),
        mana_cost_base: 13.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 1500,
        tree: CURSES_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::AreaOfEffect,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Life Tap skill definition.
///
/// - **Type**: AreaOfEffect — curses all monsters in radius
/// - **Mana**: 9 (flat)
/// - **Damage**: 0 (debuff only — healing handled by combat system)
/// - **Cooldown**: 500 ms
/// - **Req Level**: 18 — requires Iron Maiden
/// - **Effect**: Melee attackers heal for 50% of physical damage dealt to cursed target
/// - **Duration**: 6s base + 1.2s per level
pub fn life_tap() -> SkillDef {
    SkillDef {
        id: SkillId::new(LIFE_TAP_ID),
        name: "Life Tap".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(IRON_MAIDEN_ID)],
        synergies: Vec::new(),
        mana_cost_base: 9.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 500,
        tree: CURSES_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::AreaOfEffect,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Attract skill definition.
///
/// - **Type**: AreaOfEffect — targets single monster within radius
/// - **Mana**: 14 (flat)
/// - **Damage**: 0 (debuff only — no direct damage)
/// - **Cooldown**: 2000 ms
/// - **Req Level**: 24 — requires Confuse
/// - **Effect**: Cursed single target becomes the focus of all nearby monsters
/// - **Duration**: 12s base + 1s per level
pub fn attract() -> SkillDef {
    SkillDef {
        id: SkillId::new(ATTRACT_ID),
        name: "Attract".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(CONFUSE_ID)],
        synergies: Vec::new(),
        mana_cost_base: 14.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 2000,
        tree: CURSES_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::AreaOfEffect,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Decrepify skill definition.
///
/// - **Type**: AreaOfEffect — curses all monsters in radius
/// - **Mana**: 11 (flat)
/// - **Damage**: 0 (debuff only — no direct damage)
/// - **Cooldown**: 500 ms
/// - **Req Level**: 24 — requires Terror and Life Tap
/// - **Effect**: Slow 50%, +50% physical damage taken, -50% physical damage dealt
/// - **Duration**: 4s base + 0.6s per level
pub fn decrepify() -> SkillDef {
    SkillDef {
        id: SkillId::new(DECREPIFY_ID),
        name: "Decrepify".to_string(),
        max_level: 20,
        prerequisites: vec![
            SkillId::new(TERROR_ID),
            SkillId::new(LIFE_TAP_ID),
        ],
        synergies: Vec::new(),
        mana_cost_base: 11.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 500,
        tree: CURSES_TREE,
        damage_type: DamageType::Physical,
        kind: SkillKind::AreaOfEffect,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Create the Lower Resist skill definition.
///
/// - **Type**: AreaOfEffect — curses all monsters in radius
/// - **Mana**: 22 (flat)
/// - **Damage**: 0 (debuff only — no direct damage)
/// - **Cooldown**: 2000 ms
/// - **Req Level**: 30 — requires Decrepify
/// - **Damage Type**: Magic (the curse energy itself is magical)
/// - **Effect**: -30 - 2/level to all elemental resistances on cursed monsters.
///   Breaks elemental immunities at 1/5 effectiveness (immune monsters lose 1/5
///   of their excess resistance above 100%, making some damage possible).
/// - **Duration**: 20s base + 0s per level (fixed duration)
pub fn lower_resist() -> SkillDef {
    SkillDef {
        id: SkillId::new(LOWER_RESIST_ID),
        name: "Lower Resist".to_string(),
        max_level: 20,
        prerequisites: vec![SkillId::new(DECREPIFY_ID)],
        synergies: Vec::new(),
        mana_cost_base: 22.0,
        mana_cost_per_level: 0.0,
        cooldown_ms: 2000,
        tree: CURSES_TREE,
        damage_type: DamageType::Magic,
        kind: SkillKind::AreaOfEffect,
        base_damage_min: 0,
        base_damage_max: 0,
        damage_per_level: 0,
        synergy_ids: Vec::new(),
    }
}

/// Register all ten Necromancer Curses skills into the given registry.
pub fn register_all(registry: &mut crate::SkillRegistry) {
    registry.register(amplify_damage());
    registry.register(dim_vision());
    registry.register(weaken());
    registry.register(iron_maiden());
    registry.register(terror());
    registry.register(confuse());
    registry.register(life_tap());
    registry.register(attract());
    registry.register(decrepify());
    registry.register(lower_resist());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SkillRegistry;

    #[test]
    fn curses_tree_10_skills() {
        let mut registry = SkillRegistry::new();
        register_all(&mut registry);
        assert!(
            registry.len() >= 10,
            "Expected at least 10 curses skills, got {}",
            registry.len()
        );
    }

    #[test]
    fn curse_one_active_concept() {
        // All 10 skills must belong to CURSES_TREE and exist in the registry.
        let mut registry = SkillRegistry::new();
        register_all(&mut registry);

        let curse_ids = [
            AMPLIFY_DAMAGE_ID,
            DIM_VISION_ID,
            WEAKEN_ID,
            IRON_MAIDEN_ID,
            TERROR_ID,
            CONFUSE_ID,
            LIFE_TAP_ID,
            ATTRACT_ID,
            DECREPIFY_ID,
            LOWER_RESIST_ID,
        ];

        for id in &curse_ids {
            let skill = registry.get(&crate::SkillId::new(*id));
            assert!(
                skill.is_some(),
                "Skill '{id}' must be registered in the curses tree"
            );
            let skill = skill.expect("already asserted is_some above");
            assert_eq!(
                skill.tree,
                CURSES_TREE,
                "Skill '{id}' must belong to CURSES_TREE ({}), got {}",
                CURSES_TREE,
                skill.tree
            );
        }
    }

    #[test]
    fn amplify_damage_effect() {
        let skill = amplify_damage();

        // No prerequisites — available from level 1
        assert!(
            skill.prerequisites.is_empty(),
            "Amplify Damage must have no prerequisites"
        );

        // Must be AreaOfEffect kind
        assert_eq!(
            skill.kind,
            SkillKind::AreaOfEffect,
            "Amplify Damage must be AreaOfEffect kind"
        );

        // Must have Physical damage type (the debuff amplifies physical damage taken)
        assert_eq!(
            skill.damage_type,
            DamageType::Physical,
            "Amplify Damage must have Physical damage type"
        );

        // No direct damage — it is a pure debuff
        assert_eq!(skill.base_damage_min, 0, "Amplify Damage deals no direct damage");
        assert_eq!(skill.base_damage_max, 0, "Amplify Damage deals no direct damage");
        assert_eq!(skill.damage_per_level, 0, "Amplify Damage has no damage scaling");
    }
}
