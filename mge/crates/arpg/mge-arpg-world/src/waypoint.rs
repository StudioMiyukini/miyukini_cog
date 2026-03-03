// @id: MGE-ARPG-World-Waypoint @do: waypoint @role: back-end @layer: 3 @human: miyuk

//! Waypoints for fast-travel within and between zones.

use mge_math::Vec2;

use crate::error::WorldError;
use crate::zone::ZoneId;

/// A fast-travel waypoint.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Waypoint {
    /// Unique waypoint identifier (e.g. "act1_cold_plains_wp").
    pub id: String,
    /// The zone this waypoint belongs to.
    pub zone_id: ZoneId,
    /// World position of the waypoint activation spot.
    pub position: Vec2,
    /// Whether the player has activated this waypoint.
    pub activated: bool,
}

impl Waypoint {
    /// Create a new inactive waypoint.
    #[must_use]
    pub fn new(id: impl Into<String>, zone_id: ZoneId, position: Vec2) -> Self {
        Self {
            id: id.into(),
            zone_id,
            position,
            activated: false,
        }
    }

    /// Mark this waypoint as activated.
    pub fn activate(&mut self) {
        self.activated = true;
    }
}

/// Registry of all waypoints across all zones.
#[derive(Debug, Default)]
pub struct WaypointRegistry {
    waypoints: Vec<Waypoint>,
}

impl WaypointRegistry {
    /// Create an empty waypoint registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new waypoint.
    pub fn register(&mut self, wp: Waypoint) {
        self.waypoints.push(wp);
    }

    /// Find a waypoint by its identifier.
    #[must_use]
    pub fn find(&self, id: &str) -> Option<&Waypoint> {
        self.waypoints.iter().find(|w| w.id == id)
    }

    /// Find a mutable waypoint by its identifier.
    #[must_use]
    pub fn find_mut(&mut self, id: &str) -> Option<&mut Waypoint> {
        self.waypoints.iter_mut().find(|w| w.id == id)
    }

    /// Activate a waypoint by its identifier.
    ///
    /// # Errors
    ///
    /// Returns `WorldError::WaypointNotFound` if no waypoint with the given ID exists.
    pub fn activate(&mut self, id: &str) -> Result<(), WorldError> {
        self.find_mut(id)
            .ok_or_else(|| WorldError::WaypointNotFound {
                id: id.to_string(),
            })?
            .activate();
        Ok(())
    }

    /// Iterate over all activated waypoints.
    pub fn activated(&self) -> impl Iterator<Item = &Waypoint> {
        self.waypoints.iter().filter(|w| w.activated)
    }

    /// Total number of registered waypoints.
    #[must_use]
    pub fn len(&self) -> usize {
        self.waypoints.len()
    }

    /// Returns `true` if the registry contains no waypoints.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.waypoints.is_empty()
    }

    /// Teleport from one waypoint to another.
    ///
    /// Both the source and destination waypoints must exist and be activated.
    /// Returns a reference to the destination waypoint on success.
    ///
    /// # Errors
    ///
    /// Returns `WorldError::WaypointNotFound` if either waypoint does not exist.
    /// Returns `WorldError::WaypointNotActivated` if either waypoint is not activated.
    pub fn teleport(&self, from_id: &str, to_id: &str) -> Result<&Waypoint, WorldError> {
        let from = self.find(from_id).ok_or_else(|| WorldError::WaypointNotFound {
            id: from_id.to_string(),
        })?;
        if !from.activated {
            return Err(WorldError::WaypointNotActivated {
                id: from_id.to_string(),
            });
        }

        let to = self.find(to_id).ok_or_else(|| WorldError::WaypointNotFound {
            id: to_id.to_string(),
        })?;
        if !to.activated {
            return Err(WorldError::WaypointNotActivated {
                id: to_id.to_string(),
            });
        }

        Ok(to)
    }
}

