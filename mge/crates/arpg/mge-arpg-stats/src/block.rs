// @id: MGE-ARPG-Stats-Block @do: stat-block @role: back-end @layer: 3 @human: miyuk
//! Complete stat block combining base, derived, current pools, and level.

use crate::base::BaseStats;
use crate::class::CharacterClass;
use crate::derived::DerivedStats;
use crate::level::CharacterLevel;

/// Full stat profile for a living entity (player character, hireling, etc.).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatBlock {
    /// Primary attributes.
    pub base: BaseStats,
    /// Computed secondary stats (life formula, mana formula, resistances, etc.).
    pub derived: DerivedStats,
    /// Character class (determines derived-stat formulas).
    pub class: CharacterClass,
    /// Current hit points (0 = dead).
    pub current_life: i32,
    /// Current mana points.
    pub current_mana: i32,
    /// Current stamina points.
    pub current_stamina: i32,
    /// Level / XP state.
    pub level: CharacterLevel,
}

impl StatBlock {
    /// Build a new stat block from base attributes and a character class.
    ///
    /// All current pools start at their maximum values.
    #[must_use]
    pub fn new(base: BaseStats, class: CharacterClass) -> Self {
        let derived = DerivedStats::from_base(&base, class);
        Self {
            current_life: derived.max_life,
            current_mana: derived.max_mana,
            current_stamina: derived.max_stamina,
            base,
            derived,
            class,
            level: CharacterLevel::new(),
        }
    }

    /// Whether the entity is still alive (life > 0).
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.current_life > 0
    }

    /// Inflict damage, clamping life to 0. Returns the actual damage dealt.
    pub fn take_damage(&mut self, amount: i32) -> i32 {
        let actual = amount.min(self.current_life).max(0);
        self.current_life -= actual;
        actual
    }

    /// Restore life, clamping to max life.
    pub fn restore_life(&mut self, amount: i32) {
        self.current_life = (self.current_life + amount).min(self.derived.max_life);
    }

    /// Restore mana, clamping to max mana.
    pub fn restore_mana(&mut self, amount: i32) {
        self.current_mana = (self.current_mana + amount).min(self.derived.max_mana);
    }

    /// Recalculate derived stats from the current base attributes.
    ///
    /// Call this after modifying base stats (e.g. equipping an item that adds
    /// strength). Current pools are clamped to new maximums.
    pub fn recalculate(&mut self) {
        self.derived = DerivedStats::from_base(&self.base, self.class);
        self.derived.apply_resistance_cap();
        self.current_life = self.current_life.min(self.derived.max_life);
        self.current_mana = self.current_mana.min(self.derived.max_mana);
        self.current_stamina = self.current_stamina.min(self.derived.max_stamina);
    }
}
