// @id: MGE-ARPG-Stats-Level @do: level-xp @role: back-end @layer: 3 @human: miyuk
//! Character level, experience, and stat/skill point tracking.

/// Experience-point thresholds for each level (1..=99).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExpTable {
    /// Cumulative XP required to reach each level. Index 0 = level 1 (always 0),
    /// index 1 = level 2, etc.
    thresholds: Vec<u64>,
}

impl ExpTable {
    /// Build the standard D2-approximate experience table (99 levels).
    ///
    /// Level 1 requires 0 XP, level 2 requires 500 XP, then each subsequent
    /// threshold grows by a factor of approximately 1.15.
    #[must_use]
    pub fn d2_standard() -> Self {
        let mut thresholds = Vec::with_capacity(99);
        thresholds.push(0); // level 1

        let mut prev: f64 = 500.0;
        thresholds.push(prev as u64); // level 2

        for _ in 2..99 {
            prev = (prev * 1.15).round();
            thresholds.push(prev as u64);
        }
        Self { thresholds }
    }

    /// Return the cumulative XP required to reach the given `level` (1-based).
    ///
    /// Returns `None` if `level` is 0 or exceeds the table size.
    #[must_use]
    pub fn xp_for_level(&self, level: u32) -> Option<u64> {
        if level == 0 {
            return None;
        }
        self.thresholds.get((level - 1) as usize).copied()
    }

    /// Determine the highest level attainable with the given cumulative `xp`.
    #[must_use]
    pub fn level_for_xp(&self, xp: u64) -> u32 {
        // Walk from the top down to find the highest level whose threshold <= xp.
        for (i, &threshold) in self.thresholds.iter().enumerate().rev() {
            if xp >= threshold {
                return (i as u32) + 1;
            }
        }
        1
    }
}

/// Character level state: current level, XP, and unspent points.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharacterLevel {
    /// Current level (1..=99).
    pub level: u32,
    /// Cumulative experience points.
    pub experience: u64,
    /// Unspent stat points (gained on level-up).
    pub stat_points: u32,
    /// Unspent skill points (gained on level-up).
    pub skill_points: u32,
}

impl Default for CharacterLevel {
    fn default() -> Self {
        Self::new()
    }
}

impl CharacterLevel {
    /// Create a fresh level-1 character.
    #[must_use]
    pub fn new() -> Self {
        Self {
            level: 1,
            experience: 0,
            stat_points: 0,
            skill_points: 0,
        }
    }

    /// Grant experience and process any resulting level-ups.
    ///
    /// Each level-up awards 5 stat points and 1 skill point (D2 standard).
    /// The maximum level is capped at 99.
    pub fn add_experience(&mut self, xp: u64, table: &ExpTable) {
        self.experience = self.experience.saturating_add(xp);
        let new_level = table.level_for_xp(self.experience).min(99);
        if new_level > self.level {
            let levels_gained = new_level - self.level;
            self.stat_points += levels_gained * 5;
            self.skill_points += levels_gained;
            self.level = new_level;
        }
    }

    /// Spend one stat point. Returns `true` if successful, `false` if none available.
    pub fn spend_stat_point(&mut self) -> bool {
        if self.stat_points > 0 {
            self.stat_points -= 1;
            true
        } else {
            false
        }
    }

    /// Spend one skill point. Returns `true` if successful, `false` if none available.
    pub fn spend_skill_point(&mut self) -> bool {
        if self.skill_points > 0 {
            self.skill_points -= 1;
            true
        } else {
            false
        }
    }
}
