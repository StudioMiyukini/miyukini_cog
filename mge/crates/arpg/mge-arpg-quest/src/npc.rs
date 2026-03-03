// @id: MGE-ARPG-NPC @do: npc-dialogue @role: back-end @layer: 3 @human: miyuk
//! NPC dialogue system for the ARPG quest layer.
//!
//! Provides static NPC definitions (`NpcDef`) with tree-structured dialogue
//! (`DialogueNode`, `DialogueResponse`, `DialogueAction`) and a runtime
//! conversation tracker (`DialogueState`).  Pre-built Act 1 NPCs (Akara,
//! Charsi, Deckard Cain) are exposed via factory functions.

// ── Types ─────────────────────────────────────────────────────────────────────

/// A serialisable action triggered when the player picks a dialogue response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DialogueAction {
    /// Advance to the `DialogueNode` with the given id.
    NextNode(String),
    /// Activate the quest identified by `quest_id`.
    GiveQuest(String),
    /// Mark the quest identified by `quest_id` as complete.
    CompleteQuest(String),
    /// Open the vendor / shop interface.
    OpenShop,
    /// Identify unidentified items in the player's inventory.
    Identify,
    /// Close the dialogue window.
    Exit,
}

/// A single player-visible option inside a `DialogueNode`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DialogueResponse {
    /// Short label shown to the player (e.g. "Tell me more").
    pub label: String,
    /// The action executed when the player picks this response.
    pub action: DialogueAction,
}

/// One node in a branching dialogue tree.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DialogueNode {
    /// Unique identifier within the owning `NpcDef`.
    pub id: String,
    /// The NPC speech shown to the player.
    pub text: String,
    /// Available player responses.
    pub responses: Vec<DialogueResponse>,
}

/// Static definition of a non-player character and their complete dialogue tree.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NpcDef {
    /// Unique NPC identifier (e.g. `"akara"`).
    pub id: String,
    /// Display name shown in the UI (e.g. `"Akara"`).
    pub name: String,
    /// Zone identifier where the NPC resides (e.g. `"act1_rogue_encampment"`).
    pub location: String,
    /// Ordered list of dialogue nodes.  The first node is the entry point.
    pub dialogue_tree: Vec<DialogueNode>,
}

// ── Runtime state ─────────────────────────────────────────────────────────────

/// Mutable runtime state of an active player–NPC conversation.
///
/// Not serialised — reconstructed from `NpcDef` on each session start.
#[derive(Debug, Clone)]
pub struct DialogueState {
    /// Id of the NPC this conversation belongs to.
    pub npc_id: String,
    /// Id of the `DialogueNode` currently displayed to the player.
    pub current_node_id: String,
    /// `false` once the conversation has been closed (via `Exit` or exhausted tree).
    pub is_active: bool,
}

impl DialogueState {
    /// Begin a new conversation with `npc`, starting at the first node.
    ///
    /// Returns `None` when `npc.dialogue_tree` is empty.
    #[must_use]
    pub fn start(npc: &NpcDef) -> Option<Self> {
        let first_node = npc.dialogue_tree.first()?;
        Some(Self {
            npc_id: npc.id.clone(),
            current_node_id: first_node.id.clone(),
            is_active: true,
        })
    }

    /// Apply the player's choice at `response_index` and return the resulting action.
    ///
    /// - If `response_index` is out of range → `None`.
    /// - If the current node id is not found in `npc` → `None`.
    /// - On `DialogueAction::Exit` the state is marked `is_active = false`.
    /// - On `DialogueAction::NextNode(id)` `current_node_id` is updated.
    pub fn advance(
        &mut self,
        response_index: usize,
        npc: &NpcDef,
    ) -> Option<DialogueAction> {
        let node = self.current_node(npc)?;
        let response = node.responses.get(response_index)?;
        let action = response.action.clone();
        match &action {
            DialogueAction::Exit => {
                self.is_active = false;
            }
            DialogueAction::NextNode(next_id) => {
                self.current_node_id.clone_from(next_id);
            }
            // All other actions do not alter navigation state.
            DialogueAction::GiveQuest(_)
            | DialogueAction::CompleteQuest(_)
            | DialogueAction::OpenShop
            | DialogueAction::Identify => {}
        }
        Some(action)
    }

