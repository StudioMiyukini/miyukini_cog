use serde::{Deserialize, Serialize};

use mge_world::zone::{Biome, ZoneId};

/// Monster family / genus grouping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MonsterFamily {
    Fallen,
    Zombie,
    Skeleton,
    BloodRaven,
    Golem,
    Boss,
}

/// Tactical role determining how the monster behaves in combat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MonsterRole {
    /// Charges into melee range.
    Melee,
    /// Attacks from distance.
    Ranged,
    /// Uses skills / spells.
    Caster,
    /// Spawns reinforcements.
    Summoner,
    /// Runs away when low on HP.
    Flee,
}

/// Static definition of a monster type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterDef {
    pub id: u32,
    pub name: String,
    pub family: MonsterFamily,
    pub role: MonsterRole,
    /// Primary biome where this monster spawns.
    pub biome: Biome,
    /// Base monster level (drives stat scaling).
    pub base_level: u32,
    pub base_hp: u32,
    pub base_damage: u32,
    /// XP awarded on kill.
    pub xp_reward: u32,
    /// Drop rate 0–100.
    pub drop_chance: u8,
    pub is_boss: bool,
}

impl MonsterDef {
    /// Effective HP at the given area level using a simple linear scale.
    #[must_use]
    pub fn hp_at_level(&self, area_level: u32) -> u32 {
        self.base_hp.saturating_add(
            self.base_hp
                .saturating_mul(area_level.saturating_sub(self.base_level))
                / 4,
        )
    }
}

/// Act 1 monster roster.
#[must_use]
pub fn act1_roster() -> Vec<MonsterDef> {
    vec![
        MonsterDef {
            id: 1,
            name: "Fallen".into(),
            family: MonsterFamily::Fallen,
            role: MonsterRole::Melee,
            biome: Biome::Wilderness,
            base_level: 1,
            base_hp: 30,
            base_damage: 5,
            xp_reward: 10,
            drop_chance: 30,
            is_boss: false,
        },
        MonsterDef {
            id: 2,
            name: "Fallen Shaman".into(),
            family: MonsterFamily::Fallen,
            role: MonsterRole::Summoner,
            biome: Biome::Wilderness,
            base_level: 2,
            base_hp: 40,
            base_damage: 8,
            xp_reward: 20,
            drop_chance: 35,
            is_boss: false,
        },
        MonsterDef {
            id: 3,
            name: "Zombie".into(),
            family: MonsterFamily::Zombie,
            role: MonsterRole::Melee,
            biome: Biome::Forest,
            base_level: 2,
            base_hp: 60,
            base_damage: 10,
            xp_reward: 18,
            drop_chance: 25,
            is_boss: false,
        },
        MonsterDef {
            id: 4,
            name: "Skeleton".into(),
            family: MonsterFamily::Skeleton,
            role: MonsterRole::Melee,
            biome: Biome::Dungeon,
            base_level: 3,
            base_hp: 50,
            base_damage: 12,
            xp_reward: 22,
            drop_chance: 28,
            is_boss: false,
        },
        MonsterDef {
            id: 5,
            name: "Dark Ranger".into(),
            family: MonsterFamily::BloodRaven,
            role: MonsterRole::Ranged,
            biome: Biome::Forest,
            base_level: 3,
            base_hp: 55,
            base_damage: 15,
            xp_reward: 30,
            drop_chance: 40,
            is_boss: false,
        },
        MonsterDef {
            id: 6,
            name: "Spike Fiend".into(),
            family: MonsterFamily::Fallen,
            role: MonsterRole::Melee,
            biome: Biome::Cave,
            base_level: 4,
            base_hp: 70,
            base_damage: 18,
            xp_reward: 35,
            drop_chance: 32,
            is_boss: false,
        },
        MonsterDef {
            id: 7,
            name: "Skeleton Mage".into(),
            family: MonsterFamily::Skeleton,
            role: MonsterRole::Caster,
            biome: Biome::Catacomb,
            base_level: 7,
            base_hp: 80,
            base_damage: 22,
            xp_reward: 55,
            drop_chance: 38,
            is_boss: false,
        },
        MonsterDef {
            id: 100,
            name: "Blood Raven".into(),
            family: MonsterFamily::BloodRaven,
            role: MonsterRole::Ranged,
            biome: Biome::Forest,
            base_level: 3,
            base_hp: 500,
            base_damage: 40,
            xp_reward: 300,
            drop_chance: 100,
            is_boss: true,
        },
        MonsterDef {
            id: 101,
            name: "Andariel".into(),
            family: MonsterFamily::Boss,
            role: MonsterRole::Caster,
            biome: Biome::Catacomb,
            base_level: 9,
            base_hp: 3_000,
            base_damage: 120,
            xp_reward: 5_000,
            drop_chance: 100,
            is_boss: true,
        },
    ]
}

/// Filter roster to a specific zone biome.
#[must_use]
pub fn roster_for_zone(roster: &[MonsterDef], zone_id: ZoneId) -> Vec<&MonsterDef> {
    // Simple biome map matching zone.rs act1_zone_graph biomes
    let biome = match zone_id {
        10 | 20 | 30 => Biome::Wilderness,
        11 | 22 => Biome::Cave,
        21 => Biome::Forest,
        31 => Biome::Dungeon,
        40 => Biome::Cathedral,
        41 | 42 => Biome::Catacomb,
        _ => return vec![], // camp (0) and unknown zones — no spawns
    };
    roster.iter().filter(|m| m.biome == biome).collect()
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roster_has_two_bosses() {
        let r = act1_roster();
        assert_eq!(r.iter().filter(|m| m.is_boss).count(), 2);
    }

    #[test]
    fn all_normal_monsters_have_positive_xp() {
        let r = act1_roster();
        assert!(r.iter().filter(|m| !m.is_boss).all(|m| m.xp_reward > 0));
    }

    #[test]
    fn hp_at_higher_level_exceeds_base() {
        let r = act1_roster();
        let fallen = r.iter().find(|m| m.id == 1).unwrap();
        assert!(fallen.hp_at_level(5) > fallen.base_hp);
    }

    #[test]
    fn roster_for_wilderness_zone_non_empty() {
        let r = act1_roster();
        assert!(!roster_for_zone(&r, 10).is_empty());
    }

    #[test]
    fn roster_for_camp_is_empty() {
        let r = act1_roster();
        assert!(roster_for_zone(&r, 0).is_empty());
    }
}
