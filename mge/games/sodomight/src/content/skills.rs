// @id: Sodomight-Content-Skills @do: act1-skill-definitions @role: back-end @layer: 4 @human: miyuk
//! Act 1 skill definitions using the engine's `SkillDef` type.
//!
//! This module provides the initial skill set available at game start, covering
//! basic attack, elemental spells (Sorceress-style), and melee skills
//! (Barbarian-style).
#![allow(clippy::too_many_lines)]

use mge_arpg_combat::DamageType;
use mge_arpg_skills::{SkillDef, SkillId, SkillKind};

// ---------------------------------------------------------------------------
// Skill definitions
// ---------------------------------------------------------------------------

/// Returns Act 1 skill definitions using the engine's `SkillDef` type.
///
/// These represent the initial skill set available at game start, covering
/// basic attack, elemental spells (Sorceress-style), and melee skills
/// (Barbarian-style).
#[must_use]
pub fn act1_skills() -> Vec<SkillDef> {
    vec![
        SkillDef {
            id: SkillId::new("normal_attack"),
            name: "Normal Attack".into(),
            max_level: 1,
            prerequisites: vec![],
            synergies: vec![],
            mana_cost_base: 0.0,
            mana_cost_per_level: 0.0,
            cooldown_ms: 0,
            tree: 0,
            damage_type: DamageType::Physical,
            kind: SkillKind::Projectile,
            base_damage_min: 0,
            base_damage_max: 0,
            damage_per_level: 0,
            synergy_ids: vec![],
        },
        SkillDef {
            id: SkillId::new("fire_bolt"),
            name: "Fire Bolt".into(),
            max_level: 20,
            prerequisites: vec![],
            synergies: vec![],
            mana_cost_base: 4.0,
            mana_cost_per_level: 0.5,
            cooldown_ms: 0,
            tree: 0,
            damage_type: DamageType::Fire,
            kind: SkillKind::Projectile,
            base_damage_min: 3,
            base_damage_max: 6,
            damage_per_level: 2,
            synergy_ids: vec![],
        },
        SkillDef {
            id: SkillId::new("ice_bolt"),
            name: "Ice Bolt".into(),
            max_level: 20,
            prerequisites: vec![],
            synergies: vec![],
            mana_cost_base: 4.0,
            mana_cost_per_level: 0.5,
            cooldown_ms: 0,
            tree: 1,
            damage_type: DamageType::Cold,
            kind: SkillKind::Projectile,
            base_damage_min: 3,
            base_damage_max: 5,
            damage_per_level: 2,
            synergy_ids: vec![],
        },
        SkillDef {
            id: SkillId::new("charged_bolt"),
            name: "Charged Bolt".into(),
            max_level: 20,
            prerequisites: vec![],
            synergies: vec![],
            mana_cost_base: 7.0,
            mana_cost_per_level: 0.5,
            cooldown_ms: 0,
            tree: 2,
            damage_type: DamageType::Lightning,
            kind: SkillKind::Projectile,
            base_damage_min: 1,
            base_damage_max: 8,
            damage_per_level: 1,
            synergy_ids: vec![],
        },
        SkillDef {
            id: SkillId::new("bash"),
            name: "Bash".into(),
            max_level: 20,
            prerequisites: vec![],
            synergies: vec![],
            mana_cost_base: 2.0,
            mana_cost_per_level: 0.0,
            cooldown_ms: 0,
            tree: 0,
            damage_type: DamageType::Physical,
            kind: SkillKind::Projectile,
            base_damage_min: 0,
            base_damage_max: 0,
            damage_per_level: 3,
            synergy_ids: vec![],
        },
        SkillDef {
            id: SkillId::new("double_swing"),
            name: "Double Swing".into(),
            max_level: 20,
            prerequisites: vec![SkillId::new("bash")],
            synergies: vec![],
            mana_cost_base: 3.0,
            mana_cost_per_level: 0.0,
            cooldown_ms: 0,
            tree: 0,
            damage_type: DamageType::Physical,
            kind: SkillKind::Projectile,
            base_damage_min: 0,
            base_damage_max: 0,
            damage_per_level: 2,
            synergy_ids: vec![],
        },
    ]
}

// ---------------------------------------------------------------------------
// Lookup helpers
// ---------------------------------------------------------------------------

/// Find a skill definition by its id.
#[must_use]
pub fn find_skill(id: &str) -> Option<SkillDef> {
    act1_skills()
        .into_iter()
        .find(|s| s.id.as_str() == id)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_act1_skills_count() {
        let skills = act1_skills();
        assert_eq!(skills.len(), 6);
    }

    #[test]
    fn test_fire_bolt_mana_cost() {
        let fb = find_skill("fire_bolt").expect("fire_bolt must exist");
        assert!((fb.mana_cost_base - 4.0).abs() < f32::EPSILON);
        assert_eq!(fb.cooldown_ms, 0);
    }

    #[test]
    fn test_normal_attack_no_cost() {
        let atk = find_skill("normal_attack").expect("normal_attack must exist");
        assert!((atk.mana_cost_base - 0.0).abs() < f32::EPSILON);
        assert_eq!(atk.max_level, 1);
    }

    #[test]
    fn test_double_swing_requires_bash() {
        let ds = find_skill("double_swing").expect("double_swing must exist");
        assert_eq!(ds.prerequisites.len(), 1);
        assert_eq!(ds.prerequisites[0].as_str(), "bash");
    }
}