    /// Return a reference to the current `DialogueNode` inside `npc`.
    ///
    /// Returns `None` when `current_node_id` does not match any node.
    #[must_use]
    pub fn current_node<'a>(&self, npc: &'a NpcDef) -> Option<&'a DialogueNode> {
        npc.dialogue_tree
            .iter()
            .find(|n| n.id == self.current_node_id)
    }
}

// ── Act 1 NPC factories ───────────────────────────────────────────────────────

/// Build the `NpcDef` for Akara, the High Priestess of the Sisterhood.
#[must_use]
pub fn akara() -> NpcDef {
    NpcDef {
        id: "akara".to_string(),
        name: "Akara".to_string(),
        location: "act1_rogue_encampment".to_string(),
        dialogue_tree: vec![
            DialogueNode {
                id: "greeting".to_string(),
                text: "Greetings, wanderer. The Den of Evil threatens us all.".to_string(),
                responses: vec![
                    DialogueResponse {
                        label: "Tell me about the Den".to_string(),
                        action: DialogueAction::NextNode("den_info".to_string()),
                    },
                    DialogueResponse {
                        label: "I need potions".to_string(),
                        action: DialogueAction::OpenShop,
                    },
                    DialogueResponse {
                        label: "Goodbye".to_string(),
                        action: DialogueAction::Exit,
                    },
                ],
            },
            DialogueNode {
                id: "den_info".to_string(),
                text: "A great evil lurks in a cave near the Blood Moor. Clear it, and I shall reward you.".to_string(),
                responses: vec![
                    DialogueResponse {
                        label: "I accept this quest".to_string(),
                        action: DialogueAction::GiveQuest("den_of_evil".to_string()),
                    },
                    DialogueResponse {
                        label: "Not now".to_string(),
                        action: DialogueAction::Exit,
                    },
                ],
            },
        ],
    }
}

/// Build the `NpcDef` for Charsi, the blacksmith of the Rogue Encampment.
#[must_use]
pub fn charsi() -> NpcDef {
    NpcDef {
        id: "charsi".to_string(),
        name: "Charsi".to_string(),
        location: "act1_rogue_encampment".to_string(),
        dialogue_tree: vec![
            DialogueNode {
                id: "greeting".to_string(),
                text: "Hello! Need something repaired or forged?".to_string(),
                responses: vec![
                    DialogueResponse {
                        label: "About the Malus".to_string(),
                        action: DialogueAction::NextNode("malus_info".to_string()),
                    },
                    DialogueResponse {
                        label: "Repair my items".to_string(),
                        action: DialogueAction::OpenShop,
                    },
                    DialogueResponse {
                        label: "Goodbye".to_string(),
                        action: DialogueAction::Exit,
                    },
                ],
            },
            DialogueNode {
                id: "malus_info".to_string(),
                text: "My enchanted hammer, the Horadric Malus, was lost when the Monastery fell. Bring it back to me!".to_string(),
                responses: vec![
                    DialogueResponse {
                        label: "I'll find it".to_string(),
                        action: DialogueAction::GiveQuest("tools_of_the_trade".to_string()),
                    },
                    DialogueResponse {
                        label: "Not yet".to_string(),
                        action: DialogueAction::Exit,
                    },
                ],
            },
        ],
    }
}

/// Build the `NpcDef` for Deckard Cain, the last of the Horadrim.
#[must_use]
pub fn deckard_cain() -> NpcDef {
    NpcDef {
        id: "deckard_cain".to_string(),
        name: "Deckard Cain".to_string(),
        location: "act1_rogue_encampment".to_string(),
        dialogue_tree: vec![
            DialogueNode {
                id: "greeting".to_string(),
                text: "Stay a while and listen! I can identify your items.".to_string(),
                responses: vec![
                    DialogueResponse {
                        label: "Identify my items".to_string(),
                        action: DialogueAction::Identify,
                    },
                    DialogueResponse {
                        label: "Tell me about Andariel".to_string(),
                        action: DialogueAction::NextNode("andariel_info".to_string()),
                    },
                    DialogueResponse {
                        label: "Goodbye".to_string(),
                        action: DialogueAction::Exit,
                    },
                ],
            },
            DialogueNode {
                id: "andariel_info".to_string(),
                text: "Andariel, the Maiden of Anguish, has corrupted the Monastery. She must be stopped.".to_string(),
                responses: vec![
                    DialogueResponse {
                        label: "I will destroy her".to_string(),
                        action: DialogueAction::GiveQuest("sisters_to_the_slaughter".to_string()),
                    },
                    DialogueResponse {
                        label: "I need more time".to_string(),
                        action: DialogueAction::Exit,
                    },
                ],
            },
        ],
    }
}

