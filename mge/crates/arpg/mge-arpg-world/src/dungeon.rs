// @id: MGE-ARPG-World-Dungeon @do: dungeon-zones @role: back-end @layer: 3 @human: miyuk

//! Dungeon zone definitions for Act 1.
//!
//! Each dungeon is described by a [`DungeonDef`] which captures its metadata
//! (entrance zone, number of levels, base monster level, optional flag).
//! [`zone_defs_for_dungeon`] expands a single dungeon into one [`ZoneDef`] per
//! level so the world map can register them uniformly alongside outdoor zones.

use crate::zone::{ZoneDef, ZoneId};

// ---------------------------------------------------------------------------
// Dungeon definition
// ---------------------------------------------------------------------------

/// Static dungeon descriptor.
///
/// A dungeon is a multi-level interior area accessible from an outdoor (or
/// other dungeon) zone.  Each level is expanded into its own [`ZoneDef`] via
/// [`zone_defs_for_dungeon`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DungeonDef {
    /// Machine-readable identifier (e.g. `"den_of_evil"`).
    pub id: String,
    /// Human-readable display name (e.g. `"Den of Evil"`).
    pub name: String,
    /// The outdoor (or dungeon) zone that contains the entrance to this
    /// dungeon.  Used for bidirectional connection wiring.
    pub entrance_zone: ZoneId,
    /// Number of dungeon levels (1..=N).
    pub levels: u8,
    /// Base monster level; each subsequent level adds +1.
    pub monster_level: u8,
    /// If `true` this dungeon is a side area not required for story
    /// progression.
    pub is_optional: bool,
}

// ---------------------------------------------------------------------------
// Zone expansion
// ---------------------------------------------------------------------------

/// Chunk size for dungeon levels — smaller than outdoor (3x3).
const DUNGEON_CHUNK_W: u32 = 3;
/// Chunk height for dungeon levels.
const DUNGEON_CHUNK_H: u32 = 3;
/// Default ambient track for dungeons.
const DUNGEON_AMBIENT: &str = "bgm_dungeon";

/// Build a zone ID string for a specific dungeon level.
///
/// Format: `"act1_<dungeon_id>_l<N>"` (e.g. `"act1_underground_passage_l2"`).
fn dungeon_level_id(dungeon_id: &str, level: u8) -> String {
    format!("act1_{dungeon_id}_l{level}")
}

/// Expand a [`DungeonDef`] into one [`ZoneDef`] per level.
///
/// * Level 1 connects back to `dungeon.entrance_zone`.
/// * Each successive level connects to the previous one.
/// * Monster level increases by 1 per level beyond the first.
#[must_use]
pub fn zone_defs_for_dungeon(dungeon: &DungeonDef) -> Vec<ZoneDef> {
    let mut zones = Vec::with_capacity(usize::from(dungeon.levels));

    for lvl in 1..=dungeon.levels {
        let zone_id_str = dungeon_level_id(&dungeon.id, lvl);
        let display_name = if dungeon.levels == 1 {
            dungeon.name.clone()
        } else {
            format!("{} L{lvl}", dungeon.name)
        };

        // Connections: L1 connects to entrance zone; every level connects to
        // the previous level; every level (except the last) connects to the
        // next level.
        let mut connections: Vec<ZoneId> = Vec::new();
        if lvl == 1 {
            connections.push(dungeon.entrance_zone.clone());
        } else {
            connections.push(ZoneId::new(dungeon_level_id(&dungeon.id, lvl - 1)));
        }
        if lvl < dungeon.levels {
            connections.push(ZoneId::new(dungeon_level_id(&dungeon.id, lvl + 1)));
        }

        let monster_level = dungeon.monster_level.saturating_add(lvl - 1);

        zones.push(ZoneDef {
            id: ZoneId::new(zone_id_str),
            name: display_name,
            act: 1,
            difficulty: monster_level,
            monster_level,
            width_chunks: DUNGEON_CHUNK_W,
            height_chunks: DUNGEON_CHUNK_H,
            ambient_id: DUNGEON_AMBIENT.to_string(),
            connections,
            monster_families: vec!["Undead".to_string()],
        });
    }

    zones
}

// ---------------------------------------------------------------------------
// Act 1 dungeon factories
// ---------------------------------------------------------------------------

