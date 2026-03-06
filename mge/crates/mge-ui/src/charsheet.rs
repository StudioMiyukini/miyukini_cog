use serde::{Deserialize, Serialize};

/// Primary attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrimaryStat {
    Strength,
    Dexterity,
    Vitality,
    Energy,
}

/// Resistance type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResistanceType {
    Fire,
    Cold,
    Lightning,
    Poison,
}

/// Derived combat stat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DerivedStat {
    AttackRating,
    Defense,
    BlockChance,
    CritChance,
    CastSpeed,
    AttackSpeed,
    MovementSpeed,
    MagicFind,
    GoldFind,
}

/// A stat value with base + bonus breakdown.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StatValue {
    pub base: i32,
    pub bonus: i32,
}

impl StatValue {
    pub fn total(&self) -> i32 {
        self.base + self.bonus
    }
}

/// Resistance value with hell penalty and cap.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ResistanceValue {
    pub current: i32,
    pub cap: i32,
    pub penalty: i32,
}

impl ResistanceValue {
    /// Effective resistance after penalty, clamped to cap.
    pub fn effective(&self) -> i32 {
        (self.current - self.penalty).min(self.cap)
    }

    /// Whether resistance is negative (dangerous).
    pub fn is_negative(&self) -> bool {
        self.effective() < 0
    }
}

/// Full character sheet state for UI display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSheet {
    pub name: String,
    pub class: String,
    pub level: u32,
    pub experience: u64,
    pub experience_to_next: u64,
    pub stat_points_available: u32,
    pub skill_points_available: u32,

    pub strength: StatValue,
    pub dexterity: StatValue,
    pub vitality: StatValue,
    pub energy: StatValue,

    pub life: StatValue,
    pub mana: StatValue,
    pub stamina: StatValue,

    pub fire_res: ResistanceValue,
    pub cold_res: ResistanceValue,
    pub lightning_res: ResistanceValue,
    pub poison_res: ResistanceValue,

    pub attack_rating: i32,
    pub defense: i32,
    pub block_chance: i32,
    pub damage_min: i32,
    pub damage_max: i32,
    pub cast_speed: i32,
    pub attack_speed: i32,
    pub movement_speed: i32,
    pub magic_find: i32,
    pub gold_find: i32,
}

impl CharacterSheet {
    /// Experience progress as fraction `[0.0, 1.0]`.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn exp_fraction(&self) -> f32 {
        if self.experience_to_next == 0 {
            return 0.0;
        }
        (self.experience as f64 / self.experience_to_next as f64).min(1.0) as f32
    }

    /// Get a primary stat value.
    pub fn primary(&self, stat: PrimaryStat) -> &StatValue {
        match stat {
            PrimaryStat::Strength => &self.strength,
            PrimaryStat::Dexterity => &self.dexterity,
            PrimaryStat::Vitality => &self.vitality,
            PrimaryStat::Energy => &self.energy,
        }
    }

    /// Get a resistance value.
    pub fn resistance(&self, kind: ResistanceType) -> &ResistanceValue {
        match kind {
            ResistanceType::Fire => &self.fire_res,
            ResistanceType::Cold => &self.cold_res,
            ResistanceType::Lightning => &self.lightning_res,
            ResistanceType::Poison => &self.poison_res,
        }
    }

    /// Whether any resistance is negative.
    pub fn has_negative_resistances(&self) -> bool {
        self.fire_res.is_negative()
            || self.cold_res.is_negative()
            || self.lightning_res.is_negative()
            || self.poison_res.is_negative()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_sheet() -> CharacterSheet {
        CharacterSheet {
            name: "TestHero".into(),
            class: "Warrior".into(),
            level: 30,
            experience: 500_000,
            experience_to_next: 1_000_000,
            stat_points_available: 5,
            skill_points_available: 2,
            strength: StatValue { base: 100, bonus: 50 },
            dexterity: StatValue { base: 75, bonus: 20 },
            vitality: StatValue { base: 80, bonus: 30 },
            energy: StatValue { base: 25, bonus: 10 },
            life: StatValue { base: 300, bonus: 150 },
            mana: StatValue { base: 100, bonus: 50 },
            stamina: StatValue { base: 200, bonus: 0 },
            fire_res: ResistanceValue { current: 75, cap: 75, penalty: 0 },
            cold_res: ResistanceValue { current: 40, cap: 75, penalty: 100 },
            lightning_res: ResistanceValue { current: 50, cap: 75, penalty: 0 },
            poison_res: ResistanceValue { current: 30, cap: 75, penalty: 0 },
            attack_rating: 1500,
            defense: 800,
            block_chance: 35,
            damage_min: 50,
            damage_max: 120,
            cast_speed: 0,
            attack_speed: 20,
            movement_speed: 30,
            magic_find: 150,
            gold_find: 80,
        }
    }

    #[test]
    fn stat_value_total() {
        let sv = StatValue { base: 100, bonus: 50 };
        assert_eq!(sv.total(), 150);
    }

    #[test]
    fn resistance_effective_with_penalty() {
        let sheet = sample_sheet();
        assert_eq!(sheet.cold_res.effective(), -60);
        assert!(sheet.cold_res.is_negative());
        assert_eq!(sheet.fire_res.effective(), 75);
        assert!(!sheet.fire_res.is_negative());
    }

    #[test]
    fn exp_fraction() {
        let sheet = sample_sheet();
        assert!((sheet.exp_fraction() - 0.5).abs() < 0.01);
    }

    #[test]
    fn negative_resistance_detection() {
        let sheet = sample_sheet();
        assert!(sheet.has_negative_resistances());
    }

    #[test]
    fn primary_stat_lookup() {
        let sheet = sample_sheet();
        assert_eq!(sheet.primary(PrimaryStat::Strength).total(), 150);
    }
}