/// Return all Act 1 NPCs as a ready-to-use collection.
#[must_use]
pub fn act1_npcs() -> Vec<NpcDef> {
    vec![akara(), charsi(), deckard_cain()]
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── 1. Akara: follow the quest branch ────────────────────────────────────

    #[test]
    fn npc_akara_give_quest() {
        let npc = akara();
        let mut state = DialogueState::start(&npc).unwrap();

        // Greeting node — pick "Tell me about the Den" (index 0 → NextNode)
        let action = state.advance(0, &npc).unwrap();
        assert!(
            matches!(action, DialogueAction::NextNode(ref id) if id == "den_info"),
            "expected NextNode(den_info), got {action:?}"
        );
        assert_eq!(state.current_node_id, "den_info");

        // den_info node — pick "I accept this quest" (index 0 → GiveQuest)
        let action = state.advance(0, &npc).unwrap();
        assert!(
            matches!(action, DialogueAction::GiveQuest(ref id) if id == "den_of_evil"),
            "expected GiveQuest(den_of_evil), got {action:?}"
        );
    }

    // ── 2. Charsi: open shop ──────────────────────────────────────────────────

    #[test]
    fn npc_charsi_shop() {
        let npc = charsi();
        let mut state = DialogueState::start(&npc).unwrap();

        // Greeting — pick "Repair my items" (index 1 → OpenShop)
        let action = state.advance(1, &npc).unwrap();
        assert!(
            matches!(action, DialogueAction::OpenShop),
            "expected OpenShop, got {action:?}"
        );
    }

    // ── 3. Deckard Cain: identify ─────────────────────────────────────────────

    #[test]
    fn npc_cain_identify() {
        let npc = deckard_cain();
        let mut state = DialogueState::start(&npc).unwrap();

        // Greeting — pick "Identify my items" (index 0 → Identify)
        let action = state.advance(0, &npc).unwrap();
        assert!(
            matches!(action, DialogueAction::Identify),
            "expected Identify, got {action:?}"
        );
    }

    // ── 4. Exit closes dialogue ───────────────────────────────────────────────

    #[test]
    fn npc_dialogue_exit() {
        let npc = akara();
        let mut state = DialogueState::start(&npc).unwrap();

        // Greeting — pick "Goodbye" (index 2 → Exit)
        let action = state.advance(2, &npc).unwrap();
        assert!(
            matches!(action, DialogueAction::Exit),
            "expected Exit, got {action:?}"
        );
        assert!(!state.is_active, "is_active should be false after Exit");
    }

    // ── 5. act1_npcs returns exactly 3 entries ────────────────────────────────

    #[test]
    fn act1_npcs_count() {
        assert_eq!(act1_npcs().len(), 3);
    }

    // ── 6. Out-of-bounds response index returns None ──────────────────────────

    #[test]
    fn dialogue_advance_out_of_bounds() {
        let npc = akara();
        let mut state = DialogueState::start(&npc).unwrap();

        // Greeting has 3 responses (indices 0-2); index 99 is out of range.
        let result = state.advance(99, &npc);
        assert!(result.is_none(), "expected None for out-of-bounds index");
    }

    // ── 7. current_node with an unknown id returns None ───────────────────────

    #[test]
    fn dialogue_current_node_invalid() {
        let npc = akara();
        let mut state = DialogueState::start(&npc).unwrap();

        state.current_node_id = "nonexistent_node".to_string();
        let result = state.current_node(&npc);
        assert!(result.is_none(), "expected None for unknown node id");
    }
}
