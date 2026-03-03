// @id: MGE-UI-SkillTree @do: skill-tree-ui @role: front-end @layer: 2 @human: miyuk
//! Skill tree panel — data model (tabs, prerequisites, investment) and egui renderer stub.
//!
//! Provides `SkillTreeTab`, `SkillNodeState`, and `SkillTreeState` for the
//! Diablo II-style three-tab class skill tree (Summoning / PoisonBone / Curses).
//!
//! The `draw_skill_tree` function remains available for the egui render layer.

use egui::Context;
use serde::{Deserialize, Serialize};

use crate::theme::D2Colors;

// ---------------------------------------------------------------------------
// SkillTreeTab
// ---------------------------------------------------------------------------

/// Which of the three class skill columns is currently visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillTreeTab {
    /// Summoning skills (raise skeleton, golem, revive, …).
    Summoning,
    /// Poison and Bone skills (teeth, corpse explosion, bone spear, …).
    PoisonBone,
    /// Curse skills (amplify damage, decrepify, lower resist, …).
    Curses,
}

// ---------------------------------------------------------------------------
// SkillNodeState
// ---------------------------------------------------------------------------

/// Display state for a single node in the skill tree grid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillNodeState {
    /// Unique skill identifier (matches game-data key).
    pub skill_id: String,
    /// Human-readable skill name.
    pub name: String,
    /// Points already invested in this skill (0 = not learned).
    pub level: u32,
    /// Maximum investable level (typically 20).
    pub max_level: u32,
    /// Character level required before this skill can be learned.
    pub required_level: u32,
    /// All prerequisite skills currently have level >= 1.
    pub prereqs_met: bool,
    /// `true` when the skill can receive a point right now:
    /// `prereqs_met && char_level >= required_level && available_points > 0`.
    pub investable: bool,
    /// Base mana cost at level 1.
    pub mana_cost: f32,
    /// Which tab column this skill belongs to.
    pub tab: SkillTreeTab,
    /// Column position in the tree grid (0-3).
    pub grid_x: u8,
    /// Row position in the tree grid (0-5).
    pub grid_y: u8,
    /// Skill IDs that must be at level >= 1 before this skill unlocks.
    pub prereq_ids: Vec<String>,
}

// ---------------------------------------------------------------------------
// SkillTreeState
// ---------------------------------------------------------------------------

/// Complete state of the skill tree panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTreeState {
    /// Whether the panel is currently displayed.
    pub visible: bool,
    /// Which tab is active.
    pub active_tab: SkillTreeTab,
    /// All skill nodes across all three tabs.
    pub nodes: Vec<SkillNodeState>,
    /// Unspent skill points available for investment.
    pub available_points: u32,
}

impl SkillTreeState {
    /// Create a new, hidden skill tree with no nodes and zero available points.
    pub fn new() -> Self {
        Self {
            visible: false,
            active_tab: SkillTreeTab::Summoning,
            nodes: Vec::new(),
            available_points: 0,
        }
    }

    /// Toggle panel visibility.
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Switch the active tab.
    pub fn set_tab(&mut self, tab: SkillTreeTab) {
        self.active_tab = tab;
    }

    /// Return references to all nodes belonging to `tab`.
    pub fn nodes_for_tab(&self, tab: SkillTreeTab) -> Vec<&SkillNodeState> {
        self.nodes.iter().filter(|n| n.tab == tab).collect()
    }

    /// Attempt to invest one point in the skill identified by `skill_id`.
    ///
    /// Returns `true` if the investment succeeded:
    /// - the node exists,
    /// - `investable` is `true`,
    /// - `level < max_level`,
    /// - `available_points > 0`.
    ///
    /// On success, `level` is incremented and `available_points` is decremented.
    pub fn invest(&mut self, skill_id: &str) -> bool {
        if self.available_points == 0 {
            return false;
        }
        let node = self.nodes.iter_mut().find(|n| n.skill_id == skill_id);
        match node {
            Some(n) if n.investable && n.level < n.max_level => {
                n.level += 1;
                self.available_points -= 1;
                true
            }
            _ => false,
        }
    }

    /// Recalculate `prereqs_met` and `investable` for every node.
    ///
    /// Must be called after any investment or when `char_level` changes.
    ///
    /// # Algorithm
    /// For each node, `prereqs_met` is `true` when every `prereq_id` maps to a
    /// node whose `level >= 1`. `investable` additionally requires
    /// `char_level >= required_level` and `available_points > 0`.
    pub fn update_investable(&mut self, char_level: u32) {
        // Snapshot current levels to avoid borrow conflicts while mutating.
        let level_map: std::collections::HashMap<String, u32> = self
            .nodes
            .iter()
            .map(|n| (n.skill_id.clone(), n.level))
            .collect();

        let points = self.available_points;

        for node in &mut self.nodes {
            let prereqs_met = node
                .prereq_ids
                .iter()
                .all(|pid| level_map.get(pid).copied().unwrap_or(0) >= 1);

            node.prereqs_met = prereqs_met;
            node.investable = prereqs_met && char_level >= node.required_level && points > 0;
        }
    }
}

