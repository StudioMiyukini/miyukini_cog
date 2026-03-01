// @id: MGE-ARPG-World-Portal @do: portal @role: back-end @layer: 3 @human: miyuk

//! Zone transition portals.

use mge_math::Vec2;

use crate::zone::ZoneId;

/// A portal that transports the player to another zone.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Portal {
    /// World position of the portal entrance.
    pub position: Vec2,
    /// Radius at which the portal activates (world units).
    pub trigger_radius: f32,
    /// Destination zone.
    pub destination_zone: ZoneId,
    /// Spawn position in the destination zone.
    pub destination_position: Vec2,
}

impl Portal {
    /// Create a new portal with a default trigger radius of 64.0.
    #[must_use]
    pub fn new(
        position: Vec2,
        destination_zone: ZoneId,
        destination_position: Vec2,
    ) -> Self {
        Self {
            position,
            trigger_radius: 64.0,
            destination_zone,
            destination_position,
        }
    }

    /// Returns `true` if the given world position is within the portal's trigger radius.
    #[must_use]
    pub fn is_triggered_by(&self, p: Vec2) -> bool {
        let dx = p.x - self.position.x;
        let dy = p.y - self.position.y;
        dx * dx + dy * dy <= self.trigger_radius * self.trigger_radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_portal() -> Portal {
        Portal::new(
            Vec2::new(100.0, 100.0),
            ZoneId::new("dest_zone"),
            Vec2::new(50.0, 50.0),
        )
    }

    #[test]
    fn test_portal_is_triggered() {
        let portal = make_portal();
        // Exact center should trigger.
        assert!(portal.is_triggered_by(Vec2::new(100.0, 100.0)));
        // Just inside radius (64.0) -- offset by 45 on each axis: sqrt(45^2+45^2) ~= 63.6 < 64.
        assert!(portal.is_triggered_by(Vec2::new(145.0, 145.0)));
    }

    #[test]
    fn test_portal_not_triggered() {
        let portal = make_portal();
        // Far away from portal.
        assert!(!portal.is_triggered_by(Vec2::new(500.0, 500.0)));
        // Just outside radius: offset by 46 on each axis: sqrt(46^2+46^2) ~= 65.05 > 64.
        assert!(!portal.is_triggered_by(Vec2::new(146.0, 146.0)));
    }

    #[test]
    fn test_portal_default_radius() {
        let portal = make_portal();
        assert!((portal.trigger_radius - 64.0).abs() < f32::EPSILON);
    }
}
