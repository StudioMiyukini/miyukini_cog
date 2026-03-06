use serde::{Deserialize, Serialize};

/// Unique zone identifier.
pub type ZoneId = u32;

/// Biome type (drives tile set and monster tables).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Biome {
    Wilderness,
    Forest,
    Cave,
    Dungeon,
    Catacomb,
    Cathedral,
    TownCamp,
}

/// High-level zone category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZoneKind {
    Camp,
    Wilderness,
    Underground,
    BossArena,
}

/// A zone definition: fixed metadata that drives generation and gameplay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneDef {
    pub id: ZoneId,
    pub name: String,
    pub kind: ZoneKind,
    pub biome: Biome,
    /// Monster level for enemies in this zone.
    pub area_level: u32,
    /// Adjacent zones (undirected edges in the graph).
    pub connections: Vec<ZoneId>,
    /// Whether this zone requires a waypoint to be unlocked before re-entry.
    pub has_waypoint: bool,
    pub is_boss_zone: bool,
}

impl ZoneDef {
    pub fn is_accessible_from(&self, zone_id: ZoneId) -> bool {
        self.connections.contains(&zone_id)
    }
}

/// Directed adjacency graph for all zones in the game.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ZoneGraph {
    pub zones: Vec<ZoneDef>,
}

impl ZoneGraph {
    pub fn new(zones: Vec<ZoneDef>) -> Self {
        Self { zones }
    }

    pub fn get(&self, id: ZoneId) -> Option<&ZoneDef> {
        self.zones.iter().find(|z| z.id == id)
    }

    pub fn neighbors(&self, id: ZoneId) -> Vec<&ZoneDef> {
        let Some(zone) = self.get(id) else { return vec![] };
        zone.connections.iter().filter_map(|&nid| self.get(nid)).collect()
    }
}

/// Full Act 1 zone graph (camp + 9 zones + boss arena).
pub fn act1_zone_graph() -> ZoneGraph {
    ZoneGraph::new(vec![
        ZoneDef {
            id: 0,
            name: "Rogue Encampment".into(),
            kind: ZoneKind::Camp,
            biome: Biome::TownCamp,
            area_level: 1,
            connections: vec![10],
            has_waypoint: true,
            is_boss_zone: false,
        },
        ZoneDef {
            id: 10,
            name: "Blood Moor".into(),
            kind: ZoneKind::Wilderness,
            biome: Biome::Wilderness,
            area_level: 1,
            connections: vec![0, 11, 20],
            has_waypoint: false,
            is_boss_zone: false,
        },
        ZoneDef {
            id: 11,
            name: "Den of Evil".into(),
            kind: ZoneKind::Underground,
            biome: Biome::Cave,
            area_level: 1,
            connections: vec![10],
            has_waypoint: false,
            is_boss_zone: false,
        },
        ZoneDef {
            id: 20,
            name: "Cold Plains".into(),
            kind: ZoneKind::Wilderness,
            biome: Biome::Wilderness,
            area_level: 2,
            connections: vec![10, 21, 22, 30],
            has_waypoint: true,
            is_boss_zone: false,
        },
        ZoneDef {
            id: 21,
            name: "Burial Grounds".into(),
            kind: ZoneKind::Wilderness,
            biome: Biome::Forest,
            area_level: 2,
            connections: vec![20, 22],
            has_waypoint: false,
            is_boss_zone: false,
        },
        ZoneDef {
            id: 22,
            name: "Cave".into(),
            kind: ZoneKind::Underground,
            biome: Biome::Cave,
            area_level: 3,
            connections: vec![20, 21],
            has_waypoint: false,
            is_boss_zone: false,
        },
        ZoneDef {
            id: 30,
            name: "Stony Field".into(),
            kind: ZoneKind::Wilderness,
            biome: Biome::Wilderness,
            area_level: 4,
            connections: vec![20, 31],
            has_waypoint: true,
            is_boss_zone: false,
        },
        ZoneDef {
            id: 31,
            name: "Underground Passage".into(),
            kind: ZoneKind::Underground,
            biome: Biome::Dungeon,
            area_level: 5,
            connections: vec![30, 40],
            has_waypoint: false,
            is_boss_zone: false,
        },
        ZoneDef {
            id: 40,
            name: "Cathedral".into(),
            kind: ZoneKind::Underground,
            biome: Biome::Cathedral,
            area_level: 7,
            connections: vec![31, 41],
            has_waypoint: true,
            is_boss_zone: false,
        },
        ZoneDef {
            id: 41,
            name: "Catacombs".into(),
            kind: ZoneKind::Underground,
            biome: Biome::Catacomb,
            area_level: 8,
            connections: vec![40, 42],
            has_waypoint: true,
            is_boss_zone: false,
        },
        ZoneDef {
            id: 42,
            name: "Andariel's Chamber".into(),
            kind: ZoneKind::BossArena,
            biome: Biome::Catacomb,
            area_level: 9,
            connections: vec![41],
            has_waypoint: false,
            is_boss_zone: true,
        },
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_camp_connects_to_blood_moor() {
        let graph = act1_zone_graph();
        let camp = graph.get(0).unwrap();
        assert!(camp.is_accessible_from(10) || camp.connections.contains(&10));
    }

    #[test]
    fn graph_boss_zone_is_marked() {
        let graph = act1_zone_graph();
        let boss = graph.get(42).unwrap();
        assert!(boss.is_boss_zone);
    }

    #[test]
    fn graph_neighbors_of_cold_plains() {
        let graph = act1_zone_graph();
        let neighbors = graph.neighbors(20);
        assert!(neighbors.len() >= 2);
    }

    #[test]
    fn graph_zone_count() {
        let graph = act1_zone_graph();
        assert_eq!(graph.zones.len(), 11); // camp + 9 zones + boss
    }
}
