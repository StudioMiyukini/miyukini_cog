use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InputAction {
    MovePrimary,
    SkillPrimary,
    InventoryToggle,
    QuestLogToggle,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputSnapshot {
    pub triggered: Vec<InputAction>,
}
