//! @id     crates_mge_input_lib
//! @do     definir_input_action
//! @role   orchestration
//! @layer  core
//! @human  crates mge-input — module « lib ».

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
