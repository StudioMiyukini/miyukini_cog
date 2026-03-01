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
}
