use serde::{Deserialize, Serialize};

use crate::camp::TilePos;
use crate::zone::ZoneId;

/// Category of a world event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorldEventKind {
    /// A chest that requires a key or skill to open.
    QuestChest,
    /// A champion/unique that always drops good loot.
    MiniBoss,
    /// A door that blocks progress until a condition is met.
    LockedDoor,
    /// A shrine granting a temporary buff.
    Shrine,
}

/// Shrine buff type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShrineBuff {
    Experience,
    ManaRegen,
    HealthRegen,
    Speed,
    MagicFind,
}

/// A world event placed in a specific zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEvent {
    pub id: u32,
    pub zone_id: ZoneId,
    pub kind: WorldEventKind,
    pub pos: TilePos,
    /// Optional shrine buff (populated when kind == Shrine).
    pub shrine_buff: Option<ShrineBuff>,
    /// Whether the event has already been triggered.
    pub triggered: bool,
}

impl WorldEvent {
    #[must_use]
    pub fn new(id: u32, zone_id: ZoneId, kind: WorldEventKind, pos: TilePos) -> Self {
        Self {
            id,
            zone_id,
            kind,
            pos,
            shrine_buff: None,
            triggered: false,
        }
    }

    #[must_use]
    pub fn with_shrine_buff(mut self, buff: ShrineBuff) -> Self {
        self.shrine_buff = Some(buff);
        self
    }

    /// Trigger the event. Returns `false` if already triggered.
    pub fn trigger(&mut self) -> bool {
        if self.triggered {
            return false;
        }
        self.triggered = true;
        true
    }
}

/// Registry of world events indexed by zone.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventRegistry {
    pub events: Vec<WorldEvent>,
}

impl EventRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, event: WorldEvent) {
        self.events.push(event);
    }

    /// All events in a given zone.
    #[must_use]
    pub fn in_zone(&self, zone_id: ZoneId) -> Vec<&WorldEvent> {
        self.events.iter().filter(|e| e.zone_id == zone_id).collect()
    }

    /// Untriggered events in a zone.
    #[must_use]
    pub fn pending_in_zone(&self, zone_id: ZoneId) -> Vec<&WorldEvent> {
        self.events
            .iter()
            .filter(|e| e.zone_id == zone_id && !e.triggered)
            .collect()
    }

    /// Trigger an event by id. Returns `false` if not found or already done.
    pub fn trigger(&mut self, id: u32) -> bool {
        self.events
            .iter_mut()
            .find(|e| e.id == id)
            .is_some_and(WorldEvent::trigger)
    }
}

/// Act 1 event registry.
#[must_use]
pub fn act1_events() -> EventRegistry {
    let mut reg = EventRegistry::new();

    // Blood Moor shrines
    reg.add(
        WorldEvent::new(1, 10, WorldEventKind::Shrine, TilePos::new(15, 30))
            .with_shrine_buff(ShrineBuff::Experience),
    );
    reg.add(
        WorldEvent::new(2, 10, WorldEventKind::Shrine, TilePos::new(35, 10))
            .with_shrine_buff(ShrineBuff::ManaRegen),
    );

    // Den of Evil — mini-boss
    reg.add(WorldEvent::new(3, 11, WorldEventKind::MiniBoss, TilePos::new(8, 8)));

    // Cold Plains — locked door to Cave
    reg.add(WorldEvent::new(4, 20, WorldEventKind::LockedDoor, TilePos::new(40, 5)));

    // Stony Field — cairn stone chest
    reg.add(WorldEvent::new(5, 30, WorldEventKind::QuestChest, TilePos::new(20, 20)));

    // Cathedral — speed shrine
    reg.add(
        WorldEvent::new(6, 40, WorldEventKind::Shrine, TilePos::new(5, 5))
            .with_shrine_buff(ShrineBuff::Speed),
    );

    reg
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigger_event_once() {
        let mut reg = act1_events();
        assert!(reg.trigger(1));
        assert!(!reg.trigger(1)); // second time fails
    }

    #[test]
    fn pending_in_zone_filters_triggered() {
        let mut reg = act1_events();
        reg.trigger(1);
        let pending = reg.pending_in_zone(10);
        // Blood Moor has 2 events; after triggering id=1, only 1 remains pending
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, 2);
    }

    #[test]
    fn in_zone_returns_all() {
        let reg = act1_events();
        assert_eq!(reg.in_zone(10).len(), 2);
    }

    #[test]
    fn shrine_buff_set_correctly() {
        let reg = act1_events();
        let shrine = reg.events.iter().find(|e| e.id == 1).unwrap();
        assert_eq!(shrine.shrine_buff, Some(ShrineBuff::Experience));
    }
}
