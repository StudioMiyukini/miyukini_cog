//! Static tuning for each mercenary archetype: recruitment cost, per-level stat
//! growth, combat reach, and starting weapon. The mercenary system reads a
//! profile to (re)compute a merc's resolved stats from its level and gear.

use mge_items::item::ItemKind;

use crate::components::mercenary::MercType;

/// Per-archetype mercenary tuning.
#[derive(Debug, Clone, Copy)]
pub struct MercProfile {
    pub merc_type: MercType,
    /// Gold cost to hire at the village.
    pub recruit_cost: u64,
    pub base_hp: f32,
    pub hp_per_level: f32,
    pub base_dmg_min: f32,
    pub base_dmg_max: f32,
    pub dmg_per_level: f32,
    pub base_def: f32,
    pub def_per_level: f32,
    /// Flat dodge rating (archers are slippery).
    pub dodge: f32,
    pub attack_range: f32,
    pub attack_interval: f32,
    pub move_speed: f32,
    pub weapon: ItemKind,
    /// Off-hand starting item. `Some(weapon)` makes the merc dual-wield (block +
    /// faster swings + per-hand damage); `None` leaves the off-hand free.
    pub offhand: Option<ItemKind>,
}

pub const GUARDIAN: MercProfile = MercProfile {
    merc_type: MercType::Guardian,
    recruit_cost: 500,
    base_hp: 120.0,
    hp_per_level: 18.0,
    base_dmg_min: 6.0,
    base_dmg_max: 12.0,
    dmg_per_level: 2.0,
    base_def: 20.0,
    def_per_level: 4.0,
    dodge: 5.0,
    attack_range: 1.7,
    attack_interval: 1.1,
    move_speed: 3.6,
    weapon: ItemKind::Sword,
    // The Guardian fights with a blade in each hand.
    offhand: Some(ItemKind::Axe),
};

pub const ARCHER: MercProfile = MercProfile {
    merc_type: MercType::Archer,
    recruit_cost: 400,
    base_hp: 70.0,
    hp_per_level: 9.0,
    base_dmg_min: 8.0,
    base_dmg_max: 16.0,
    dmg_per_level: 2.5,
    base_def: 8.0,
    def_per_level: 1.5,
    dodge: 40.0,
    attack_range: 7.0,
    attack_interval: 0.9,
    move_speed: 4.2,
    weapon: ItemKind::Bow,
    offhand: None,
};

pub const HEALER: MercProfile = MercProfile {
    merc_type: MercType::Healer,
    recruit_cost: 600,
    base_hp: 80.0,
    hp_per_level: 10.0,
    base_dmg_min: 3.0,
    base_dmg_max: 6.0,
    dmg_per_level: 1.0,
    base_def: 10.0,
    def_per_level: 2.0,
    dodge: 10.0,
    attack_range: 5.0,
    attack_interval: 1.5,
    move_speed: 4.0,
    weapon: ItemKind::Wand,
    offhand: None,
};

pub const MAGE: MercProfile = MercProfile {
    merc_type: MercType::Mage,
    recruit_cost: 550,
    base_hp: 75.0,
    hp_per_level: 9.0,
    base_dmg_min: 7.0,
    base_dmg_max: 14.0,
    dmg_per_level: 2.2,
    base_def: 8.0,
    def_per_level: 1.5,
    dodge: 8.0,
    attack_range: 6.0,
    attack_interval: 1.4,
    move_speed: 3.8,
    weapon: ItemKind::Wand,
    offhand: None,
};

/// The static profile for a mercenary archetype.
#[must_use]
pub fn merc_profile(merc_type: MercType) -> MercProfile {
    match merc_type {
        MercType::Guardian => GUARDIAN,
        MercType::Archer => ARCHER,
        MercType::Healer => HEALER,
        MercType::Mage => MAGE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recruit_costs_match_design() {
        assert_eq!(merc_profile(MercType::Guardian).recruit_cost, 500);
        assert_eq!(merc_profile(MercType::Archer).recruit_cost, 400);
        assert_eq!(merc_profile(MercType::Healer).recruit_cost, 600);
        assert_eq!(merc_profile(MercType::Mage).recruit_cost, 550);
    }

    #[test]
    fn guardian_is_tankier_than_archer() {
        assert!(GUARDIAN.base_hp > ARCHER.base_hp);
        assert!(GUARDIAN.base_def > ARCHER.base_def);
        assert!(ARCHER.dodge > GUARDIAN.dodge);
    }
}