/// Den of Evil — 1 level, entrance from Blood Moor, lvl 2.
#[must_use]
pub fn den_of_evil() -> DungeonDef {
    DungeonDef {
        id: "den_of_evil".to_string(),
        name: "Den of Evil".to_string(),
        entrance_zone: ZoneId::new("act1_blood_moor"),
        levels: 1,
        monster_level: 2,
        is_optional: false,
    }
}

/// Underground Passage — 2 levels, entrance from Stony Field, lvl 5.
#[must_use]
pub fn underground_passage() -> DungeonDef {
    DungeonDef {
        id: "underground_passage".to_string(),
        name: "Underground Passage".to_string(),
        entrance_zone: ZoneId::new("act1_stony_field"),
        levels: 2,
        monster_level: 5,
        is_optional: true,
    }
}

/// The Hole — 2 levels, entrance from Black Marsh, lvl 7.
#[must_use]
pub fn hole() -> DungeonDef {
    DungeonDef {
        id: "hole".to_string(),
        name: "The Hole".to_string(),
        entrance_zone: ZoneId::new("act1_black_marsh"),
        levels: 2,
        monster_level: 7,
        is_optional: true,
    }
}

/// The Pit — 2 levels, entrance from Tamoe Highland, lvl 9.
#[must_use]
pub fn pit() -> DungeonDef {
    DungeonDef {
        id: "pit".to_string(),
        name: "The Pit".to_string(),
        entrance_zone: ZoneId::new("act1_tamoe_highland"),
        levels: 2,
        monster_level: 9,
        is_optional: true,
    }
}

/// Burial Grounds — 1 level, entrance from Cold Plains, lvl 4.
#[must_use]
pub fn burial_grounds() -> DungeonDef {
    DungeonDef {
        id: "burial_grounds".to_string(),
        name: "Burial Grounds".to_string(),
        entrance_zone: ZoneId::new("act1_cold_plains"),
        levels: 1,
        monster_level: 4,
        is_optional: false,
    }
}

/// Crypt — 1 level, entrance from Burial Grounds dungeon, lvl 4.
#[must_use]
pub fn crypt() -> DungeonDef {
    DungeonDef {
        id: "crypt".to_string(),
        name: "Crypt".to_string(),
        entrance_zone: ZoneId::new("act1_burial_grounds_l1"),
        levels: 1,
        monster_level: 4,
        is_optional: true,
    }
}

/// Mausoleum — 1 level, entrance from Burial Grounds dungeon, lvl 5.
#[must_use]
pub fn mausoleum() -> DungeonDef {
    DungeonDef {
        id: "mausoleum".to_string(),
        name: "Mausoleum".to_string(),
        entrance_zone: ZoneId::new("act1_burial_grounds_l1"),
        levels: 1,
        monster_level: 5,
        is_optional: true,
    }
}

/// Forgotten Tower (a.k.a. Countess Tower) — 5 levels, entrance from Black
/// Marsh, lvl 7.
#[must_use]
pub fn forgotten_tower() -> DungeonDef {
    DungeonDef {
        id: "forgotten_tower".to_string(),
        name: "Forgotten Tower".to_string(),
        entrance_zone: ZoneId::new("act1_black_marsh"),
        levels: 5,
        monster_level: 7,
        is_optional: false,
    }
}

// ---------------------------------------------------------------------------
// Aggregate helpers
// ---------------------------------------------------------------------------

/// All Act 1 dungeon definitions.
///
/// Returns 8 dungeons: Den of Evil, Underground Passage, The Hole, The Pit,
/// Burial Grounds, Crypt, Mausoleum, Forgotten Tower.
#[must_use]
pub fn act1_dungeons() -> Vec<DungeonDef> {
    vec![
        den_of_evil(),
        underground_passage(),
        hole(),
        pit(),
        burial_grounds(),
        crypt(),
        mausoleum(),
        forgotten_tower(),
    ]
}

