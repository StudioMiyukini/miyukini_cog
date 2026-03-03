// @id: MGE-ARPG-Activation @do: skill-activation @role: back-end @layer: 3 @human: miyuk
//! Skill activation checks combining mana, cooldown, and investment level.

use crate::{SkillBook, SkillCooldownTracker, SkillId, SkillRegistry};

/// Check whether a skill can be used right now.
///
/// A skill is usable when **all** of the following are true:
/// 1. The skill exists in the registry.
/// 2. The character has learned the skill (level >= 1 in the book).
/// 3. The character has enough mana to pay the cost at the current level.
/// 4. The skill's cooldown has elapsed (or has no cooldown).
///
/// # Arguments
/// * `skill_id` — The skill to check.
/// * `book` — The character's skill investment tracker.
/// * `current_mana` — The character's current mana pool.
/// * `cooldowns` — The cooldown tracker for all skills.
/// * `registry` — The authoritative skill definition source.
pub fn can_use_skill(
    skill_id: &SkillId,
    book: &SkillBook,
    current_mana: f32,
    cooldowns: &SkillCooldownTracker,
    registry: &SkillRegistry,
) -> bool {
    // 1. Skill must exist in the registry.
    let Some(def) = registry.get(skill_id) else {
        return false;
    };

    // 2. Character must have learned the skill.
    let level = book.level_of(skill_id);
    if level == 0 {
        return false;
    }

    // 3. Enough mana for the cost at current level.
    let cost = def.mana_cost(level);
    if current_mana < cost {
        return false;
    }

    // 4. Cooldown must have elapsed.
    cooldowns.is_ready(skill_id)
}
