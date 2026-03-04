// @id: MUIE-SkillTree @do: skill-tree-organism @role: organism @layer: 6 @human: miyuk

//! Skill tree panel organism -- 3 tabs, grid of skill nodes with prerequisite lines.

use egui::{Context, Pos2, Stroke};
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::molecules::skill_node::{SkillNode, SkillNodeData};
use crate::convert::rgba_to_color32;

/// Data for one skill tree tab.
#[derive(Debug, Clone)]
pub struct SkillTreeTabData {
    /// Tab name (e.g., "Fire", "Lightning", "Cold").
    pub name: String,
    /// Skills in this tree.
    pub skills: Vec<SkillNodeData>,
}

/// Complete skill tree data.
pub struct SkillTreeData {
    /// The 3 skill tree tabs.
    pub tabs: [SkillTreeTabData; 3],
    /// Currently active tab index (0-2).
    pub active_tab: usize,
    /// Remaining skill points.
    pub remaining_points: u32,
}

/// The skill tree panel organism.
pub struct SkillTreePanel;

impl SkillTreePanel {
    /// Draw the skill tree panel as a window.
    pub fn show(ctx: &Context, is_open: &mut bool, data: &SkillTreeData) {
        if !*is_open {
            return;
        }

        egui::Window::new("Arbre de competences")
            .resizable(false)
            .collapsible(false)
            .open(is_open)
            .default_pos([100.0, 50.0])
            .min_width(320.0)
            .show(ctx, |ui| {
                // Tab buttons
                ui.horizontal(|ui| {
                    for (i, tab) in data.tabs.iter().enumerate() {
                        let is_active = i == data.active_tab;
                        let text_color = if is_active {
                            rgba_to_color32(&D2_PALETTE.text_high)
                        } else {
                            rgba_to_color32(&D2_PALETTE.text_secondary)
                        };
                        let label = egui::RichText::new(&tab.name)
                            .color(text_color)
                            .size(12.0);
                        if ui.add(egui::Label::new(label).sense(egui::Sense::click())).clicked() {
                            // Tab switching would be handled by the caller
                        }
                        if i < 2 {
                            ui.label(" | ");
                        }
                    }
                });

                ui.separator();

                // Active tab skills
                let active_tab = data.active_tab.min(2);
                let tab = &data.tabs[active_tab];
                let has_points = data.remaining_points > 0;

                // Skill grid (3 columns x 6 rows)
                let node_size = 44.0;
                let gap = 8.0;
                ui.vertical(|ui| {
                    // Collect node positions for drawing lines
                    let mut node_centers: Vec<Pos2> = Vec::new();

                    // Draw skills row by row
                    let max_rows = 6;
                    for row in 0..max_rows {
                        let row_u32 = row as u32;
                        let row_skills: Vec<&SkillNodeData> = tab.skills.iter()
                            .filter(|s| s.grid_row == row_u32)
                            .collect();

                        ui.horizontal(|ui| {
                            // Pad to align in 3-column grid
                            for col in 0..3_u32 {
                                let skill_opt = row_skills.iter()
                                    .find(|s| s.grid_col == col);

                                if let Some(skill) = skill_opt {
                                    let resp = SkillNode::new(skill)
                                        .points_available(has_points)
                                        .show(ui);
                                    node_centers.push(resp.rect.center());
                                } else {
                                    ui.add_space(node_size);
                                    let rect = ui.min_rect();
                                    node_centers.push(rect.center());
                                }

                                if col < 2 {
                                    ui.add_space(gap);
                                }
                            }
                        });

                        ui.add_space(gap / 2.0);
                    }

                    // Draw prerequisite lines
                    // (simplified: connect skills to their prerequisites via straight lines)
                    let gold = rgba_to_color32(&D2_PALETTE.accent_primary);
                    let gray = rgba_to_color32(&D2_PALETTE.border_subtle);
                    let painter = ui.painter();

                    for skill in &tab.skills {
                        let skill_idx = skill.grid_row as usize * 3 + skill.grid_col as usize;
                        if skill_idx >= node_centers.len() {
                            continue;
                        }
                        let end = node_centers[skill_idx];

                        for &prereq_idx in &skill.prereq_indices {
                            if prereq_idx >= node_centers.len() {
                                continue;
                            }
                            let start = node_centers[prereq_idx];
                            let line_color = if skill.prereqs_met { gold } else { gray };
                            painter.line_segment(
                                [start, end],
                                Stroke::new(1.5, line_color),
                            );
                        }
                    }
                });

                ui.separator();

                // Remaining points
                let pts_text = format!("Points de competence : {}", data.remaining_points);
                let pts_color = if data.remaining_points > 0 {
                    rgba_to_color32(&D2_PALETTE.text_high)
                } else {
                    rgba_to_color32(&D2_PALETTE.text_secondary)
                };
                ui.label(egui::RichText::new(&pts_text).color(pts_color).size(11.0));
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_tree_tab_data() {
        let tab = SkillTreeTabData {
            name: "Fire".to_string(),
            skills: vec![],
        };
        assert_eq!(tab.name, "Fire");
    }
}