impl Default for SkillTreeState {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// egui renderer (stub — full render to be implemented in a later task)
// ---------------------------------------------------------------------------

/// Draw the skill tree panel (stub renderer).
///
/// Opens as an egui window when `is_open` is `true`. For the full data model,
/// use [`SkillTreeState`] which is maintained independently of the render call.
pub fn draw_skill_tree(ctx: &Context, is_open: &mut bool) {
    if !*is_open {
        return;
    }

    egui::Window::new("Arbre de competences")
        .resizable(false)
        .collapsible(false)
        .open(is_open)
        .default_pos([100.0, 50.0])
        .min_width(300.0)
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new("Arbre de competences")
                    .color(D2Colors::GOLD_BRIGHT)
                    .size(14.0),
            );
            ui.separator();
            ui.label(
                egui::RichText::new("(A implementer -- arbre par classe avec 3 colonnes)")
                    .color(D2Colors::TEXT_NORMAL)
                    .size(11.0),
            );
        });
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: build a `SkillNodeState` with sensible defaults.
    fn make_node(
        skill_id: &str,
        tab: SkillTreeTab,
        prereq_ids: Vec<String>,
        level: u32,
    ) -> SkillNodeState {
        SkillNodeState {
            skill_id: skill_id.to_string(),
            name: skill_id.to_string(),
            level,
            max_level: 20,
            required_level: 1,
            prereqs_met: false,
            investable: false,
            mana_cost: 4.0,
            tab,
            grid_x: 0,
            grid_y: 0,
            prereq_ids,
        }
    }

    /// A skill whose prerequisite has level 0 must NOT be investable.
    #[test]
    fn skill_tree_prereq_lock() {
        let mut state = SkillTreeState::new();
        state.available_points = 3;

        // "bone_spear" requires "teeth" which has level 0.
        state.nodes.push(make_node("teeth", SkillTreeTab::PoisonBone, vec![], 0));
        state
            .nodes
            .push(make_node("bone_spear", SkillTreeTab::PoisonBone, vec!["teeth".to_string()], 0));

        state.update_investable(10);

        let bone_spear = state.nodes.iter().find(|n| n.skill_id == "bone_spear").unwrap();
        assert!(!bone_spear.prereqs_met, "prereqs_met must be false when prereq level is 0");
        assert!(!bone_spear.investable, "investable must be false when prereqs not met");
    }

    /// Investing in a ready node decrements points and increments level.
    #[test]
    fn skill_tree_invest() {
        let mut state = SkillTreeState::new();
        state.available_points = 5;

        let mut node = make_node("raise_skeleton", SkillTreeTab::Summoning, vec![], 0);
        node.prereqs_met = true;
        node.investable = true;
        state.nodes.push(node);

        let invested = state.invest("raise_skeleton");

        assert!(invested, "invest must return true when conditions are met");

        let after = state.nodes.iter().find(|n| n.skill_id == "raise_skeleton").unwrap();
        assert_eq!(after.level, 1, "level must be 1 after first investment");
        assert_eq!(state.available_points, 4, "available_points must decrease by 1");
    }

    /// Investing in a maxed skill (level == max_level) must return false.
    #[test]
    fn skill_tree_max_20() {
        let mut state = SkillTreeState::new();
        state.available_points = 5;

        let mut node = make_node("amplify_damage", SkillTreeTab::Curses, vec![], 20);
        // Even though investable is true, max is already reached.
        node.prereqs_met = true;
        node.investable = true;
        state.nodes.push(node);

        let invested = state.invest("amplify_damage");

        assert!(!invested, "invest must return false when level == max_level");
        assert_eq!(state.available_points, 5, "points must not change on failed invest");
    }

    /// `nodes_for_tab` returns only nodes belonging to the requested tab.
    #[test]
    fn skill_tree_tab_filter() {
        let mut state = SkillTreeState::new();

        state
            .nodes
            .push(make_node("raise_skeleton", SkillTreeTab::Summoning, vec![], 1));
        state
            .nodes
            .push(make_node("clay_golem", SkillTreeTab::Summoning, vec![], 0));
        state
            .nodes
            .push(make_node("teeth", SkillTreeTab::PoisonBone, vec![], 2));
        state.nodes.push(make_node(
            "amplify_damage",
            SkillTreeTab::Curses,
            vec![],
            1,
        ));

        let summoning = state.nodes_for_tab(SkillTreeTab::Summoning);
        assert_eq!(summoning.len(), 2, "Summoning tab must have exactly 2 nodes");
        assert!(summoning.iter().all(|n| n.tab == SkillTreeTab::Summoning));

        let poison = state.nodes_for_tab(SkillTreeTab::PoisonBone);
        assert_eq!(poison.len(), 1, "PoisonBone tab must have exactly 1 node");

        let curses = state.nodes_for_tab(SkillTreeTab::Curses);
        assert_eq!(curses.len(), 1, "Curses tab must have exactly 1 node");
    }
}
