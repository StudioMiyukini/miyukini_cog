// @id: MGE-ARPG-Synergy @do: synergy-calc @role: back-end @layer: 3 @human: miyuk
//! Synergy bonus calculator.

use crate::{SkillBook, SkillId, SkillRegistry};

/// Stateless calculator for synergy bonuses between skills.
///
/// In D2-style games, skills receive passive percentage bonuses from
/// other invested skills. This calculator aggregates those bonuses.
pub struct SynergyCalculator;

impl SynergyCalculator {
    /// Calculate the total synergy bonus for a specific stat on a skill.
    ///
    /// For each synergy defined on the target skill that matches `stat`,
    /// the bonus is `synergy.bonus_per_level * book.level_of(synergy.source_skill)`.
    ///
    /// Returns 0.0 if the skill has no synergies, or if no synergy matches
    /// the requested stat, or if the skill is not in the registry.
    pub fn calculate_bonus(
        skill_id: &SkillId,
        stat: &str,
        registry: &SkillRegistry,
        book: &SkillBook,
    ) -> f32 {
        let Some(def) = registry.get(skill_id) else {
            return 0.0;
        };

        def.synergies
            .iter()
            .filter(|syn| syn.stat == stat)
            .map(|syn| syn.bonus_per_level * book.level_of(&syn.source_skill) as f32)
            .sum()
    }

    /// Compute the total synergy bonus multiplier across **all** synergies of a skill.
    ///
    /// Unlike [`Self::calculate_bonus`], this method does not filter by stat — it
    /// accumulates every synergy defined on the skill.  This is the primary entry
    /// point when the caller wants the aggregate damage multiplier (e.g. 0.70 = +70%).
    ///
    /// # Arguments
    ///
    /// * `skill_id` — Target skill whose synergies are evaluated.
    /// * `book`     — Character skill book supplying hard-point levels.
    /// * `registry` — Registry used to look up the skill definition.
    ///
    /// Returns 0.0 if the skill is not found or has no synergies.
    pub fn compute(skill_id: &SkillId, book: &SkillBook, registry: &SkillRegistry) -> f32 {
        let Some(def) = registry.get(skill_id) else {
            return 0.0;
        };

        def.synergies
            .iter()
            .map(|syn| syn.bonus_per_level * book.level_of(&syn.source_skill) as f32)
            .sum()
    }

    /// Calculate the effective damage range after applying a synergy bonus.
    ///
    /// Formula:
    /// ```text
    /// scaled_min = base_min + damage_per_level * level
    /// scaled_max = base_max + damage_per_level * level
    /// effective_min = (scaled_min as f32 * (1.0 + synergy_bonus)) as i32
    /// effective_max = (scaled_max as f32 * (1.0 + synergy_bonus)) as i32
    /// ```
    ///
    /// The f32-to-i32 truncation is intentional (D2-style floor behaviour).
    ///
    /// # Arguments
    ///
    /// * `base_min`        — Minimum base damage at level 1.
    /// * `base_max`        — Maximum base damage at level 1.
    /// * `skill_level`     — Invested hard-point level of the skill.
    /// * `damage_per_level`— Flat damage added per level.
    /// * `synergy_bonus`   — Aggregate synergy multiplier (e.g. 0.70 for +70%).
    ///
    /// Returns `(effective_min, effective_max)`.
    #[allow(clippy::cast_possible_truncation)]
    pub fn effective_damage(
        base_min: i32,
        base_max: i32,
        skill_level: u32,
        damage_per_level: i32,
        synergy_bonus: f32,
    ) -> (i32, i32) {
        #[allow(clippy::cast_possible_wrap)]
        let level_bonus = damage_per_level * skill_level as i32;
        let scaled_min = base_min + level_bonus;
        let scaled_max = base_max + level_bonus;
        let multiplier = 1.0_f32 + synergy_bonus;
        let effective_min = (scaled_min as f32 * multiplier) as i32;
        let effective_max = (scaled_max as f32 * multiplier) as i32;
        (effective_min, effective_max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poison_bone;

    fn build_registry() -> SkillRegistry {
        let mut registry = SkillRegistry::new();
        poison_bone::register_all(&mut registry);
        registry
    }

    /// Bone Spear has a synergy from Teeth (+7% per level).
    /// With 10 hard points in Teeth: expected bonus = 0.07 * 10 = 0.70.
    #[test]
    fn synergy_bone_spear_teeth() {
        let registry = build_registry();
        let mut book = SkillBook::new(20);
        let teeth_id = SkillId::new(poison_bone::TEETH_ID);
        // Invest 10 points in Teeth (no prerequisites required).
        for _ in 0..10 {
            book.invest(&teeth_id, &registry).expect("invest should succeed");
        }

        let bone_spear_id = SkillId::new(poison_bone::PB_BONE_SPEAR_ID);
        let bonus = SynergyCalculator::compute(&bone_spear_id, &book, &registry);
        let expected = 0.07_f32 * 10.0_f32;
        assert!(
            (bonus - expected).abs() < f32::EPSILON,
            "Expected synergy bonus {expected}, got {bonus}",
        );
    }

    /// When the source skill has 0 invested points the bonus must be 0.0.
    #[test]
    fn synergy_zero_points() {
        let registry = build_registry();
        let book = SkillBook::new(0);
        let bone_spear_id = SkillId::new(poison_bone::PB_BONE_SPEAR_ID);
        let bonus = SynergyCalculator::compute(&bone_spear_id, &book, &registry);
        assert!(
            bonus.abs() < f32::EPSILON,
            "Expected 0.0 synergy bonus with no invested points, got {bonus}",
        );
    }

    /// Effective damage calculation:
    /// base 15-17, level 5, dmg/lvl 5, synergy 0.70.
    /// scaled = 15+25=40, 17+25=42.
    /// effective = (40 * 1.70) as i32 = 68, (42 * 1.70) as i32 = 71.
    #[test]
    fn effective_damage_with_synergy() {
        let (eff_min, eff_max) =
            SynergyCalculator::effective_damage(15, 17, 5, 5, 0.70);
        assert_eq!(eff_min, 68, "Expected effective_min = 68, got {eff_min}");
        assert_eq!(eff_max, 71, "Expected effective_max = 71, got {eff_max}");
    }
}
