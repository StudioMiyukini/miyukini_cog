// @id: Sodomight-Content-Zones @do: act1-zone-definitions @role: back-end @layer: 4 @human: miyuk
//! Act 1 zone definitions and monster population mappings.
//!
//! Zones are ordered by intended progression: town, then increasing
//! difficulty areas through to the act boss arena.
#![allow(clippy::too_many_lines)]

// ---------------------------------------------------------------------------
// Zone definitions
// ---------------------------------------------------------------------------

/// Static zone definition for Sodomight's content layer.
///
/// Extends the engine's `ZoneDef` with monster population data needed by the
/// spawning system. The engine's `ZoneDef` uses chunk-based dimensions; here
/// we specify tile counts directly since content authors think in tiles.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContentZoneDef {
    /// Unique zone identifier.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Act number (1-based).
    pub act: u8,
    /// Zone width in tiles.
    pub width_tiles: u32,
    /// Zone height in tiles.
    pub height_tiles: u32,
    /// Monster archetype IDs that can spawn in this zone.
    pub monster_ids: Vec<String>,
    /// Target monster density: monsters per 100 tiles.
    pub monster_density: u32,
    /// Whether this zone is a safe town (no monsters, no PvP).
    pub is_town: bool,
}

/// Returns Act 1 zone definitions.
///
/// Zones are ordered by intended progression: town, then increasing
/// difficulty areas through to the act boss arena. Monster populations use
/// the 15-family bestiary IDs.
#[must_use]
pub fn act1_zones() -> Vec<ContentZoneDef> {
    vec![
        ContentZoneDef {
            id: "rogue_encampment".into(),
            name: "Rogue Encampment".into(),
            act: 1,
            width_tiles: 30,
            height_tiles: 20,
            monster_ids: vec![],
            monster_density: 0,
            is_town: true,
        },
        ContentZoneDef {
            id: "blood_moor".into(),
            name: "Blood Moor".into(),
            act: 1,
            width_tiles: 40,
            height_tiles: 40,
            monster_ids: vec![
                "fallen".into(),
                "fallen_shaman".into(),
                "zombie".into(),
                "quill_rat".into(),
            ],
            monster_density: 8,
            is_town: false,
        },
        ContentZoneDef {
            id: "den_of_evil".into(),
            name: "Den of Evil".into(),
            act: 1,
            width_tiles: 20,
            height_tiles: 15,
            monster_ids: vec!["fallen".into(), "fallen_shaman".into(), "zombie".into()],
            monster_density: 15,
            is_town: false,
        },
        ContentZoneDef {
            id: "cold_plains".into(),
            name: "Cold Plains".into(),
            act: 1,
            width_tiles: 50,
            height_tiles: 50,
            monster_ids: vec![
                "skeleton_warrior".into(),
                "skeleton_archer".into(),
                "corrupted_rogue_melee".into(),
            ],
            monster_density: 6,
            is_town: false,
        },
        ContentZoneDef {
            id: "stony_field".into(),
            name: "Stony Field".into(),
            act: 1,
            width_tiles: 45,
            height_tiles: 45,
            monster_ids: vec![
                "skeleton_warrior".into(),
                "goatman_melee".into(),
                "goatman_fire".into(),
            ],
            monster_density: 7,
            is_town: false,
        },
        ContentZoneDef {
            id: "dark_wood".into(),
            name: "Dark Wood".into(),
            act: 1,
            width_tiles: 40,
            height_tiles: 40,
            monster_ids: vec![
                "brute".into(),
                "corrupted_rogue_melee".into(),
                "corrupted_rogue_archer".into(),
            ],
            monster_density: 8,
            is_town: false,
        },
        ContentZoneDef {
            id: "black_marsh".into(),
            name: "Black Marsh".into(),
            act: 1,
            width_tiles: 50,
            height_tiles: 50,
            monster_ids: vec![
                "tainted".into(),
                "ghoul".into(),
                "dark_hunter".into(),
            ],
            monster_density: 7,
            is_town: false,
        },
        ContentZoneDef {
            id: "tamoe_highland".into(),
            name: "Tamoe Highland".into(),
            act: 1,
            width_tiles: 45,
            height_tiles: 45,
            monster_ids: vec![
                "dark_hunter".into(),
                "skeleton_mage".into(),
                "corrupted_rogue_archer".into(),
            ],
            monster_density: 8,
            is_town: false,
        },
        ContentZoneDef {
            id: "burial_grounds".into(),
            name: "Burial Grounds".into(),
            act: 1,
            width_tiles: 25,
            height_tiles: 25,
            monster_ids: vec!["skeleton_warrior".into(), "skeleton_archer".into()],
            monster_density: 10,
            is_town: false,
        },
        ContentZoneDef {
            id: "cathedral".into(),
            name: "Cathedral".into(),
            act: 1,
            width_tiles: 30,
            height_tiles: 30,
            monster_ids: vec![
                "skeleton_warrior".into(),
                "skeleton_mage".into(),
                "dark_hunter".into(),
            ],
            monster_density: 12,
            is_town: false,
        },
    ]
}

