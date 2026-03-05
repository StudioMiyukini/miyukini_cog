// @id: MUIE-SkillNode @do: skill-node-molecule @role: molecule @layer: 6 @human: miyuk

//! Skill tree node molecule -- icon + level + prerequisite connection point.
//!
//! Each node represents one skill in the skill tree. It shows the skill icon
//! with its current invested level and can be connected to prerequisite nodes
//! via lines drawn by the parent organism.

use crate::atoms::skill_icon::{SkillIcon, SkillState};
use egui::{Pos2, Response, Ui};

/// Data for a skill tree node.
#[derive(Debug, Clone)]
pub struct SkillNodeData {
    /// Skill name.
    pub name: String,
    /// Invested skill points.
    pub level: u32,
    /// Maximum skill level.
    pub max_level: u32,
    /// Whether prerequisites are met.
    pub prereqs_met: bool,
    /// Required character level to unlock.
    pub required_level: u32,
    /// Grid position (col, row) in the 3x6 skill tree grid.
    pub grid_col: u32,
    pub grid_row: u32,
    /// IDs of prerequisite skills (for drawing connection lines).
    pub prereq_indices: Vec<usize>,
}

/// A skill tree node molecule.
pub struct SkillNode<'a> {
    /// Skill data.
    pub data: &'a SkillNodeData,
    /// Whether the player has available skill points.
    pub points_available: bool,
    /// Whether this skill is currently selected/active.
    pub selected: bool,
}

impl<'a> SkillNode<'a> {
    /// Create a new skill node.
    pub fn new(data: &'a SkillNodeData) -> Self {
        Self {
            data,
            points_available: false,
            selected: false,
        }
    }

    /// Set whether skill points are available.
    pub fn points_available(mut self, available: bool) -> Self {
        self.points_available = available;
        self
    }

    /// Set whether this node is selected.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Determine the visual state of this node.
    fn skill_state(&self) -> SkillState {
        if self.selected {
            SkillState::Active
        } else if !self.data.prereqs_met {
            SkillState::Locked
        } else if self.data.level > 0 || self.points_available {
            SkillState::Inactive
        } else {
            SkillState::Locked
        }
    }

    /// Draw the skill node and return the egui [`Response`].
    ///
    /// The returned response indicates whether the node was clicked (to invest a point)
    /// or right-clicked (to assign the skill).
    pub fn show(self, ui: &mut Ui) -> Response {
        let state = self.skill_state();

        let response = SkillIcon::new(&self.data.name, state)
            .level(self.data.level)
            .max_level(self.data.max_level)
            .size(40.0)
            .show(ui);

        response
    }

    /// Get the center position for drawing prerequisite lines.
    ///
    /// This should be called after `show()` using the response rect.
    pub fn center_from_response(response: &Response) -> Pos2 {
        response.rect.center()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node() -> SkillNodeData {
        SkillNodeData {
            name: "Fireball".to_string(),
            level: 5,
            max_level: 20,
            prereqs_met: true,
            required_level: 12,
            grid_col: 1,
            grid_row: 2,
            prereq_indices: vec![0],
        }
    }

    #[test]
    fn test_skill_node_state_active() {
        let data = make_node();
        let node = SkillNode::new(&data).selected(true);
        assert_eq!(node.skill_state(), SkillState::Active);
    }

    #[test]
    fn test_skill_node_state_locked() {
        let mut data = make_node();
        data.prereqs_met = false;
        let node = SkillNode::new(&data);
        assert_eq!(node.skill_state(), SkillState::Locked);
    }

    #[test]
    fn test_skill_node_state_inactive_with_points() {
        let data = make_node();
        let node = SkillNode::new(&data).points_available(true);
        assert_eq!(node.skill_state(), SkillState::Inactive);
    }
}