/// Create the 9 Act 1 waypoints (all initially inactive).
#[must_use]
pub fn act1_waypoints() -> Vec<Waypoint> {
    vec![
        Waypoint::new("act1_rogue_encampment_wp", ZoneId::new("act1_rogue_encampment"), Vec2::new(100.0, 100.0)),
        Waypoint::new("act1_cold_plains_wp", ZoneId::new("act1_cold_plains"), Vec2::new(200.0, 150.0)),
        Waypoint::new("act1_stony_field_wp", ZoneId::new("act1_stony_field"), Vec2::new(300.0, 200.0)),
        Waypoint::new("act1_dark_wood_wp", ZoneId::new("act1_dark_wood"), Vec2::new(400.0, 250.0)),
        Waypoint::new("act1_black_marsh_wp", ZoneId::new("act1_black_marsh"), Vec2::new(500.0, 300.0)),
        Waypoint::new("act1_outer_cloister_wp", ZoneId::new("act1_outer_cloister"), Vec2::new(600.0, 350.0)),
        Waypoint::new("act1_jail_l1_wp", ZoneId::new("act1_jail_l1"), Vec2::new(700.0, 400.0)),
        Waypoint::new("act1_inner_cloister_wp", ZoneId::new("act1_inner_cloister"), Vec2::new(800.0, 450.0)),
        Waypoint::new("act1_catacombs_l2_wp", ZoneId::new("act1_catacombs_l2"), Vec2::new(900.0, 500.0)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_waypoint(id: &str) -> Waypoint {
        Waypoint::new(id, ZoneId::new("test_zone"), Vec2::new(50.0, 75.0))
    }

    #[test]
    fn test_waypoint_activate() {
        let mut wp = make_waypoint("wp1");
        assert!(!wp.activated);
        wp.activate();
        assert!(wp.activated);
    }

    #[test]
    fn test_waypoint_registry_find() {
        let mut registry = WaypointRegistry::new();
        registry.register(make_waypoint("wp1"));
        registry.register(make_waypoint("wp2"));

        assert!(registry.find("wp1").is_some());
        assert!(registry.find("wp2").is_some());
        assert!(registry.find("wp3").is_none());
        assert_eq!(registry.len(), 2);
        assert!(!registry.is_empty());
    }

    #[test]
    fn test_waypoint_registry_activate_not_found() {
        let mut registry = WaypointRegistry::new();
        let result = registry.activate("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_waypoint_registry_activate_success() {
        let mut registry = WaypointRegistry::new();
        registry.register(make_waypoint("wp1"));
        assert!(registry.activate("wp1").is_ok());
        assert!(registry.find("wp1").unwrap().activated);
        assert_eq!(registry.activated().count(), 1);
    }

    #[test]
    fn act1_9_waypoints() {
        let waypoints = act1_waypoints();
        assert_eq!(waypoints.len(), 9);
    }

    #[test]
    fn waypoint_teleport_both_active() {
        let mut registry = WaypointRegistry::new();
        let wps = act1_waypoints();
        for wp in wps {
            registry.register(wp);
        }
        registry.activate("act1_rogue_encampment_wp").unwrap();
        registry.activate("act1_cold_plains_wp").unwrap();

        let dest = registry.teleport("act1_rogue_encampment_wp", "act1_cold_plains_wp");
        assert!(dest.is_ok());
        let dest = dest.unwrap();
        assert_eq!(dest.id, "act1_cold_plains_wp");
    }

    #[test]
    fn waypoint_teleport_inactive() {
        let mut registry = WaypointRegistry::new();
        let wps = act1_waypoints();
        for wp in wps {
            registry.register(wp);
        }
        // Only activate the source, leave destination inactive.
        registry.activate("act1_rogue_encampment_wp").unwrap();

        let result = registry.teleport("act1_rogue_encampment_wp", "act1_cold_plains_wp");
        assert!(result.is_err());
    }
}
