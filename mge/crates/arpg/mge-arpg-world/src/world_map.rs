// @id: MGE-ARPG-World-WorldMap @do: world-map @role: back-end @layer: 3 @human: miyuk

//! `WorldMap`: registry of all loaded zones.
//!
//! Also contains Act 1 monastery zone definitions and the `act1()` constructor
//! that builds a complete Act 1 world map from outdoor + monastery zones.

use std::collections::HashMap;

use crate::error::WorldError;
use crate::zone::{Zone, ZoneDef, ZoneId};

/// Runtime registry of all world zones.
#[derive(Debug, Default)]
pub struct WorldMap {
    zones: HashMap<String, Zone>,
}

impl WorldMap {
    /// Create an empty world map.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a zone. Overwrites any existing zone with the same ID.
    pub fn register(&mut self, zone: Zone) {
        self.zones.insert(zone.def.id.0.clone(), zone);
    }

    /// Get an immutable reference to a zone by ID.
    ///
    /// # Errors
    ///
    /// Returns `WorldError::ZoneNotFound` if the zone is not registered.
    pub fn get(&self, id: &ZoneId) -> Result<&Zone, WorldError> {
        self.zones
            .get(&id.0)
            .ok_or_else(|| WorldError::ZoneNotFound { id: id.0.clone() })
    }

    /// Get a mutable reference to a zone by ID.
    ///
    /// # Errors
    ///
    /// Returns `WorldError::ZoneNotFound` if the zone is not registered.
    pub fn get_mut(&mut self, id: &ZoneId) -> Result<&mut Zone, WorldError> {
        self.zones
            .get_mut(&id.0)
            .ok_or_else(|| WorldError::ZoneNotFound { id: id.0.clone() })
    }

    /// All registered zone IDs.
    #[must_use]
    pub fn zone_ids(&self) -> Vec<&String> {
        self.zones.keys().collect()
    }

    /// Number of registered zones.
    #[must_use]
    pub fn zone_count(&self) -> usize {
        self.zones.len()
    }

    /// Returns `true` if no zones are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.zones.is_empty()
    }

    /// Build a complete Act 1 world map from all outdoor + monastery zones.
    ///
    /// Returns a `WorldMap` with 18+ zones (7 outdoor + 11 monastery).
    /// Dungeon zones will be added separately when the dungeon module is ready.
    #[must_use]
    pub fn act1() -> Self {
        let mut map = Self::new();
        for def in act1_all_zone_defs() {
            map.register(Zone::new(def));
        }
        map
    }
}

// ---------------------------------------------------------------------------
// Act 1 — Monastery zone definitions (Outer Cloister through Catacombs L4)
// @id: MGE-ARPG-World-Act1-Monastery @do: act1-monastery @role: back-end @layer: 3 @human: miyuk
// ---------------------------------------------------------------------------

/// Outer Cloister — monastery entrance, level 9.
#[must_use]
pub fn outer_cloister() -> ZoneDef {
    ZoneDef {
        id: ZoneId::new("act1_outer_cloister"),
        name: "Outer Cloister".to_string(),
        act: 1,
        difficulty: 9,
        monster_level: 9,
        width_chunks: 3,
        height_chunks: 3,
        ambient_id: "bgm_monastery".to_string(),
        connections: vec![
            ZoneId::new("act1_tamoe_highland"),
            ZoneId::new("act1_barracks"),
        ],
        monster_families: vec!["Dark Hunter".to_string()],
    }
}

/// Barracks — corrupted rogue garrison, level 9.
#[must_use]
pub fn barracks() -> ZoneDef {
    ZoneDef {
        id: ZoneId::new("act1_barracks"),
        name: "Barracks".to_string(),
        act: 1,
        difficulty: 9,
        monster_level: 9,
        width_chunks: 3,
        height_chunks: 3,
        ambient_id: "bgm_monastery".to_string(),
        connections: vec![
            ZoneId::new("act1_outer_cloister"),
            ZoneId::new("act1_jail_l1"),
        ],
        monster_families: vec!["Corrupted Rogue".to_string()],
    }
}

/// Jail Level 1 — first dungeon level, level 10.
#[must_use]
pub fn jail_l1() -> ZoneDef {
    ZoneDef {
        id: ZoneId::new("act1_jail_l1"),
        name: "Jail L1".to_string(),
        act: 1,
        difficulty: 10,
        monster_level: 10,
        width_chunks: 3,
        height_chunks: 3,
        ambient_id: "bgm_dungeon".to_string(),
        connections: vec![
            ZoneId::new("act1_barracks"),
            ZoneId::new("act1_jail_l2"),
        ],
        monster_families: vec!["Skeleton".to_string(), "Tainted".to_string()],
    }
}

