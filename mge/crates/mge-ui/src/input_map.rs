use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Logical game action (not tied to a physical key).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameAction {
    MoveClick,
    SkillPrimary,
    SkillSecondary,
    Skill3,
    Skill4,
    Skill5,
    Skill6,
    ToggleInventory,
    ToggleCharSheet,
    ToggleQuestLog,
    ToggleSystemMenu,
    ShowMap,
    UsePotion1,
    UsePotion2,
    UsePotion3,
    UsePotion4,
    Interact,
    ShowLoot,
    Screenshot,
    ForceStand,
}

/// Physical input source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputKey {
    MouseLeft,
    MouseRight,
    MouseMiddle,
    Key(char),
    Escape,
    Tab,
    Space,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    Shift,
    Ctrl,
    Alt,
}

/// Context in which an action fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputContext {
    /// In the game world with no overlay open.
    World,
    /// A UI overlay is focused.
    Overlay,
    /// Always active regardless of context.
    Global,
}

/// A single binding: key -> action with context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBinding {
    pub key: InputKey,
    pub action: GameAction,
    pub context: InputContext,
}

/// Input map managing key bindings and conflict detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMap {
    bindings: Vec<KeyBinding>,
}

impl Default for InputMap {
    fn default() -> Self {
        Self {
            bindings: vec![
                KeyBinding {
                    key: InputKey::MouseLeft,
                    action: GameAction::MoveClick,
                    context: InputContext::World,
                },
                KeyBinding {
                    key: InputKey::MouseRight,
                    action: GameAction::SkillSecondary,
                    context: InputContext::World,
                },
                KeyBinding {
                    key: InputKey::Key('i'),
                    action: GameAction::ToggleInventory,
                    context: InputContext::Global,
                },
                KeyBinding {
                    key: InputKey::Key('c'),
                    action: GameAction::ToggleCharSheet,
                    context: InputContext::Global,
                },
                KeyBinding {
                    key: InputKey::Key('q'),
                    action: GameAction::ToggleQuestLog,
                    context: InputContext::Global,
                },
                KeyBinding {
                    key: InputKey::Escape,
                    action: GameAction::ToggleSystemMenu,
                    context: InputContext::Global,
                },
                KeyBinding {
                    key: InputKey::Tab,
                    action: GameAction::ShowMap,
                    context: InputContext::Global,
                },
                KeyBinding {
                    key: InputKey::Key('1'),
                    action: GameAction::UsePotion1,
                    context: InputContext::World,
                },
                KeyBinding {
                    key: InputKey::Key('2'),
                    action: GameAction::UsePotion2,
                    context: InputContext::World,
                },
                KeyBinding {
                    key: InputKey::Key('3'),
                    action: GameAction::UsePotion3,
                    context: InputContext::World,
                },
                KeyBinding {
                    key: InputKey::Key('4'),
                    action: GameAction::UsePotion4,
                    context: InputContext::World,
                },
                KeyBinding {
                    key: InputKey::Key('a'),
                    action: GameAction::ShowLoot,
                    context: InputContext::World,
                },
                KeyBinding {
                    key: InputKey::Shift,
                    action: GameAction::ForceStand,
                    context: InputContext::World,
                },
                KeyBinding {
                    key: InputKey::F5,
                    action: GameAction::Screenshot,
                    context: InputContext::Global,
                },
            ],
        }
    }
}

impl InputMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get all actions triggered by a key in a given context.
    pub fn actions_for(&self, key: InputKey, context: InputContext) -> Vec<GameAction> {
        self.bindings
            .iter()
            .filter(|b| b.key == key && (b.context == context || b.context == InputContext::Global))
            .map(|b| b.action)
            .collect()
    }

    /// Rebind a key to an action. Removes existing binding for that action first.
    pub fn rebind(&mut self, action: GameAction, key: InputKey, context: InputContext) {
        self.bindings.retain(|b| b.action != action);
        self.bindings.push(KeyBinding { key, action, context });
    }

    /// Detect key conflicts: multiple actions bound to the same key in overlapping contexts.
    pub fn conflicts(&self) -> Vec<(InputKey, Vec<GameAction>)> {
        let mut map: HashMap<(InputKey, InputContext), Vec<GameAction>> = HashMap::new();
        for b in &self.bindings {
            map.entry((b.key, b.context)).or_default().push(b.action);
        }
        map.into_iter()
            .filter(|(_, actions)| actions.len() > 1)
            .map(|((key, _), actions)| (key, actions))
            .collect()
    }

    /// Get the key bound to a specific action.
    pub fn key_for(&self, action: GameAction) -> Option<InputKey> {
        self.bindings.iter().find(|b| b.action == action).map(|b| b.key)
    }

    pub fn bindings(&self) -> &[KeyBinding] {
        &self.bindings
    }
}

/// Mouse click context for the game world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClickTarget {
    Ground { world_x: i32, world_y: i32 },
    Monster { entity_id: u64 },
    Npc { entity_id: u64 },
    Item { item_id: u64 },
    Portal,
    Waypoint,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bindings_exist() {
        let map = InputMap::new();
        assert!(!map.bindings().is_empty());
        let actions = map.actions_for(InputKey::Key('i'), InputContext::World);
        assert!(actions.contains(&GameAction::ToggleInventory));
    }

    #[test]
    fn rebind_replaces_action() {
        let mut map = InputMap::new();
        map.rebind(GameAction::ToggleInventory, InputKey::Key('b'), InputContext::Global);
        assert_eq!(map.key_for(GameAction::ToggleInventory), Some(InputKey::Key('b')));
        let old = map.actions_for(InputKey::Key('i'), InputContext::World);
        assert!(!old.contains(&GameAction::ToggleInventory));
    }

    #[test]
    fn no_default_conflicts() {
        let map = InputMap::new();
        assert!(map.conflicts().is_empty());
    }

    #[test]
    fn detects_conflicts() {
        let mut map = InputMap::new();
        map.bindings.push(KeyBinding {
            key: InputKey::Key('i'),
            action: GameAction::Interact,
            context: InputContext::Global,
        });
        let conflicts = map.conflicts();
        assert!(!conflicts.is_empty());
    }

    #[test]
    fn global_context_matches_world() {
        let map = InputMap::new();
        let actions = map.actions_for(InputKey::Escape, InputContext::World);
        assert!(actions.contains(&GameAction::ToggleSystemMenu));
    }
}