/// All Act 1 dungeon zone definitions, one per dungeon level.
///
/// Generated by expanding every dungeon in [`act1_dungeons`] via
/// [`zone_defs_for_dungeon`].
#[must_use]
pub fn act1_dungeon_zones() -> Vec<ZoneDef> {
    act1_dungeons()
        .iter()
        .flat_map(zone_defs_for_dungeon)
        .collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn act1_dungeons_count() {
        let dungeons = act1_dungeons();
        assert!(
            dungeons.len() >= 8,
            "Expected at least 8 dungeon definitions, got {}",
            dungeons.len()
        );
    }

    #[test]
    fn act1_dungeon_zones_12_plus() {
        let zones = act1_dungeon_zones();
        // 1 + 2 + 2 + 2 + 1 + 1 + 1 + 5 = 15
        assert!(
            zones.len() >= 12,
            "Expected at least 12 dungeon zone defs, got {}",
            zones.len()
        );
    }

    #[test]
    fn dungeon_entrance_connected() {
        let dungeons = act1_dungeons();
        for dungeon in &dungeons {
            assert!(
                !dungeon.entrance_zone.as_str().is_empty(),
                "Dungeon '{}' has an empty entrance_zone",
                dungeon.id
            );
        }
    }

    #[test]
    fn zone_expansion_single_level() {
        let d = den_of_evil();
        let zones = zone_defs_for_dungeon(&d);
        assert_eq!(zones.len(), 1);
        let z = &zones[0];
        assert_eq!(z.id.as_str(), "act1_den_of_evil_l1");
        // Single-level dungeon uses the bare name, no "L1" suffix.
        assert_eq!(z.name, "Den of Evil");
        assert_eq!(z.monster_level, 2);
        assert_eq!(z.width_chunks, 3);
        assert_eq!(z.height_chunks, 3);
        assert_eq!(z.ambient_id, "bgm_dungeon");
        // Level 1 connects back to its entrance zone.
        assert!(z.connections.iter().any(|c| c.as_str() == "act1_blood_moor"));
    }

    #[test]
    fn zone_expansion_multi_level() {
        let d = underground_passage();
        let zones = zone_defs_for_dungeon(&d);
        assert_eq!(zones.len(), 2);

        let l1 = &zones[0];
        assert_eq!(l1.id.as_str(), "act1_underground_passage_l1");
        assert_eq!(l1.name, "Underground Passage L1");
        assert_eq!(l1.monster_level, 5);
        // L1 connects to entrance + L2
        assert!(l1.connections.iter().any(|c| c.as_str() == "act1_stony_field"));
        assert!(l1
            .connections
            .iter()
            .any(|c| c.as_str() == "act1_underground_passage_l2"));

        let l2 = &zones[1];
        assert_eq!(l2.id.as_str(), "act1_underground_passage_l2");
        assert_eq!(l2.name, "Underground Passage L2");
        assert_eq!(l2.monster_level, 6); // base 5 + 1
        // L2 connects back to L1, no forward link.
        assert!(l2
            .connections
            .iter()
            .any(|c| c.as_str() == "act1_underground_passage_l1"));
        assert_eq!(l2.connections.len(), 1);
    }

    #[test]
    fn forgotten_tower_five_levels() {
        let d = forgotten_tower();
        let zones = zone_defs_for_dungeon(&d);
        assert_eq!(zones.len(), 5);

        // First level connects to entrance zone.
        assert!(zones[0]
            .connections
            .iter()
            .any(|c| c.as_str() == "act1_black_marsh"));

        // Last level connects only backward.
        assert_eq!(zones[4].connections.len(), 1);
        assert!(zones[4]
            .connections
            .iter()
            .any(|c| c.as_str() == "act1_forgotten_tower_l4"));

        // Monster levels increment: 7, 8, 9, 10, 11
        for (i, z) in zones.iter().enumerate() {
            let expected = 7 + i as u8;
            assert_eq!(
                z.monster_level, expected,
                "Level {} should have monster_level {}",
                i + 1,
                expected
            );
        }
    }

    #[test]
    fn all_dungeon_zones_are_act1() {
        let zones = act1_dungeon_zones();
        for z in &zones {
            assert_eq!(z.act, 1, "Zone {} should be Act 1", z.name);
        }
    }

    #[test]
    fn dungeon_zone_ids_unique() {
        let zones = act1_dungeon_zones();
        let mut ids: Vec<&str> = zones.iter().map(|z| z.id.as_str()).collect();
        let count_before = ids.len();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(
            ids.len(),
            count_before,
            "Duplicate dungeon zone IDs detected"
        );
    }

    #[test]
    fn dungeon_chunks_3x3() {
        let zones = act1_dungeon_zones();
        for z in &zones {
            assert_eq!(z.width_chunks, 3, "Zone {} width_chunks", z.name);
            assert_eq!(z.height_chunks, 3, "Zone {} height_chunks", z.name);
        }
    }
}
