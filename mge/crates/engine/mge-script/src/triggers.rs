// @id: MGE-Script-Triggers @do: trigger-system @role: back-end @layer: 2 @human: miyuk
//! Event trigger system — maps game events to script executions.

use crate::{ScriptContext, ScriptEngine, ScriptResult};

/// Types of events that can trigger a script.
#[derive(Debug, Clone)]
pub enum TriggerType {
    /// Player enters a zone.
    ZoneEnter {
        /// The zone that triggers the script.
        zone_id: String,
    },
    /// Player talks to an NPC.
    NpcTalk {
        /// The NPC that triggers the script.
        npc_id: String,
    },
    /// A monster is killed (by type). Use `"*"` for all monsters.
    MonsterKilled {
        /// Monster type filter.
        monster_type: String,
    },
    /// An item is picked up.
    ItemPickedUp {
        /// The item that triggers the script.
        item_id: String,
    },
    /// A quest is completed.
    QuestComplete {
        /// The quest that triggers the script.
        quest_id: String,
    },
    /// The player levels up.
    LevelUp {
        /// The exact level that triggers the script.
        level: i32,
    },
}

/// A single trigger mapping: event type to script id.
pub struct Trigger {
    /// The event type that activates this trigger.
    pub trigger_type: TriggerType,
    /// Id of the compiled script to execute.
    pub script_id: String,
}

/// Manages a list of triggers and fires matching scripts when events occur.
pub struct TriggerSystem {
    triggers: Vec<Trigger>,
}

impl TriggerSystem {
    /// Create an empty trigger system.
    pub fn new() -> Self {
        Self {
            triggers: Vec::new(),
        }
    }

    /// Register a new trigger.
    pub fn register(&mut self, trigger_type: TriggerType, script_id: &str) {
        self.triggers.push(Trigger {
            trigger_type,
            script_id: script_id.to_string(),
        });
    }

    /// Fire all scripts registered for a `ZoneEnter` event matching `zone_id`.
    pub fn fire_zone_enter(
        &self,
        zone_id: &str,
        engine: &ScriptEngine,
        ctx: &mut ScriptContext,
    ) -> ScriptResult<()> {
        for t in &self.triggers {
            if let TriggerType::ZoneEnter { zone_id: z } = &t.trigger_type {
                if z == zone_id {
                    engine.run(&t.script_id, ctx)?;
                }
            }
        }
        Ok(())
    }

    /// Fire all scripts registered for an `NpcTalk` event matching `npc_id`.
    pub fn fire_npc_talk(
        &self,
        npc_id: &str,
        engine: &ScriptEngine,
        ctx: &mut ScriptContext,
    ) -> ScriptResult<()> {
        ctx.npc_id = Some(npc_id.to_string());
        for t in &self.triggers {
            if let TriggerType::NpcTalk { npc_id: n } = &t.trigger_type {
                if n == npc_id {
                    engine.run(&t.script_id, ctx)?;
                }
            }
        }
        Ok(())
    }

    /// Fire all scripts registered for a `MonsterKilled` event.
    ///
    /// Automatically increments the kill count for `monster_type` in the context
    /// before running any scripts. A trigger with `monster_type = "*"` matches
    /// all monster types.
    pub fn fire_monster_killed(
        &self,
        monster_type: &str,
        engine: &ScriptEngine,
        ctx: &mut ScriptContext,
    ) -> ScriptResult<()> {
        // Increment the kill count in the context.
        *ctx.kill_counts
            .entry(monster_type.to_string())
            .or_insert(0) += 1;

        for t in &self.triggers {
            if let TriggerType::MonsterKilled { monster_type: m } =
                &t.trigger_type
            {
                if m == monster_type || m == "*" {
                    engine.run(&t.script_id, ctx)?;
                }
            }
        }
        Ok(())
    }
}

impl Default for TriggerSystem {
    fn default() -> Self {
        Self::new()
    }
}
