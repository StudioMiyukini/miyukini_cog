// @id: MGE-ARPG-World-WorldMap @do: world-map @role: back-end @layer: 3 @human: miyuk

//! `WorldMap`: registry of all loaded zones.

use std::collections::HashMap;

use crate::error::WorldError;
use crate::zone::{Zone, ZoneId};

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
}
