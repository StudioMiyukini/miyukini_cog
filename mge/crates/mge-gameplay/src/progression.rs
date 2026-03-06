use serde::{Deserialize, Serialize};

use crate::character::Class;
use crate::stats::{CharacterStats, Modifier, StatKind, StatModifier};

/// Max level cap.
pub const MAX_LEVEL: u32 = 99;

/// XP required to go from level `n` to `n+1`.
/// Uses a D2-like exponential curve.
pub fn xp_for_level(level: u32) -> u64 {
    if level == 0 || level >= MAX_LEVEL {
        return u64::MAX;
    }
    // Approximation: 500 * level^2 * (level as f64 * 0.02 + 1.0)
    let l = u64::from(level);
    500 * l * l + 10 * l * l * l / 10
}

/// Total XP needed to reach a given level from 0.
pub fn total_xp_for_level(level: u32) -> u64 {
    (1..level).map(xp_for_level).sum()
}

/// Stat points gained per level.
pub const STAT_POINTS_PER_LEVEL: u32 = 5;

/// Skill points gained per level.
pub const SKILL_POINTS_PER_LEVEL: u32 = 1;

/// Point allocation record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationHistory {
    /// Stat kind -> total points spent.
    pub stat_spent: Vec<(StatKind, u32)>,
}

impl AllocationHistory {
    pub fn spent_in(&self, stat: StatKind) -> u32 {
        self.stat_spent.iter().find(|(s, _)| *s == stat).map_or(0, |(_, n)| *n)
    }
}

/// The progression state of a character.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterProgression {
    pub class: Class,
    pub level: u32,
    pub experience: u64,
    pub stat_points: u32,
    pub skill_points: u32,
    pub allocation: AllocationHistory,
}

impl CharacterProgression {
    pub fn new(class: Class) -> Self {
        Self {
            class,
            level: 1,
            experience: 0,
            stat_points: 0,
            skill_points: 0,
            allocation: AllocationHistory { stat_spent: Vec::new() },
        }
    }

    /// XP needed for the next level.
    pub fn xp_to_next(&self) -> u64 {
        xp_for_level(self.level)
    }

    /// XP progress fraction for the current level.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    pub fn xp_fraction(&self) -> f32 {
        let needed = self.xp_to_next();
        if needed == u64::MAX || needed == 0 {
            return 1.0;
        }
        (self.experience as f64 / needed as f64).min(1.0) as f32
    }

    /// Grant XP. Returns how many levels were gained.
    pub fn grant_xp(&mut self, amount: u64) -> u32 {
        let mut gained = 0u32;
        self.experience += amount;
        while self.level < MAX_LEVEL {
            let needed = xp_for_level(self.level);
            if self.experience >= needed {
                self.experience -= needed;
                self.level += 1;
                self.stat_points += STAT_POINTS_PER_LEVEL;
                self.skill_points += SKILL_POINTS_PER_LEVEL;
                gained += 1;
            } else {
                break;
            }
        }
        gained
    }

    /// Spend a stat point on a primary attribute. Returns false if no points left.
    pub fn spend_stat_point(&mut self, stat: StatKind, char_stats: &mut CharacterStats) -> bool {
        if self.stat_points == 0 {
            return false;
        }
        let entry = self.allocation.stat_spent.iter_mut().find(|(s, _)| *s == stat);
        if let Some((_, n)) = entry {
            *n += 1;
        } else {
            self.allocation.stat_spent.push((stat, 1));
        }

        // Apply modifier from this spent point (flat +1 to primary attr)
        let modifier_id = 0x8000_0000 | stat as u64;
        char_stats.remove_source(modifier_id);
        let total_spent = self.allocation.spent_in(stat);
        char_stats.add_modifier(StatModifier {
            source_id: modifier_id,
            stat,
            modifier: Modifier::Flat(total_spent.cast_signed()),
        });

        self.stat_points -= 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::StatKind;

    #[test]
    fn xp_curve_increases_with_level() {
        assert!(xp_for_level(2) > xp_for_level(1));
        assert!(xp_for_level(50) > xp_for_level(10));
    }

    #[test]
    fn level_up_on_xp_grant() {
        let mut prog = CharacterProgression::new(Class::Warrior);
        let needed = prog.xp_to_next();
        let gained = prog.grant_xp(needed);
        assert_eq!(gained, 1);
        assert_eq!(prog.level, 2);
        assert_eq!(prog.stat_points, STAT_POINTS_PER_LEVEL);
        assert_eq!(prog.skill_points, SKILL_POINTS_PER_LEVEL);
    }

    #[test]
    fn multiple_level_ups_in_one_grant() {
        let mut prog = CharacterProgression::new(Class::Warrior);
        let needed: u64 = (1..=5).map(xp_for_level).sum();
        let gained = prog.grant_xp(needed);
        assert_eq!(gained, 5);
        assert_eq!(prog.level, 6);
    }

    #[test]
    fn spend_stat_point_decrements_pool() {
        let mut prog = CharacterProgression::new(Class::Warrior);
        prog.stat_points = 3;
        let mut stats = CharacterStats::new(Class::Warrior, 1);
        assert!(prog.spend_stat_point(StatKind::Strength, &mut stats));
        assert_eq!(prog.stat_points, 2);
        assert_eq!(prog.allocation.spent_in(StatKind::Strength), 1);
    }

    #[test]
    fn spend_stat_point_fails_with_no_points() {
        let mut prog = CharacterProgression::new(Class::Warrior);
        let mut stats = CharacterStats::new(Class::Warrior, 1);
        assert!(!prog.spend_stat_point(StatKind::Strength, &mut stats));
    }

    #[test]
    fn max_level_capped() {
        let mut prog = CharacterProgression::new(Class::Warrior);
        prog.level = MAX_LEVEL - 1;
        prog.grant_xp(u64::MAX / 2);
        assert_eq!(prog.level, MAX_LEVEL);
    }
}