/// Jail Level 2 — middle jail, level 10.
#[must_use]
pub fn jail_l2() -> ZoneDef {
    ZoneDef {
        id: ZoneId::new("act1_jail_l2"),
        name: "Jail L2".to_string(),
        act: 1,
        difficulty: 10,
        monster_level: 10,
        width_chunks: 3,
        height_chunks: 3,
        ambient_id: "bgm_dungeon".to_string(),
        connections: vec![
            ZoneId::new("act1_jail_l1"),
            ZoneId::new("act1_jail_l3"),
        ],
        monster_families: vec!["Skeleton".to_string(), "Tainted".to_string()],
    }
}

/// Jail Level 3 — deepest jail, level 10.
#[must_use]
pub fn jail_l3() -> ZoneDef {
    ZoneDef {
        id: ZoneId::new("act1_jail_l3"),
        name: "Jail L3".to_string(),
        act: 1,
        difficulty: 10,
        monster_level: 10,
        width_chunks: 3,
        height_chunks: 3,
        ambient_id: "bgm_dungeon".to_string(),
        connections: vec![
            ZoneId::new("act1_jail_l2"),
            ZoneId::new("act1_inner_cloister"),
        ],
        monster_families: vec!["Skeleton".to_string(), "Tainted".to_string()],
    }
}

/// Inner Cloister — safe transitional zone, level 10, no monsters.
#[must_use]
pub fn inner_cloister() -> ZoneDef {
    ZoneDef {
        id: ZoneId::new("act1_inner_cloister"),
        name: "Inner Cloister".to_string(),
        act: 1,
        difficulty: 10,
        monster_level: 10,
        width_chunks: 3,
        height_chunks: 3,
        ambient_id: "bgm_monastery".to_string(),
        connections: vec![
            ZoneId::new("act1_jail_l3"),
            ZoneId::new("act1_cathedral"),
        ],
        monster_families: Vec::new(),
    }
}

/// Cathedral — level 11, undead zone.
#[must_use]
pub fn cathedral() -> ZoneDef {
    ZoneDef {
        id: ZoneId::new("act1_cathedral"),
        name: "Cathedral".to_string(),
        act: 1,
        difficulty: 11,
        monster_level: 11,
        width_chunks: 3,
        height_chunks: 3,
        ambient_id: "bgm_monastery".to_string(),
        connections: vec![
            ZoneId::new("act1_inner_cloister"),
            ZoneId::new("act1_catacombs_l1"),
        ],
        monster_families: vec!["Skeleton".to_string(), "Ghoul".to_string()],
    }
}

/// Catacombs Level 1 — first catacomb layer, level 11.
#[must_use]
pub fn catacombs_l1() -> ZoneDef {
    ZoneDef {
        id: ZoneId::new("act1_catacombs_l1"),
        name: "Catacombs L1".to_string(),
        act: 1,
        difficulty: 11,
        monster_level: 11,
        width_chunks: 3,
        height_chunks: 3,
        ambient_id: "bgm_dungeon".to_string(),
        connections: vec![
            ZoneId::new("act1_cathedral"),
            ZoneId::new("act1_catacombs_l2"),
        ],
        monster_families: vec!["Skeleton Mage".to_string(), "Ghoul".to_string()],
    }
}

/// Catacombs Level 2 — second catacomb layer, level 11.
#[must_use]
pub fn catacombs_l2() -> ZoneDef {
    ZoneDef {
        id: ZoneId::new("act1_catacombs_l2"),
        name: "Catacombs L2".to_string(),
        act: 1,
        difficulty: 11,
        monster_level: 11,
        width_chunks: 3,
        height_chunks: 3,
        ambient_id: "bgm_dungeon".to_string(),
        connections: vec![
            ZoneId::new("act1_catacombs_l1"),
            ZoneId::new("act1_catacombs_l3"),
        ],
        monster_families: vec!["Skeleton Mage".to_string(), "Ghoul".to_string()],
    }
}

/// Catacombs Level 3 — third catacomb layer, level 12.
#[must_use]
pub fn catacombs_l3() -> ZoneDef {
    ZoneDef {
        id: ZoneId::new("act1_catacombs_l3"),
        name: "Catacombs L3".to_string(),
        act: 1,
        difficulty: 12,
        monster_level: 12,
        width_chunks: 3,
        height_chunks: 3,
        ambient_id: "bgm_dungeon".to_string(),
        connections: vec![
            ZoneId::new("act1_catacombs_l2"),
            ZoneId::new("act1_catacombs_l4"),
        ],
        monster_families: vec!["Skeleton Mage".to_string(), "Ghoul".to_string()],
    }
}

/// Catacombs Level 4 — Andariel's lair, boss zone, level 12.
#[must_use]
pub fn catacombs_l4() -> ZoneDef {
    ZoneDef {
        id: ZoneId::new("act1_catacombs_l4"),
        name: "Catacombs L4".to_string(),
        act: 1,
        difficulty: 12,
        monster_level: 12,
        width_chunks: 3,
        height_chunks: 3,
        ambient_id: "bgm_dungeon".to_string(),
        connections: vec![ZoneId::new("act1_catacombs_l3")],
        monster_families: vec!["Andariel".to_string()],
    }
}