// ---------------------------------------------------------------------------
// Lookup helpers
// ---------------------------------------------------------------------------

/// Find a zone definition by its id.
#[must_use]
pub fn find_zone(id: &str) -> Option<ContentZoneDef> {
    act1_zones().into_iter().find(|z| z.id == id)
}

/// Returns the monster family names that can spawn in a given zone.
///
/// Zone IDs follow the `act1_*` convention (e.g. `"act1_blood_moor"`).
/// The prefix `act1_` is stripped internally so the mapping matches
/// `ContentZoneDef::id` values. Returns an empty vec for unknown zones.
#[must_use]
pub fn monsters_for_zone(zone_id: &str) -> Vec<&'static str> {
    // Strip the act prefix if present so callers can use either form.
    let key = zone_id.strip_prefix("act1_").unwrap_or(zone_id);

    match key {
        "blood_moor" => vec!["Fallen", "Fallen Shaman", "Zombie"],
        "cold_plains" => vec!["Skeleton Warrior", "Skeleton Archer", "Corrupted Rogue"],
        "stony_field" => vec!["Skeleton Warrior", "Goatman", "Goatman Fire Clan"],
        "dark_wood" => vec!["Brute", "Corrupted Rogue", "Corrupted Rogue Archer"],
        "black_marsh" => vec!["Tainted", "Ghoul", "Dark Hunter"],
        "tamoe_highland" => vec!["Dark Hunter", "Skeleton Mage", "Corrupted Rogue Archer"],
        _ => vec![],
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_act1_zones_count() {
        let zones = act1_zones();
        // Town + 9 combat zones = 10
        assert_eq!(zones.len(), 10);
    }

    #[test]
    fn test_rogue_encampment_is_town() {
        let town = find_zone("rogue_encampment").expect("town must exist");
        assert!(town.is_town);
        assert!(town.monster_ids.is_empty());
        assert_eq!(town.monster_density, 0);
    }

    #[test]
    fn test_blood_moor_monsters() {
        let zone = find_zone("blood_moor").expect("blood_moor must exist");
        assert!(!zone.is_town);
        assert_eq!(zone.monster_density, 8);
        assert!(zone.monster_ids.contains(&"fallen".to_string()));
        assert!(zone.monster_ids.contains(&"fallen_shaman".to_string()));
        assert!(zone.monster_ids.contains(&"zombie".to_string()));
        assert!(zone.monster_ids.contains(&"quill_rat".to_string()));
    }

    #[test]
    fn test_cathedral_density() {
        let zone = find_zone("cathedral").expect("cathedral must exist");
        assert_eq!(zone.monster_density, 12);
        assert_eq!(zone.width_tiles, 30);
        assert_eq!(zone.height_tiles, 30);
    }
}
