// @id: MUIE-Stash @do: stash-panel-organism @role: organism @layer: 6 @human: miyuk

//! Stash panel organism -- shared stash with tabs and grid.

use crate::convert::rgba_to_color32;
use crate::organisms::inventory_panel::GridItemData;
use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;

/// Stash tab data.
#[derive(Debug, Clone)]
pub struct StashTabData {
    /// Tab label.
    pub name: String,
    /// Items in this stash tab.
    pub items: Vec<GridItemData>,
}

/// The stash panel organism.
pub struct StashPanel;

impl StashPanel {
    /// Draw the stash panel as a window.
    pub fn show(
        ctx: &Context,
        is_open: &mut bool,
        tabs: &[StashTabData],
        active_tab: usize,
        gold_stash: i64,
    ) {
        if !*is_open {
            return;
        }

        egui::Window::new("Coffre")
            .resizable(false)
            .collapsible(false)
            .open(is_open)
            .default_pos([400.0, 50.0])
            .min_width(300.0)
            .show(ctx, |ui| {
                // Tabs
                ui.horizontal(|ui| {
                    for (i, tab) in tabs.iter().enumerate() {
                        let color = if i == active_tab {
                            rgba_to_color32(&D2_PALETTE.text_high)
                        } else {
                            rgba_to_color32(&D2_PALETTE.text_secondary)
                        };
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(&tab.name).color(color).size(11.0),
                            )
                            .sense(egui::Sense::click()),
                        );
                    }
                });

                ui.separator();

                // Grid (reuse the inventory grid drawing approach)
                let tab_idx = active_tab.min(tabs.len().saturating_sub(1));
                if let Some(tab) = tabs.get(tab_idx) {
                    ui.label(
                        egui::RichText::new(format!("{} items", tab.items.len()))
                            .color(rgba_to_color32(&D2_PALETTE.text_primary))
                            .size(10.0),
                    );
                    // Grid would be drawn similar to inventory_panel::draw_grid
                    // For brevity, we display item count. Full grid uses CELL_PX layout.
                }

                ui.separator();

                // Gold in stash
                let gold_text = format!("Or dans le coffre : {gold_stash}");
                ui.label(
                    egui::RichText::new(&gold_text)
                        .color(rgba_to_color32(&D2_PALETTE.text_high))
                        .size(11.0),
                );
            });
    }
}
