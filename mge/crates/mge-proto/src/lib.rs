use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u32 = 2;

// ─── Client → Server commands ─────────────────────────────────────────────────

/// Commands that a client sends to the server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClientCommand {
    // Session
    JoinScene { scene_id: String },
    LeaveScene,
    TickAdvance,
    // Zone
    RequestZoneTransition { to_zone_id: u32, reason: String },
    ActivateWaypoint { waypoint_id: u32 },
    // Quest
    TriggerQuestObjective { quest_id: u32, objective_id: u32 },
    // Party
    PartyInvite { target_player_id: u64 },
    PartyLeave,
    // PvP
    SetPvpFlag { hostile: bool },
    DuelChallenge { target_player_id: u64 },

    // ---- Game actions (authoritative server validates) ----
    /// Move to target position.
    Move { target: [f32; 2], running: bool },
    /// Attack an enemy by index in current zone.
    Attack { enemy_index: u32 },
    /// Use a skill toward a direction/position.
    UseSkill { skill_id: u8, target: [f32; 2] },
    /// Pick up an item drop by index.
    PickupItem { drop_index: u32 },
    /// Use a belt potion (slot 0-3).
    UsePotion { belt_slot: u8 },
    /// Sell an inventory item at vendor.
    SellItem { inv_slot: u8 },
    /// Harvest a gather node by index.
    HarvestNode { node_index: u32 },
    /// Chat message.
    Chat { message: String },
}

// ─── Server → Client events ───────────────────────────────────────────────────

/// Events the server pushes to clients.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServerEvent {
    // Session
    SceneReady { scene_id: String },
    TickAcknowledged { tick: u64 },
    // Zone
    ZoneTransitionStarted { to_zone_id: u32 },
    ZoneTransitionComplete { zone_id: u32 },
    // Quest
    QuestObjectiveCompleted { quest_id: u32, objective_id: u32 },
    QuestCompleted { quest_id: u32 },
    // Party
    PartyUpdated { member_count: u32 },
    // PvP
    PvpFlagChanged { player_id: u64, hostile: bool },

    // ---- Game state broadcasts ----
    /// Another player's position/state update (broadcast to nearby clients).
    PlayerSync {
        player_id: u64,
        pos: [f32; 2],
        hp_frac: f32,
        mp_frac: f32,
        level: u32,
        facing_right: bool,
        running: bool,
        active_skill: u8,
    },
    /// A player joined the current zone.
    PlayerJoined { player_id: u64, name: String, level: u32 },
    /// A player left the current zone.
    PlayerLeft { player_id: u64 },
    /// Chat message from another player.
    ChatMessage { player_id: u64, name: String, message: String },
    /// An enemy took damage (broadcast for visual sync).
    EnemyDamaged { enemy_index: u32, new_hp: f32, attacker_id: u64 },
    /// An enemy was killed.
    EnemyKilled { enemy_index: u32, killer_id: u64, xp: f32 },
    /// A gather node was harvested by someone.
    NodeHarvested { node_index: u32, harvester_id: u64 },
    /// Full zone snapshot on zone enter.
    ZoneSnapshot { zone_id: u32, enemy_count: u32, node_count: u32 },
}

// ─── Snapshot envelope ────────────────────────────────────────────────────────

/// Envelope wrapping any snapshot payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SnapshotEnvelope {
    pub protocol_version: u32,
    pub tick: u64,
    pub scene_id: String,
}

impl SnapshotEnvelope {
    #[must_use]
    pub fn bootstrap(scene_id: impl Into<String>) -> Self {
        Self { protocol_version: PROTOCOL_VERSION, tick: 0, scene_id: scene_id.into() }
    }

    /// Returns `true` if this snapshot is compatible with `other_version`.
    #[must_use]
    pub fn is_compatible(&self, other_version: u32) -> bool {
        self.protocol_version == other_version
    }
}

// ─── Delta encoding ───────────────────────────────────────────────────────────

/// A field-level delta entry for incremental updates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeltaField {
    pub field_id: u16,
    pub value_bytes: Vec<u8>,
}

/// A delta snapshot: only changed fields since the last full snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSnapshot {
    pub envelope: SnapshotEnvelope,
    /// Tick of the baseline full snapshot this delta is relative to.
    pub baseline_tick: u64,
    pub fields: Vec<DeltaField>,
}

impl DeltaSnapshot {
    #[must_use]
    pub fn new(scene_id: impl Into<String>, tick: u64, baseline_tick: u64) -> Self {
        Self {
            envelope: SnapshotEnvelope {
                protocol_version: PROTOCOL_VERSION,
                tick,
                scene_id: scene_id.into(),
            },
            baseline_tick,
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, field_id: u16, value_bytes: Vec<u8>) {
        self.fields.push(DeltaField { field_id, value_bytes });
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bootstrap_snapshot_uses_current_protocol() {
        let snapshot = SnapshotEnvelope::bootstrap("act1_rogue_camp");
        assert_eq!(snapshot.protocol_version, PROTOCOL_VERSION);
        assert_eq!(snapshot.tick, 0);
    }

    #[test]
    fn snapshot_compatibility_check() {
        let s = SnapshotEnvelope::bootstrap("zone");
        assert!(s.is_compatible(PROTOCOL_VERSION));
        assert!(!s.is_compatible(1));
    }

    #[test]
    fn delta_snapshot_add_fields() {
        let mut delta = DeltaSnapshot::new("zone", 10, 5);
        assert!(delta.is_empty());
        delta.add_field(1, vec![0x01]);
        assert!(!delta.is_empty());
    }

    #[test]
    fn client_commands_are_distinct() {
        let a = ClientCommand::JoinScene { scene_id: "s1".into() };
        let b = ClientCommand::LeaveScene;
        assert_ne!(a, b);
    }

    #[test]
    fn server_event_transition_complete() {
        let e = ServerEvent::ZoneTransitionComplete { zone_id: 42 };
        assert_eq!(e, ServerEvent::ZoneTransitionComplete { zone_id: 42 });
    }
}
