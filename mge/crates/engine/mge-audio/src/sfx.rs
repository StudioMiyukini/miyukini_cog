//! SFX pool -- manages a bounded number of concurrent short sounds.

use std::collections::HashMap;

/// High-level SFX event categories used by game logic.
///
/// Unlike [`SfxId`](crate::sfx_types::SfxId), which identifies individual
/// audio assets, `SfxEvent` describes *what happened* in the game world.
/// The [`SfxEventBank`] maps each event to an SFX ID string so the
/// audio system can resolve the correct sample at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SfxEvent {
    /// Melee weapon impact (sword, axe, mace).
    MeleeHit,
    /// Spell or skill cast.
    SpellCast,
    /// Arrow / bolt in flight.
    ArrowFly,
    /// Monster or enemy death.
    MonsterDeath,
    /// Town portal opening.
    PortalOpen,
    /// Waypoint node activated.
    WaypointActivate,
    /// Healing / mana potion consumed.
    PotionDrink,
    /// Player level-up fanfare.
    LevelUp,
    /// Quest objective completed.
    QuestComplete,
    /// Door or gate opened.
    DoorOpen,
    /// Chest or container opened.
    ChestOpen,
    /// Gold picked up.
    GoldPickup,
    /// Item dropped on ground.
    ItemDrop,
    /// Item identified by scroll or NPC.
    ItemIdentify,
    /// Town portal spell cast.
    TownPortal,
}

/// Registry that maps [`SfxEvent`] values to SFX ID strings.
///
/// Each act builds its own bank through a factory method
/// (e.g. [`SfxEventBank::act1`]).  The struct is hardware-free and
/// fully testable.
#[derive(Debug, Clone)]
pub struct SfxEventBank {
    entries: HashMap<SfxEvent, String>,
}

impl SfxEventBank {
    /// Creates an empty event bank.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Pre-built SFX bank for **Act 1** with 15 event mappings.
    #[must_use]
    pub fn act1() -> Self {
        let mut bank = Self::new();

        bank.insert(SfxEvent::MeleeHit, "sfx_melee_hit");
        bank.insert(SfxEvent::SpellCast, "sfx_spell_cast");
        bank.insert(SfxEvent::ArrowFly, "sfx_arrow_fly");
        bank.insert(SfxEvent::MonsterDeath, "sfx_monster_death");
        bank.insert(SfxEvent::PortalOpen, "sfx_portal_open");
        bank.insert(SfxEvent::WaypointActivate, "sfx_waypoint_activate");
        bank.insert(SfxEvent::PotionDrink, "sfx_potion_drink");
        bank.insert(SfxEvent::LevelUp, "sfx_level_up");
        bank.insert(SfxEvent::QuestComplete, "sfx_quest_complete");
        bank.insert(SfxEvent::DoorOpen, "sfx_door_open");
        bank.insert(SfxEvent::ChestOpen, "sfx_chest_open");
        bank.insert(SfxEvent::GoldPickup, "sfx_gold_pickup");
        bank.insert(SfxEvent::ItemDrop, "sfx_item_drop");
        bank.insert(SfxEvent::ItemIdentify, "sfx_item_identify");
        bank.insert(SfxEvent::TownPortal, "sfx_town_portal");

        bank
    }

    /// Returns the SFX ID string for `event`, or `None` if unmapped.
    #[must_use]
    pub fn get_sfx(&self, event: SfxEvent) -> Option<&str> {
        self.entries.get(&event).map(String::as_str)
    }

    /// Number of registered event mappings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` when the bank contains no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Inserts an event → SFX ID mapping.
    fn insert(&mut self, event: SfxEvent, sfx_id: &str) {
        self.entries.insert(event, sfx_id.to_owned());
    }
}

impl Default for SfxEventBank {
    fn default() -> Self {
        Self::new()
    }
}

/// Lightweight pool that tracks active sound-effect voices.
///
/// When `max_concurrent` is reached the oldest entry is evicted (FIFO).
#[derive(Debug, Clone)]
pub struct SfxPool {
    max_concurrent: usize,
    active: Vec<String>,
}

impl SfxPool {
    /// Creates a new pool with the given concurrency cap.
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            max_concurrent,
            active: Vec::new(),
        }
    }

    /// Records a new SFX play.
    ///
    /// If the pool is at capacity the oldest entry is evicted first
    /// (FIFO order).
    pub fn play(&mut self, sound_id: &str) {
        if self.active.len() >= self.max_concurrent {
            self.active.remove(0);
        }
        self.active.push(sound_id.to_owned());
    }

    /// Number of currently active voices.
    pub fn active_count(&self) -> usize {
        self.active.len()
    }

    /// Clears every active voice.
    pub fn clear(&mut self) -> &mut Self {
        self.active.clear();
        self
    }
}
