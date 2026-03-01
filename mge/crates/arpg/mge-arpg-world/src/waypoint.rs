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
}
