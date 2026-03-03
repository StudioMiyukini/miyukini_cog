// @id: MGE-ARPG-Stats-Derived @do: derived-stats @role: back-end @layer: 3 @human: miyuk
//! Derived (secondary) stats computed from base attributes and class identity.

use crate::base::BaseStats;
use crate::class::CharacterClass;

/// Maximum resistance cap (D2 standard: 75%).
pub const RESISTANCE_CAP: i32 = 75;

/// Secondary stats computed from [`BaseStats`] plus equipment/buff modifiers.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DerivedStats {
    /// Maximum hit points.
    pub max_life: i32,
    /// Maximum mana points.
    pub max_mana: i32,
    /// Maximum stamina points.
    pub max_stamina: i32,
    /// Physical armor value.
    pub armor: i32,
    /// Minimum weapon damage.
    pub min_damage: i32,
    /// Maximum weapon damage.
    pub max_damage: i32,
    /// Attack rating (chance to hit).
    pub attack_rating: i32,
    /// Defense rating (chance to be missed).
    pub defense_rating: i32,
    /// Fire resistance (%).
    pub fire_res: i32,
    /// Cold resistance (%).
    pub cold_res: i32,
    /// Lightning resistance (%).
    pub light_res: i32,
    /// Poison resistance (%).
    pub poison_res: i32,
    /// Faster Run/Walk (%).
    pub faster_run_walk: i32,
    /// Faster Cast Rate (%).
    pub faster_cast_rate: i32,
    /// Increased Attack Speed (%).
    pub increased_attack_speed: i32,
}

impl DerivedStats {
    /// Compute derived stats from base attributes and a character class.
    ///
    /// Per-class coefficients (base HP, VIT-per-HP, base mana, ENE-per-mana)
    /// allow each class to scale differently. Currently all classes share the
    /// same D2-approximate formulas; class-specific tuning will follow.
    #[must_use]
    pub fn from_base(base: &BaseStats, class: CharacterClass) -> Self {
        let (base_hp, vit_per_hp, base_mana, ene_per_mana) = Self::class_coefficients(class);

        let vit_eff = base.vitality.effective();
        let ene_eff = base.energy.effective();
        let str_eff = base.strength.effective();
        let dex_eff = base.dexterity.effective();

        Self {
            max_life: base_hp + vit_eff * vit_per_hp,
            max_mana: base_mana + ene_eff * ene_per_mana,
            max_stamina: vit_eff * 2 + 80,
            armor: dex_eff / 4,
            min_damage: 1 + str_eff / 10,
            max_damage: 2 + str_eff / 5,
            attack_rating: dex_eff * 5,
            defense_rating: dex_eff * 4,
            fire_res: 0,
            cold_res: 0,
            light_res: 0,
            poison_res: 0,
            faster_run_walk: 0,
            faster_cast_rate: 0,
            increased_attack_speed: 0,
        }
    }

    /// Returns `(base_hp, vit_per_hp, base_mana, ene_per_mana)` for a class.
    ///
    /// All classes currently share the Necromancer formula: `base_hp=45`,
    /// `vit_per_hp=2`, `base_mana=25`, `ene_per_mana=2`. Per-class
    /// differentiation will be added in a future task.
    #[must_use]
    const fn class_coefficients(_class: CharacterClass) -> (i32, i32, i32, i32) {
        // Uniform for now — differentiation tracked as future work.
        (45, 2, 25, 2)
    }

    /// Clamp all four elemental resistances to [`RESISTANCE_CAP`].
    pub fn apply_resistance_cap(&mut self) {
        self.fire_res = self.fire_res.min(RESISTANCE_CAP);
        self.cold_res = self.cold_res.min(RESISTANCE_CAP);
        self.light_res = self.light_res.min(RESISTANCE_CAP);
        self.poison_res = self.poison_res.min(RESISTANCE_CAP);
    }
}
