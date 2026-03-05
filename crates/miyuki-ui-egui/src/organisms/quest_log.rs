// @id: MUIE-QuestLog @do: quest-log-organism @role: organism @layer: 6 @human: miyuk

//! Quest log organism -- 5 act tabs with quest entries.

use crate::convert::rgba_to_color32;
use crate::molecules::quest_entry::{QuestEntry, QuestState};
use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;

/// Quest data for one act.
#[derive(Debug, Clone)]
pub struct ActQuestData {
    /// Act name (e.g., "Acte I").
    pub name: String,
    /// Quest entries.
    pub quests: Vec<QuestInfo>,
}

/// Individual quest info.
#[derive(Debug, Clone)]
pub struct QuestInfo {
    /// Quest name.
    pub name: String,
    /// Completion state.
    pub state: QuestState,
    /// Description text.
    pub description: String,
}

/// The quest log organism.
pub struct QuestLog;

impl QuestLog {
    /// Draw the quest log as a window.
    pub fn show(ctx: &Context, is_open: &mut bool, acts: &[ActQuestData], active_act: usize) {
        if !*is_open {
            return;
        }

        egui::Window::new("Journal de quetes")
            .resizable(false)
            .collapsible(false)
            .open(is_open)
            .default_pos([200.0, 100.0])
            .min_width(300.0)
            .show(ctx, |ui| {
                // Act tabs
                ui.horizontal(|ui| {
                    for (i, act) in acts.iter().enumerate() {
                        let color = if i == active_act {
                            rgba_to_color32(&D2_PALETTE.text_high)
                        } else {
                            rgba_to_color32(&D2_PALETTE.text_secondary)
                        };
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(&act.name).color(color).size(11.0),
                            )
                            .sense(egui::Sense::click()),
                        );
                    }
                });

                ui.separator();

                // Quest entries for active act
                let act_idx = active_act.min(acts.len().saturating_sub(1));
                if let Some(act) = acts.get(act_idx) {
                    for quest in &act.quests {
                        QuestEntry::new(&quest.name, quest.state).show(ui);
                    }
                }
            });
    }
}