/// All Act 1 monastery zones (Outer Cloister through Catacombs L4).
///
/// Returns 11 zones covering the monastery, jail, cathedral, and catacombs.
#[must_use]
pub fn act1_monastery_zones() -> Vec<ZoneDef> {
    vec![
        outer_cloister(),
        barracks(),
        jail_l1(),
        jail_l2(),
        jail_l3(),
        inner_cloister(),
        cathedral(),
        catacombs_l1(),
        catacombs_l2(),
        catacombs_l3(),
        catacombs_l4(),
    ]
}

/// All Act 1 zone definitions (outdoor + monastery + dungeons).
///
/// Returns 33 zones: 7 outdoor + 11 monastery + 15 dungeon levels.
#[must_use]
pub fn act1_all_zone_defs() -> Vec<ZoneDef> {
    let mut defs = crate::zone::act1_outdoor_zones();
    defs.extend(act1_monastery_zones());
    defs.extend(crate::dungeon::act1_dungeon_zones());
    defs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zone::ZoneDef;

    fn make_zone(id: &str) -> Zone {
        Zone::new(ZoneDef {
            id: ZoneId::new(id),
            name: format!("Zone {id}"),
            act: 1,
            difficulty: 10,
            monster_level: 12,
            width_chunks: 2,
            height_chunks: 2,
            ambient_id: "bgm_test".to_string(),
            connections: Vec::new(),
            monster_families: Vec::new(),
        })
    }

    #[test]
    fn test_world_map_register_and_get() {
        let mut map = WorldMap::new();
        map.register(make_zone("zone_a"));
        let result = map.get(&ZoneId::new("zone_a"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().def.id.as_str(), "zone_a");
    }

    #[test]
    fn test_world_map_zone_not_found() {
        let map = WorldMap::new();
        let result = map.get(&ZoneId::new("nonexistent"));
        assert!(result.is_err());
    }

    #[test]
    fn test_world_map_zone_count() {
        let mut map = WorldMap::new();
        assert!(map.is_empty());
        assert_eq!(map.zone_count(), 0);

        map.register(make_zone("zone_a"));
        map.register(make_zone("zone_b"));
        assert_eq!(map.zone_count(), 2);
        assert!(!map.is_empty());
        assert_eq!(map.zone_ids().len(), 2);
    }

    // -- Act 1 monastery zone tests --

    #[test]
    fn monastery_11_zones() {
        let zones = act1_monastery_zones();
        assert_eq!(
            zones.len(),
            11,
            "Expected 11 monastery zones, got {}",
            zones.len()
        );
        // All must be Act 1
        for z in &zones {
            assert_eq!(z.act, 1, "Zone {} should be Act 1", z.name);
        }
        // All monastery zones use 3x3 chunks
        for z in &zones {
            assert_eq!(
                z.width_chunks, 3,
                "Zone {} should have width_chunks=3",
                z.name
            );
            assert_eq!(
                z.height_chunks, 3,
                "Zone {} should have height_chunks=3",
                z.name
            );
        }
    }

    #[test]
    fn world_map_act1_total() {
        let map = WorldMap::act1();
        assert!(
            map.zone_count() >= 18,
            "Act 1 world map should have at least 18 zones, got {}",
            map.zone_count()
        );
    }

    #[test]
    fn catacombs_l4_boss() {
        let zones = act1_monastery_zones();
        let cat4 = zones
            .iter()
            .find(|z| z.id.as_str() == "act1_catacombs_l4");
        assert!(cat4.is_some(), "Catacombs L4 should exist in monastery zones");
        let cat4 = cat4.unwrap();
        assert!(
            cat4.monster_families.iter().any(|m| m == "Andariel"),
            "Catacombs L4 should have Andariel in monster_families, got {:?}",
            cat4.monster_families
        );
    }

    #[test]
    fn monastery_connections_bidirectional() {
        use std::collections::HashMap;
        let zones = act1_monastery_zones();
        let zone_map: HashMap<&str, &ZoneDef> =
            zones.iter().map(|z| (z.id.as_str(), z)).collect();

        for zone in &zones {
            for conn in &zone.connections {
                // Only check bidirectionality for zones within the monastery set.
                // Connections to outdoor zones (e.g. tamoe_highland) are validated
                // at the full act1 level.
                if let Some(target) = zone_map.get(conn.as_str()) {
                    let back_link = target
                        .connections
                        .iter()
                        .any(|c| c.as_str() == zone.id.as_str());
                    assert!(
                        back_link,
                        "Zone {} connects to {} but {} does not connect back",
                        zone.id.as_str(),
                        conn.as_str(),
                        conn.as_str(),
                    );
                }
            }
        }
    }

    #[test]
    fn act1_all_zone_defs_no_duplicate_ids() {
        use std::collections::HashSet;
        let defs = act1_all_zone_defs();
        let mut seen = HashSet::new();
        for d in &defs {
            assert!(
                seen.insert(d.id.as_str()),
                "Duplicate zone ID: {}",
                d.id.as_str()
            );
        }
    }
}
