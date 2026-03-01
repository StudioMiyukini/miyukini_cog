// @id: MGE-ARPG-Quest-Objective @do: quest-objectives @role: back-end @layer: 3 @human: miyuk
//! Quest objective types and tracking.

/// The kind of objective a quest can require.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ObjectiveKind {
    /// Kill a specific monster type a number of times.
    KillMonster {
        /// Identifier of the monster type to kill.
        monster_id: String,
        /// How many kills are required.
        required: u32,
        /// How many have been killed so far.
        killed: u32,
    },
    /// Reach a specific map zone.
    ReachLocation {
        /// Identifier of the target zone.
        zone_id: String,
        /// Whether the location has been reached.
        reached: bool,
    },
    /// Collect a specific item a number of times.
    CollectItem {
        /// Identifier of the item to collect.
        item_id: String,
        /// How many items are required.
        required: u32,
        /// How many have been collected so far.
        collected: u32,
    },
    /// Talk to a specific NPC.
    TalkToNpc {
        /// Identifier of the NPC.
        npc_id: String,
        /// Whether the NPC has been talked to.
        talked: bool,
    },
    /// Activate a world object (shrine, waypoint, etc.).
    ActivateObject {
        /// Identifier of the object.
        object_id: String,
        /// Whether the object has been activated.
        activated: bool,
    },
}

impl ObjectiveKind {
    /// Returns `true` when the objective has been fulfilled.
    pub fn is_complete(&self) -> bool {
        match self {
            Self::KillMonster {
                required, killed, ..
            } => *killed >= *required,
            Self::ReachLocation { reached, .. } => *reached,
            Self::CollectItem {
                required,
                collected,
                ..
            } => *collected >= *required,
            Self::TalkToNpc { talked, .. } => *talked,
            Self::ActivateObject { activated, .. } => *activated,
        }
    }

    /// Returns a human-readable progress string.
    pub fn progress_text(&self) -> String {
        match self {
            Self::KillMonster {
                monster_id,
                required,
                killed,
            } => format!("{killed}/{required} {monster_id} killed"),
            Self::ReachLocation { zone_id, reached } => {
                if *reached {
                    format!("{zone_id} reached")
                } else {
                    format!("{zone_id} not yet reached")
                }
            }
            Self::CollectItem {
                item_id,
                required,
                collected,
            } => format!("{collected}/{required} {item_id} collected"),
            Self::TalkToNpc { npc_id, talked } => {
                if *talked {
                    format!("Talked to {npc_id}")
                } else {
                    format!("Talk to {npc_id}")
                }
            }
            Self::ActivateObject {
                object_id,
                activated,
            } => {
                if *activated {
                    format!("{object_id} activated")
                } else {
                    format!("Activate {object_id}")
                }
            }
        }
    }
}

/// A single quest objective with metadata.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Objective {
    /// Unique identifier for this objective within the quest.
    pub id: String,
    /// Human-readable description.
    pub description: String,
    /// The specific kind and tracking data.
    pub kind: ObjectiveKind,
    /// If `true`, this objective is not required for quest completion.
    pub optional: bool,
}

impl Objective {
    /// Returns `true` when the underlying objective kind is complete.
    pub fn is_complete(&self) -> bool {
        self.kind.is_complete()
    }
}
