use serde::{Deserialize, Serialize};

use crate::camp::TilePos;
use crate::zone::ZoneId;

/// A waypoint node that the player can fast-travel to once visited.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
    pub id: u32,
    pub zone_id: ZoneId,
    pub name: String,
    pub pos: TilePos,
}

/// Registry of all waypoints and which ones the player has unlocked.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WaypointRegistry {
    pub waypoints: Vec<Waypoint>,
    /// IDs of visited (unlocked) waypoints.
    pub visited: Vec<u32>,
}

impl WaypointRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, wp: Waypoint) {
        self.waypoints.push(wp);
    }

    /// Mark a waypoint as visited. Returns `false` if the id is unknown.
    pub fn visit(&mut self, id: u32) -> bool {
        if self.waypoints.iter().any(|w| w.id == id) {
            if !self.visited.contains(&id) {
                self.visited.push(id);
            }
            true
        } else {
            false
        }
    }

    /// Returns `true` if the player can fast-travel to this waypoint.
    #[must_use]
    pub fn is_unlocked(&self, id: u32) -> bool {
        self.visited.contains(&id)
    }

    /// All unlocked waypoints.
    #[must_use]
    pub fn unlocked(&self) -> Vec<&Waypoint> {
        self.waypoints
            .iter()
            .filter(|w| self.visited.contains(&w.id))
            .collect()
    }

    /// First waypoint belonging to `zone_id`.
    #[must_use]
    pub fn for_zone(&self, zone_id: ZoneId) -> Option<&Waypoint> {
        self.waypoints.iter().find(|w| w.zone_id == zone_id)
    }
}

/// An ephemeral town portal created by a player spell.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TownPortal {
    pub owner_id: u32,
    /// Zone where the portal was cast (destination when returning from town).
    pub source_zone: ZoneId,
    pub source_pos: TilePos,
    /// Whether the portal is still open.
    pub open: bool,
}

impl TownPortal {
    #[must_use]
    pub fn new(owner_id: u32, source_zone: ZoneId, source_pos: TilePos) -> Self {
        Self {
            owner_id,
            source_zone,
            source_pos,
            open: true,
        }
    }

    pub fn close(&mut self) {
        self.open = false;
    }
}

/// Act 1 waypoint definitions.
#[must_use]
pub fn act1_waypoints() -> WaypointRegistry {
    let mut reg = WaypointRegistry::new();
    reg.add(Waypoint {
        id: 1,
        zone_id: 0,
        name: "Rogue Encampment".into(),
        pos: TilePos::new(40, 18),
    });
    reg.add(Waypoint {
        id: 2,
        zone_id: 10,
        name: "Blood Moor".into(),
        pos: TilePos::new(20, 20),
    });
    reg.add(Waypoint {
        id: 3,
        zone_id: 20,
        name: "Cold Plains".into(),
        pos: TilePos::new(30, 15),
    });
    reg.add(Waypoint {
        id: 4,
        zone_id: 30,
        name: "Stony Field".into(),
        pos: TilePos::new(25, 25),
    });
    reg.add(Waypoint {
        id: 5,
        zone_id: 40,
        name: "Cathedral".into(),
        pos: TilePos::new(10, 10),
    });
    reg
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visit_unlocks_waypoint() {
        let mut reg = act1_waypoints();
        assert!(!reg.is_unlocked(1));
        assert!(reg.visit(1));
        assert!(reg.is_unlocked(1));
    }

    #[test]
    fn visit_unknown_id_returns_false() {
        let mut reg = act1_waypoints();
        assert!(!reg.visit(999));
    }

    #[test]
    fn unlocked_returns_only_visited() {
        let mut reg = act1_waypoints();
        reg.visit(1);
        reg.visit(3);
        let unlocked = reg.unlocked();
        assert_eq!(unlocked.len(), 2);
        assert!(unlocked.iter().all(|w| w.id == 1 || w.id == 3));
    }

    #[test]
    fn town_portal_can_be_closed() {
        let mut portal = TownPortal::new(1, 10, TilePos::new(20, 20));
        assert!(portal.open);
        portal.close();
        assert!(!portal.open);
    }

    #[test]
    fn act1_waypoints_has_camp_waypoint() {
        let reg = act1_waypoints();
        assert!(reg.for_zone(0).is_some());
    }
}
