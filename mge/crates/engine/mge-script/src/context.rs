// @id: MGE-Script-Context @do: script-context @role: back-end @layer: 2 @human: miyuk
//! Script execution context — state injected into every Rhai script.

use std::collections::HashMap;

/// Everything a script can read or modify about the current game state.
///
/// Passed by clone into the Rhai engine scope (all fields are simple,
/// `Clone + Send + Sync` types).
#[derive(Debug, Clone)]
pub struct ScriptContext {
    /// ID of the current character.
    pub character_id: String,
    /// Character level.
    pub level: i32,
    /// Core stat: strength.
    pub strength: i32,
    /// Core stat: dexterity.
    pub dexterity: i32,
    /// Simplified inventory: `item_id -> quantity`.
    pub inventory: HashMap<String, u32>,
    /// Active quest flags: `quest_id -> state`.
    pub quest_flags: HashMap<String, String>,
    /// Current zone id.
    pub zone_id: String,
    /// Current NPC (set when a dialogue is active).
    pub npc_id: Option<String>,
    /// Kill counts by monster type (for kill-count quests).
    pub kill_counts: HashMap<String, u32>,
    /// Unlocked waypoints: `act -> [waypoint_id, ...]`.
    pub waypoints: HashMap<i32, Vec<String>>,
    // --- pending actions (returned to the game loop) ---
    /// Messages to display to the player.
    pub pending_messages: Vec<String>,
    /// Zone to warp the player to.
    pub pending_warp: Option<String>,
    /// Items to grant the player: `(item_id, qty)`.
    pub pending_item_grants: Vec<(String, u32)>,
    /// XP to award.
    pub pending_xp: i64,
}

impl ScriptContext {
    /// Create a fresh context for a character in a zone.
    pub fn new(character_id: &str, zone_id: &str) -> Self {
        Self {
            character_id: character_id.to_string(),
            level: 1,
            strength: 10,
            dexterity: 10,
            inventory: HashMap::new(),
            quest_flags: HashMap::new(),
            zone_id: zone_id.to_string(),
            npc_id: None,
            kill_counts: HashMap::new(),
            waypoints: HashMap::new(),
            pending_messages: Vec::new(),
            pending_warp: None,
            pending_item_grants: Vec::new(),
            pending_xp: 0,
        }
    }
}
